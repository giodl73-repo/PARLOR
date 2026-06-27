# Pulse 01 — Workspace, core contract, chess kernel

## Intent

Create the PARLOR workspace and prove the scope with a perft-verified chess
kernel.

## Work

- Workspace `Cargo.toml` with `parlor-core`, `parlor-chess`, `parlor-cli`.
- `parlor-core`: cross-game `Game`/`Perft` traits, `EvidenceLabel`,
  `PerftBenchmark`/`PerftCheck`.
- `parlor-chess`: 8x8 mailbox board, FEN parser, pseudo-legal generation with a
  king-safety filter (copy-make), castling/en-passant/promotion handling, perft
  and perft-divide, and the cited benchmark table (chessprogramming wiki).
- `parlor-cli`: `parlor games`, `parlor chess verify|perft|moves`.

## Validation commands

- `cargo test --release --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo run --release -p parlor-cli -- chess verify`

## Outcome

All 7 cited perft benchmarks reproduced exactly; startpos perft(5) = 4,865,609
verified in-tree. fmt/clippy/test green. Wave 1 settled.
