# PARLOR Wave

Use this skill to plan or close a PARLOR wave.

## Operating rule

A wave advances the parlor by exactly one game or one shared capability. Keep
`parlor-core` product-neutral; game-specific rules and notation live in the game
crate.

## Workflow

1. Read `README.md`, `PRODUCT_PLAN.md`, and `context/waves/PHASES.md`.
2. Open `context/waves/<date>-<slug>/WAVE.md` with goal, scope, out-of-scope,
   validation commands, and a pulse list.
3. Drive pulses one at a time to a green validation pass.
4. Close the wave only when every cited benchmark passes and fmt/clippy/test are
   green; record the result in `WAVE.md` and update `PHASES.md`.

## Validation

- `cargo test --release --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
