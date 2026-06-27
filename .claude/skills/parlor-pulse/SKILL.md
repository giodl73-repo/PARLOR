# PARLOR Pulse

Use this skill to execute one PARLOR pulse (a single unit of work inside a wave).

## Rules

1. Every pulse names its validation commands and leaves the workspace green.
2. New games depend on `parlor-core`, never on another game crate.
3. A correctness benchmark that does not match its published value is a defect,
   reported as such — never silently re-baselined.
4. Reserve `EvidenceLabel::Proven` for results established by exhaustive
   enumeration in the kernel; label evaluations and rollouts `Heuristic` /
   `Estimated`.

## Workflow

1. Open `context/waves/<date>-<slug>/pulses/pulse-NN.md` with intent, work, and
   validation commands.
2. Implement in the relevant crate; add tests, including any cited benchmark.
3. Run the validation commands; record the outcome in the pulse file.

## Validation

- `cargo test --release --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
