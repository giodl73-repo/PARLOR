# Wave 2026-06 — Chess kernel

## Goal

Stand up the PARLOR workspace with a product-neutral cross-game contract and a
first game (chess) whose move generation is verified against published perft
counts. Establish the "games side by side" structure so later games slot in
beside chess without changing the core.

## Scope

- `parlor-core`: `Game`, `Perft`, `EvidenceLabel`, `PerftBenchmark`, `PerftCheck`.
- `parlor-chess`: board, FEN parsing, fully legal move generation (castling, en
  passant, promotions, king-safety filtering), perft, perft-divide, and the
  cited benchmark table.
- `parlor-cli`: `games`, `chess verify`, `chess perft`, `chess moves`.

## Out of scope

- Any search/evaluation/play strength (PARLOR is a kernel, not an engine).
- Other games (reserved as planned crates in the roster).

## Validation

- `cargo test --release --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo run --release -p parlor-cli -- chess verify` reproduces all 7 cited
  perft benchmarks exactly.

## Pulses

- `pulse-01` — workspace, core contract, chess kernel, CLI, perft verification.

## Result

Settled. All 7 cited perft benchmarks (startpos d1–d4, Kiwipete d1–d3) reproduce
exactly; the test suite additionally verifies startpos perft(5) = 4,865,609.
fmt/clippy/test green.
