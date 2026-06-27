# PARLOR — Product Plan

## Thesis

The portfolio has proven a discipline — typed Rust kernels, cited evidence, and
verifiable claims — across infrastructure and access analysis. PARLOR carries the
same discipline into a brand-new scope: **classic games modelled as cited,
verifiable kernels**, many games side by side under one workspace.

The wedge that makes the scope rigorous rather than recreational: classic games
have *exact, published ground truth*. Chess perft counts, backgammon match-equity
tables, and checkers endgame databases are authorities a kernel can be checked
against. PARLOR's value is a clean-room, evidence-labelled kernel for each game
whose correctness is machine-verifiable, not asserted.

## Architecture

- `parlor-core` (product-neutral): the cross-game contract — `Game`, `Perft`,
  `EvidenceLabel` (Proven / Cited / Heuristic / Estimated), `PerftBenchmark`,
  `PerftCheck`. New games depend on this, not on each other.
- `parlor-<game>`: one crate per game, implementing the contract. `parlor-chess`
  is first.
- `parlor-cli`: a single front door dispatching to each game.

Rule: keep `parlor-core` neutral. Game-specific notation, rules, and evaluation
live in the game crate.

## Roadmap (one game advanced at a time)

1. **chess** (done): board, FEN, legal move generation, perft, cited benchmarks
   (startpos d1–d5, Kiwipete d1–d3). CLI: `verify`, `perft`, `moves`.
2. **backgammon** (next): board, pip count, legal move generation under a dice
   roll, and a Monte-Carlo rollout equity estimate labelled `Estimated`; cite a
   published match-equity table as `Cited`.
3. **checkers/draughts**: perfect-information kernel with its own perft-style
   enumeration benchmark.
4. **go**: rules kernel (legal moves, ko, captures, territory scoring).

Cross-game analysis layers (shared, later): opening/branching-factor statistics,
state-space size estimates (labelled), and a uniform `verify` surface.

## Evidence rules

- Every published count or table cited to a named authority (`data/sources.md`
  once external tables are admitted).
- `Proven` is reserved for results this kernel establishes by exhaustive
  enumeration. Evaluations and rollouts are `Heuristic`/`Estimated`.
- A correctness benchmark that does not match its published value is a defect,
  reported as such — never silently re-baselined.

## Non-goals

- No game engines (no search/strength), GUIs, online play, or wagering.
- `parlor-core` never grows game-specific vocabulary.

## Validation contract

- `cargo test --release --workspace` (includes cited perft benchmarks).
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo fmt --all -- --check`.
