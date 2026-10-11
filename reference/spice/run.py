#!/usr/bin/env python3
"""Run DW-303 ngspice decks and regenerate SI-unit CSV reference data.

Python standard library only. Run from any directory; --ngspice overrides the
executable. Temporary decks/logs are isolated, and output is confined to the
assigned reference directories. Failed simulations never become golden data.
"""
from __future__ import annotations

import argparse
import bisect
import csv
import hashlib
import json
import math
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

SPICE = Path(__file__).resolve().parent
GOLDEN = SPICE.parent / "golden"


def interpolate(rows, t, column):
    """Linearly interpolate an adaptive-step real-valued SPICE trace."""
    times = [r[0] for r in rows]
    i = max(1, min(len(rows) - 1, bisect.bisect_left(times, t)))
    a, b = rows[i - 1], rows[i]
    return a[column] + (b[column] - a[column]) * (t - a[0]) / (b[0] - a[0])


def frequency(rows, column, threshold, start=0.1):
    """Measure rising threshold crossings; return null for fewer than 3."""
    crossings = []
    for a, b in zip(rows, rows[1:]):
        if a[0] >= start and a[column] < threshold <= b[column]:
            crossings.append(a[0] + (b[0] - a[0]) *
                             (threshold - a[column]) / (b[column] - a[column]))
    if len(crossings) < 3:
        return None
    return (len(crossings) - 1) / (crossings[-1] - crossings[0])


def rms(rows, column, start, end):
    """Time-weighted RMS, avoiding bias from adaptive timesteps."""
    integral = duration = 0.0
    for a, b in zip(rows, rows[1:]):
        if start <= a[0] and b[0] <= end:
            dt = b[0] - a[0]
            integral += dt * (a[column] ** 2 + b[column] ** 2) / 2
            duration += dt
    return math.sqrt(integral / duration) if duration else 0.0


def fit_tau(rows, column, asymptote, start, end):
    """Least-squares log fit with an explicitly stated assumed asymptote."""
    points = [(r[0], math.log(abs(r[column] - asymptote))) for r in rows
              if start <= r[0] <= end and abs(r[column] - asymptote) > 1e-8]
    if len(points) < 2:
        return None
    xmean = sum(x for x, _ in points) / len(points)
    ymean = sum(y for _, y in points) / len(points)
    slope = sum((x-xmean)*(y-ymean) for x, y in points) / sum(
        (x-xmean)**2 for x, _ in points)
    return -1 / slope if slope < 0 else None


