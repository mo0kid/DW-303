# Sequencer core

`dw303-sequencer` is `no_std`, forbids unsafe through workspace lints, has no
allocator dependency, and depends only on the existing `ravet-foundation` path
crate. It models the original digital note/gate/accent/slide controls. Analog
slide lag, envelope capacitors and accent-sweep voltages belong to the circuit
crates; this core supplies `VoiceCommand` using the contract's relative semitone
index (Down C = 0, Mid C = 12, Up C = 24, upper C = 36). MIDI keyboard keys are
passed through as semitone indices; an engine wanting a particular absolute
MIDI-to-oscillator tuning must establish that reference separately.

## Data and original memory organisation

A bank contains four groups, each with A/B sections of eight patterns: 64
patterns. `PatternRef::index()` gives group × 16 + section × 8 + pattern.
Pitch-mode `Step` entries hold C..C', octave, accent and outgoing slide. The
independent time array holds Note, Tie (the original sustain time entry), or
Rest. Only Note consumes a pitch entry. This separation preserves the original
pitch/time entry concept for the later front-panel editor. Pattern length counts
time entries; the pitch cursor rewinds with the pattern. A tie inherits pitch,
accent and the outgoing slide latch of its sounding note. An all-tie pattern
has no sounding note and is silent. A rest cuts continuation history.

The original has seven tracks with 64 measure memories each. The owner's manual
also describes combining adjacent track memories for longer arrangements: the
maximum beginning at tracks 1..7 is 256, 256, 256, 256, 192, 128, 64 measures.
Here each `Track` owns 64 entries and length 0..64; combined-track memory borrowing
and the panel's track-to-pattern-group restrictions are left for the editor.
References may select any group, and tracks repeat at their stored length.
Each track entry stores a pattern reference and a signed transpose. Empty tracks
are silent. `tick_track` also sustains ties and slides across measure boundaries,
and supports mixed straight/triplet patterns using beat-based pattern durations.

