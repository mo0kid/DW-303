"""Regression checks for runner measurements and generated SPICE evidence.

Run after run.py: python3 -m unittest discover -s reference/spice -p 'test_*.py'
These checks detect export/control-sign/regeneration mistakes; they do not claim
that the generic device models meet the hardware accuracy budgets.
"""
import csv
import hashlib
import json
import math
from pathlib import Path
import unittest

from run import fit_tau, frequency

SPICE = Path(__file__).resolve().parent
GOLDEN = SPICE.parent / 'golden'


class MeasurementTests(unittest.TestCase):
    def test_crossings_on_irregular_grid(self):
        rows = [[i/997, math.sin(2*math.pi*13*i/997)] for i in range(1000)]
        self.assertAlmostEqual(frequency(rows, 1, 0), 13, places=3)
        self.assertIsNone(frequency([[0, 0], [1, 0]], 1, 0))

    def test_floored_decay_fit(self):
        rows = [[i*.001, 1.5+9*math.exp(-i*.001/.068)] for i in range(500)]
        self.assertAlmostEqual(fit_tau(rows, 1, 1.5, .01, .1), .068, places=10)


class GoldenTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.summary = json.loads((GOLDEN/'summary.json').read_text())

    def test_all_exports_are_finite_monotonic_and_present(self):
        summary = self.summary
        self.assertEqual(len(summary['runs']), 66)
        for run in summary['runs'].values():
            for name in run['files']:
                with self.subTest(file=name), (GOLDEN/name).open() as source:
                    reader = csv.reader(source)
                    width = len(next(reader))
                    previous = -math.inf
                    count = 0
                    for row in reader:
                        values = list(map(float, row))
                        self.assertEqual(len(values), width)
                        self.assertTrue(all(math.isfinite(x) for x in values))
                        self.assertGreater(values[0], previous)
                        previous = values[0]
                        count += 1
                    self.assertGreater(count, 100)
        with (GOLDEN/'env/converter.csv').open() as source:
            self.assertEqual(sum(1 for _ in csv.reader(source)), 31)

    def test_static_transfer_uses_voltage_scale(self):
        with (GOLDEN/'vco/transfer_data.csv').open() as source:
            rows = list(csv.reader(source))[1:]
        self.assertEqual(float(rows[0][0]), 5)
        self.assertEqual(float(rows[-1][0]), 13)
        self.assertEqual(len(rows), 401)

    def test_provenance_matches_current_device_models(self):
        for name, expected in self.summary['include_sha256'].items():
            self.assertEqual(hashlib.sha256((SPICE/name).read_bytes()).hexdigest(), expected)

    def test_slide_and_tune_calibration(self):
        self.assertAlmostEqual(self.summary['vco']['slide_tau_s'], .022, delta=2e-6)
        self.assertAlmostEqual(self.summary['vco']['a2']['frequency_hz'], 110, delta=.02)

    def test_accent_bypasses_decay_pot(self):
        env = self.summary['env']
        taus = [env[f'decay_{name}_accentTrue']['fit_tau_floor_1p5v_s']
                for name in ('min', 'mid', 'max')]
        self.assertLess(max(taus)/min(taus), 1.01)
        self.assertGreater(env['decay_max_accentFalse']['fit_tau_floor_1p5v_s'], .9)
        for res in ('0', '0.5', '1'):
            self.assertGreater(env[f'accents8_res{res}']['c13_end_last_step_v'],
                               env[f'accents1_res{res}']['c13_end_last_step_v'])

    def test_ota_control_current_sign_and_gain_scaling(self):
        vca = self.summary['vca']
        for source in ('signal', 'tap'):
            low = vca[f'i1u_{source}']['vca_gain_db']
            high = vca[f'i100u_{source}']['vca_gain_db']
            self.assertGreater(low, -100)  # catches backwards pin-1 current sign
            self.assertAlmostEqual(high-low, 40, delta=.05)
        self.assertGreater(vca['veg']['peak_control_current_a'], 40e-6)
        self.assertLess(vca['veg']['peak_control_current_a'], 55e-6)


if __name__ == '__main__':
    unittest.main()