class Runner:
    """Isolated batch execution with checked vector export and provenance."""
    def __init__(self, executable):
        self.executable = executable
        self.records = {}

    def run(self, block, name, params=None, edits=(), save=True):
        deck = (SPICE / f"{block}.cir").read_text()
        for key, value in (params or {}).items():
            deck, count = re.subn(rf"\b{key}=[^\s]+", f"{key}={value}", deck, count=1)
            if count != 1:
                raise ValueError(f"Unknown deck parameter {key}")
        for old, new in edits:
            if old not in deck:
                raise ValueError(f"Missing edit target {old}")
            deck = deck.replace(old, new)
        with tempfile.TemporaryDirectory(prefix="dw303-spice-") as directory:
            tmp = Path(directory)
            for include in SPICE.glob("*.inc"):
                shutil.copyfile(include, tmp / include.name)
            (tmp / "deck.cir").write_text(deck)
            result = subprocess.run([self.executable, "-b", "deck.cir"], cwd=tmp,
                                    capture_output=True, text=True, timeout=120)
            log = result.stdout + result.stderr
            if result.returncode or re.search(r"(?im)^\s*(error|fatal)\b|timestep too small", log):
                raise RuntimeError(f"{block}/{name} failed:\n{log[-5000:]}")
            tables = {}
            for kind in ("data", "ac"):
                path = tmp / f"{kind}.txt"
                if not path.exists():
                    continue
                with path.open() as source:
                    headers = source.readline().split()
                    rows = [list(map(float, line.split())) for line in source if line.strip()]
                if len(rows) < 2 or any(not math.isfinite(v) for row in rows for v in row):
                    raise RuntimeError(f"Invalid data: {block}/{name}/{kind}")
                if any(len(row) != len(headers) for row in rows):
                    raise RuntimeError(f"Column mismatch: {block}/{name}/{kind}")
                tables[kind] = rows
                if save:
                    out = GOLDEN / ("vco" if block == "square-transfer" else block)
                    out.mkdir(parents=True, exist_ok=True)
                    (out / f"{name}.log").write_text(log)
                    # Preserve AC grid; transient resampled at 20us (VCO/VCF),
                    # 250us (envelopes). Data retains physical node voltages.
                    if kind == "data" and headers[0] == "time":
                        step = 20e-6 if block in ("vco", "vcf") else 250e-6
                        grid = []
                        index = 1
                        for k in range(int(rows[-1][0] / step) + 1):
                            t = k * step
                            while index < len(rows)-1 and rows[index][0] < t:
                                index += 1
                            a, b = rows[index-1], rows[index]
                            fraction = (t-a[0])/(b[0]-a[0])
                            grid.append([t] + [a[j]+fraction*(b[j]-a[j])
                                               for j in range(1, len(headers))])
                        exported = grid
                    else:
                        exported = rows
                    with (out / f"{name}_{kind}.csv").open("w", newline="") as dest:
                        writer = csv.writer(dest)
                        writer.writerow(headers)
                        writer.writerows([format(v, ".12g") for v in row] for row in exported)
            if "data" not in tables and "ac" not in tables:
                raise RuntimeError(f"No exported vectors: {block}/{name}")
            if save:
                self.records[f"{block}/{name}"] = {
                    "parameters": params or {},
                    "deck_sha256": hashlib.sha256(deck.encode()).hexdigest(),
                    "files": [f"{out.name}/{name}_{kind}.csv" for kind in tables],
                }
            return tables


