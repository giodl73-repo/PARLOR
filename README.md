# PARLOR

Classic games as **cited, verifiable Rust kernels** — a parlor where chess,
backgammon, checkers, and Go sit side by side under one workspace.

Every game ships as its own crate implementing a shared contract
(`parlor-core`), so the CLI and analysis layers treat them uniformly. The
portfolio rule carries over from its sibling repos: **every quantitative claim is
labelled by how it is known.** A *proven* enumeration is not a *heuristic*
evaluation, and the kernel is pinned to published ground truth.

## Why this is rigorous, not a toy

Chess has an unimpeachable correctness benchmark: **perft** (the exact leaf-node
count of the legal-move tree to depth *N*). Published perft values are exact, so
a move generator either reproduces them or it has a bug. PARLOR's chess kernel
reproduces the canonical counts exactly, including the "Kiwipete" position that
exercises castling, en passant, and promotions:

```
$ parlor chess verify
[PASS] startpos-d1   depth 1 | expected        20 | observed        20
[PASS] startpos-d2   depth 2 | expected       400 | observed       400
[PASS] startpos-d3   depth 3 | expected      8902 | observed      8902
[PASS] startpos-d4   depth 4 | expected    197281 | observed    197281
[PASS] kiwipete-d1   depth 1 | expected        48 | observed        48
[PASS] kiwipete-d2   depth 2 | expected      2039 | observed      2039
[PASS] kiwipete-d3   depth 3 | expected     97862 | observed     97862
```

Source: chessprogramming wiki, ["Perft Results"](https://www.chessprogramming.org/Perft_Results).
The in-tree test suite additionally verifies startpos perft(5) = 4,865,609.

## The parlor roster

| Game | Status | Core |
|---|---|---|
| **chess** | implemented | perft-verified legal move generation |
| backgammon | planned | stochastic equity / pip / doubling-cube kernel |
| checkers | planned | perft-verifiable perfect-information kernel |
| go | planned | rules / territory kernel |

Backgammon is the deliberate second entry: it flips the analytical core from a
deterministic, perfect-information move tree to a **stochastic** one (pip counts,
roll equities, match-equity tables, the doubling cube), proving the series spans
both kinds of game.

## Workspace

```
crates/
  parlor-core    # cross-game contract: Game, Perft, EvidenceLabel, PerftBenchmark
  parlor-chess   # chess kernel: board, FEN, legal moves, perft, cited benchmarks
  parlor-cli     # one front door: `parlor games`, `parlor chess ...`
```

## Usage

```
parlor games                                   # list the roster
parlor chess verify                            # check the kernel against cited perft
parlor chess perft [--fen "<FEN>"] [--depth N] [--divide]
parlor chess moves [--fen "<FEN>"]
```

## Validation

```
cargo test --release --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Non-goals

- Not a chess *engine*: no search, evaluation, or play strength. PARLOR is a
  rules/enumeration kernel, not a competitor to Stockfish.
- No GUI or online play.
- No networked or wagering features for any game.

## License

MIT.