The primary evidence is Roland's original [TB-303 owner's manual](https://cdn.roland.com/assets/media/pdf/TB-303_OM.pdf).
The full PDF exceeded the browser's fetch limit; the [transcription of the same
original manual](https://manuals.plus/m/2fc6b5c2ce3b581e621075a9ba2b69785eb7ddbbf05941809a477c3e2e4c917a)
was inspected, especially printed pages 52–57 and 65–66. These are original-manual
claims, rather than claims about the modern Roland software instrument.

## Triplets and timing

Straight mode subdivides each quarter into four sixteenths. The original triplet
mode is **three steps per quarter**, each one third of a beat: twelve time entries
span four quarter-note beats. It is not a fixed 3/4 time signature. A 3/4 pattern
can use twelve straight steps or nine triplet steps. The original manual lists
15 as the maximum triplet-mode step count; this storage API deliberately allows
1..16 in both modes as requested. A later authentic front-panel editor can impose
the original 15-step triplet limit.

`Timing::NOMINAL.gate_fraction` is 0.5: an ordinary note gates for the first half
of its step. A following Tie holds the gate through the current step; the final
unslid tie releases halfway through its own step. Outgoing slide holds the gate
through the source and its ties. The next note changes pitch with `slide=true`
and `trigger=false`. Its own outgoing slide determines whether its gate is held
again. Rest always gates off at its start. Accent follows the sounding pitch
entry, and changes at a new Note even when slid. Attack pulses last one sample.
The digital controls are reconstructed on host seeks; a mid-step start does not
create a new attack, but its gate reflects the current phase.

### VERIFY register

- `Timing::gate_fraction`: 0.5 is a best estimate of the original CPU-generated
  non-slide duty cycle. Neither a firmware disassembly nor a measured original
  gate waveform has established it here. It is adjustable for calibration.
- `VoiceCommand::slide` and the playback slide-latch rule: outgoing slide keeps
  the source gate high and enables destination pitch lag. Exact slide-switch
  assertion/deassertion timing around the CPU's gate/CV update, and inheritance
  through ties, need measurement against an original unit. No extra overlap
  milliseconds or anticipatory CV change have been invented.
- `TrackEntry::transpose`: signed -12..12 storage is a useful API extension.
  Original key-shift limits and encoding need checking against the panel/manual
  diagrams; do not treat this signed representation as a reconstruction of the
  original RAM or firmware format.

No analog component values are represented here, so there is no analog
`Components` struct or claimed schematic RC model in this digital stage.

## Clock and engine integration

`prepare` accepts finite sample rates 44.1–384 kHz; the default is 48 kHz.
Tempo accepts finite 0 < BPM <= 1000. Invalid numeric tempo/position inputs retain
the last valid clock setting and increment diagnostics, since zero is not a usable
clock tempo. Invalid externally supplied sample phases are zeroed and counted.
Other phases are clamped. Pattern lengths/pitches and transpose are defensively
bounded during playback, even if a control-thread caller skipped validation.

Internal and host timing use an absolute sample count relative to a beat anchor,
rather than repeated phase addition. Mathematical boundaries fall on the first
sample at or after the beat edge. A 1e-9-step roundoff tolerance snaps values
extremely close to a mathematical boundary. Continuous host blocks retain the
anchor when their positions agree within 1e-7 beats; therefore block partitioning
does not change edge samples. A tempo change reanchors the timeline without
clearing phase. Explicit discontinuities, changes of playing state, or unexpected
beat positions resynchronise history. Negative host positions support pre-roll.
Missing transport fields retain the previous setting. Host tempo ramps and loops
inside a callback require caller-side block splits and fresh transport contexts;
`TransportDetails` is not automatically scheduled by this core.

Apply events **before** rendering their `Event.frame_offset` sample, preserving
same-frame order. For host mode call `begin_host_block` before its first sample.
`Sequencer::event(EventData)` dispatches MIDI clocks and live notes. Then call
`tick(&pattern)` or `tick_track(&memory, track_index)` once per sample. External
clocks may supply `StepEvent` directly to `tick_at`. Use one playback mode
consistently; call `reset` when switching between pattern and track mode.

MIDI Start rewinds; the first following TimingClock starts step zero. Continue
retains subdivision/pulse position; Stop freezes it. Six pulses form a straight
step; eight form a triplet step. Subdivision edges occur on actual received clock
pulses, so clock arrival jitter is retained. Between pulses, the last measured
pulse interval estimates gate phase; before a second pulse it uses the configured
BPM (default 120). There is no PLL or speculative subdivision edge. Lost MIDI
clocks can freeze a gate; explicit Stop or engine disconnect handling must clear
it. Call handlers at the sample timestamp supplied by the adapter.

Stopped playback uses `MonoNoteInput`: fixed storage covers all 2,048 channel/key
identities; last note wins, and releases return to the most recent held note
without attack. Overlap enables slide. Default accent threshold is velocity >=100.
Repeated note-on replaces the same channel/key identity; zero MIDI velocity is
note-off. The raw MIDI path preserves integer velocity; foundation NoteOn uses
rounded normalised velocity. MIDI reset and CC120/123 release keys. Extended note
IDs, scoped expressions, sustain pedal, pitch bend and UMP are not implemented.
Held keyboard state is tracked while the sequencer runs; those attack pulses are
consumed, so stopping with keys held resumes them as gate state without a new
attack. A fresh stopped-mode NoteOn generates an attack normally.

## Versioned binary state

Every top-level record has four ASCII magic bytes and version byte 1. Integers are
single bytes, so byte order is irrelevant. Nested records omit their headers.

| Record | Magic | Bytes |
| --- | --- | ---: |
| Pattern | D3PT | 39 |
| Bank | D3BK | 2181 |
| Track | D3TR | 134 |
| Memory | D303 | 3084 |

Pattern payload: length byte, triplet byte (0/1), then sixteen pairs of packed
pitch and time bytes. Pitch low nibble is 0..12, bits 4..5 are octave 0..2, bit 6
accent and bit 7 slide. Time is 0=Note, 1=Tie, 2=Rest. Track payload: length byte
then 64 reference/transpose pairs. Reference is the flat 0..63 index; transpose
stores signed semitones plus 12 (0..24). Memory holds a bank followed by seven
tracks. Unused entries are saved and validated, preserving partially edited state.

`encode` writes into caller storage and returns the exact written size; larger
output buffers are allowed. On error a prefix may be overwritten. `decode` takes
exactly that prefix, rejects trailing/truncated data, wrong magic/version and
invalid fields, and returns only a fully validated record. The format has no
checksum: mutations that remain valid field values cannot be identified as
corruption. `validate` is a control-thread operation, not a per-sample operation.
No dependencies or root workspace files were intentionally changed.

## Validation and limits

Sixteen integration tests pass with `cargo test --offline -p dw303-sequencer`.
They cover exact edges at 73/120/137.5/241 BPM in both subdivisions with 1/7/64/512
frame partitions, internal/host equality, mid-step start, backward seek/loop jump,
stop and phase-preserving tempo change, MIDI six/eight-pulse timing and half-step
gate release, gate durations at 48/96/192 kHz, slide/tie/rest and pitch cursors,
track-boundary ties/slides and one-step looping, mixed triplet tracks, live MIDI
priority/legato/accent/channel identity, all 2,048 held notes, full-state codec
round trips, malformed/truncated/trailing records, and invalid controls.

`cargo clippy --offline -p dw303-sequencer --all-targets -- -D warnings` passes.
All assigned Rust files are formatted with edition-2024 rustfmt. Hot paths use
fixed arrays and bounded loops (at most 16 rhythm positions for a pattern, 64
track entries and up to 1,024 positions for track note-history reconstruction,
and 2,048 identities for keyboard updates); there is no allocation or unsafe code.
Track reconstruction favours correctness over CPU cost; benchmark it in the
engine before optimising. No allocator instrumentation, CPU deadline measurement,
hardware captures, firmware comparisons or analog golden tests were performed.
The timing VERIFY items, combined-track editing, original group restrictions,
within-block host automation and missing extended MIDI features remain limits.