def main():
    """Run calibration, waveform, AC, transient and control sweep matrices."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ngspice", default="/opt/homebrew/bin/ngspice")
    args = parser.parse_args()
    runner = Runner(args.ngspice)
    summary = {"schema_version": 1, "units": "SI; dB amplitude; phase radians",
               "temperature_c": 27, "reference_status": "generic-model simulation, not hardware truth",
               "ngspice_version": subprocess.check_output([args.ngspice, "--version"], text=True).splitlines()[1:4],
               "vco": {}, "vcf": {}, "env": {}, "vca": {}}
    # Tune exactly as page 7: A=2.75V -> 110Hz; do not tune device waveform shape.
    lo, hi = 0.0, 1.0
    for _ in range(13):
        trim = (lo + hi) / 2
        tables = runner.run("vco", "calibration", {"TM4": trim}, save=False)
        measured = frequency(tables["data"], 2, 9)
        if measured is None:
            raise RuntimeError("VCO does not oscillate during calibration")
        if measured < 110:
            lo = trim
        else:
            hi = trim
    trim = (lo + hi) / 2
    summary["vco"]["tm4_fraction"] = trim
    for cv, name, expected in [(1, "c1", 110*2**-1.75), (2, "c2", 110*2**-.75),
                               (2.75, "a2", 110), (4, "c4", 110*2**1.25)]:
        rows = runner.run("vco", name, {"CV": cv, "CVNEXT": cv, "TM4": trim})["data"]
        f = frequency(rows, 2, 9)
        last = [r for r in rows if r[0] > .15]
        summary["vco"][name] = {"cv_v": cv, "frequency_hz": f,
                                "target_hz": expected, "pitch_error_cents": 1200*math.log2(f/expected),
                                "saw_min_v": min(r[3] for r in last), "saw_max_v": max(r[3] for r in last),
                                "square_min_v": min(r[4] for r in last), "square_max_v": max(r[4] for r in last)}
    rows = runner.run("vco", "slide_c2_c3", {"CV": 2, "CVNEXT": 3, "TM4": trim})["data"]
    summary["vco"]["slide_tau_s"] = fit_tau(rows, 1, 3, .105, .16)
    # Square static transfer: this supplements waveforms; capacitor edges matter.
    # Static Q8 transfer retains its native DC voltage sweep grid.
    runner.run("square-transfer", "transfer")
    for current in (2e-6, 10e-6, 40e-6, 160e-6):
        for res in (0, .5, 1):
            name = f"i{current*1e6:g}u_res{res:g}"
            tables = runner.run("vcf", name, {"CURRENT": current, "RES": res})
            ac = tables["ac"]
            peak = max(ac, key=lambda r: r[1])
            rows = tables["data"]
            early, late = rms(rows, 2, .04, .1), rms(rows, 2, .45, .6)
            sustained = late > 1e-4 and late > .5*early
            summary["vcf"][name] = {"current_a": current, "resonance": res,
                "peak_hz": peak[0], "peak_db": peak[1], "phase_at_peak_rad": peak[2],
                "late_output_rms_v": late, "sustained_oscillation": sustained,
                "oscillation_hz": frequency(rows, 2, 0, .45) if sustained else None,
                "hand_pole_33nf_hz": current/(2*math.pi*4*.025865*33e-9)}
    for decay, name in [(0, "min"), (.5, "mid"), (1, "max")]:
        for accent in (0, 12):
            label = f"decay_{name}_accent{bool(accent)}"
            rows = runner.run("env", label, {"DECAY": decay, "ACC": accent})["data"]
            nominal = .068 if accent else .068+((100**decay-1)/99)
            peak = max(rows, key=lambda r: r[2])
            summary["env"][label] = {"peak_meg_v": peak[2], "peak_env_v": max(r[3] for r in rows),
                "nominal_rc_tau_s": nominal,
                "fit_tau_floor_1p5v_s": fit_tau(rows, 2, 1.5, peak[0]+nominal*.2, peak[0]+nominal),
                "pulse_above_6v_s": sum(b[0]-a[0] for a, b in zip(rows, rows[1:]) if a[1]>6),
                "meg_at_4s_v": rows[-1][2], "env_at_4s_v": rows[-1][3]}
    # Finite 1..8 note trains; 120 BPM sixteenths=125ms, gate=62.5ms.
    for res in (0, .5, 1):
        for count in range(1, 9):
            gates = ["0 12"]
            acts = ["0 0"]
            for note in range(count):
                t = .01+.125*note
                gates += [f"{t:g} 12", f"{t+1e-6:g} 0", f"{t+.0625:g} 0", f"{t+.062501:g} 12"]
                acts += [f"{t:g} 0", f"{t+1e-6:g} 12", f"{t+.125-2e-6:g} 12", f"{t+.125-1e-6:g} 0"]
            # Accent latched through whole step, independent of gate length.
            label = f"accents{count}_res{res:g}"
            tables = runner.run("env", label, {"RES": res}, edits=[
                ("Vngate ngate 0 PWL(0 12 .01 12 .010001 0 .0725 0 .072501 12)",
                 "Vngate ngate 0 PWL("+" ".join(gates)+")"),
                ("Vact act 0 {ACC}", "Vact act 0 PWL("+" ".join(acts)+")"),
                ("tran .25m 4 0 100u", "tran .25m 1.5 0 100u")])
            rows = tables["data"]
            end = .01+.125*(count-1)
            summary["env"][label] = {"c13_before_last_note_v": interpolate(rows, end, 6),
                "c13_end_last_step_v": interpolate(rows, end+.1249, 6),
                "sweep_peak_last_step_v": max(r[5] for r in rows if end <= r[0] <= end+.125)}
    # Static converter with explicit physical envelope level: 0.3V rest / 9.9V peak.
    converter_rows = []
    for env in (.3, 9.9):
        for cut in (0, .25, .5, .75, 1):
            for mod in (0, .5, 1):
                tables = runner.run("env", f"converter_e{env:g}_c{cut:g}_m{mod:g}",
                    {"CUT": cut, "MOD": mod, "RES": 1}, edits=[
                        ("Xcontrol env acc current", "Vfixed fixed 0 "+str(env)+"\nXcontrol fixed acc current"),
                        ("tran .25m 4 0 100u", "tran 1m .01 0 1m")], save=False)
                converter_rows.append([cut, mod, env, tables["data"][-1][7], tables["data"][-1][8]])
    with (GOLDEN / "env" / "converter.csv").open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["cutoff_rotation", "env_mod_rotation", "env_v", "ladder_current_a", "vq9_v"])
        writer.writerows(converter_rows)
    summary["env"]["converter_current_range_a"] = [min(r[3] for r in converter_rows), max(r[3] for r in converter_rows)]
    # Page-7 centre CUTOFF/min ENV MOD/ACC, RES max: locate ringing peak,
    # then report whether TM3 can attain 400..666.7Hz without changing device models.
    tm3_rows = []
    for tm3 in (0, 117500, 235000, 352500, 470000):
        control = runner.run("env", "tm3_control", {"CUT": .5, "MOD": 0, "RES": 1, "TM3": tm3},
            edits=[("tran .25m 4 0 100u", "tran 1m .01 0 1m")], save=False)
        current = control["data"][0][7]
        tables = runner.run("vcf", f"tm3_{tm3:g}", {"CURRENT": current, "RES": 1})
        peak = max(tables["ac"], key=lambda r:r[1])
        tm3_rows.append({"tm3_ohm": tm3, "current_a": current, "ac_peak_hz": peak[0],
                         "target_period_s": .002, "within_page7_band": 400<=peak[0]<=666.6667,
                         "transient_ring_frequency_hz": frequency([r for r in tables["data"] if r[0] <= .1], 2, 0, .011)})
    summary["vcf"]["tm3_page7"] = tm3_rows
    summary["vcf"]["observed_self_oscillation_threshold_res"] = None
    for current in (1e-6, 10e-6, 47e-6, 100e-6):
        for source in ("signal", "tap", "mix"):
            params = {"IABC": current, "SIGNAL": int(source=="signal"),
                      "TAP": int(source=="tap"), "MIX": int(source=="mix")}
            tables = runner.run("vca", f"i{current*1e6:g}u_{source}", params)
            ac = min(tables["ac"], key=lambda r:abs(r[0]-1000))
            summary["vca"][f"i{current*1e6:g}u_{source}"] = {"at_hz": ac[0], "vca_gain_db": ac[1],
                "line_gain_db": ac[2], "line_phase_rad": ac[3],
                "input_attenuation": ac[4], "mixer_gain_from_vca": (ac[5]/(10**(ac[1]/20))) if source != "mix" else None,
                "ota_voltage_gain": (10**(ac[1]/20)/ac[4]) if source != "mix" else None,
                "expected_ota_gm_s": current/.05173}
    rows = runner.run("vca", "veg", {"IABC": 0, "SIGNAL": 1})["data"]
    summary["vca"]["veg"] = {"peak_control_current_a": max(r[2] for r in rows),
        "minimum_c42_v": min(r[1] for r in rows),
        "fit_recharge_tau_s": fit_tau(rows, 1, 12, .1, 2),
        "control_current_at_3p1s_a": interpolate(rows, 3.1, 2)}
    summary["runs"] = runner.records
    summary["include_sha256"] = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in SPICE.glob("*.inc")}
    GOLDEN.mkdir(parents=True, exist_ok=True)
    (GOLDEN / "summary.json").write_text(json.dumps(summary, indent=2, allow_nan=False)+"\n")
    print(json.dumps({k:v for k,v in summary.items() if k not in ("runs",)}, indent=2))


if __name__ == "__main__":
    main()
