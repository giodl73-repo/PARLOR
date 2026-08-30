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
| **backgammon** | implemented | rules kernel + verified pip count and dice distribution |
| **checkers** | implemented | English-draughts kernel, perft-verified (mandatory capture) |
| **go** | implemented | rules kernel (captures, suicide, ko) + area scoring |

Backgammon is the deliberate second entry: it flips the analytical core from a
deterministic, perfect-information move tree to a **stochastic** one. Its cited,
verifiable anchors are the canonical opening **pip count of 167 per player** and
the **dice distribution** (21 distinct rolls whose probabilities sum to 1 over 36
outcomes; mean roll value 49/6), with legal-move generation under a roll verified
by rule-property tests:

```
$ parlor backgammon verify
[PASS] opening pip count is 167 per player (white 167, black 167)
[PASS] the 21 dice rolls' probabilities sum to 1
[PASS] mean roll value is 49/6
```

Both games were authored end-to-end by the CRAFT dogfood loop (`run-wave
--execute` + the Copilot draft provider, gated on clippy + the cited tests).

## Workspace

```
crates/
  parlor-core        # cross-game contract: Game, Perft, EvidenceLabel, PerftBenchmark
  parlor-chess       # chess kernel: board, FEN, legal moves, perft, cited benchmarks
  parlor-backgammon  # backgammon kernel: board, pip count, dice model, legal plays
  parlor-checkers    # checkers (English draughts) kernel: legal moves, perft, cited benchmarks
  parlor-go          # go kernel: captures, suicide, ko, legal moves, area scoring
  parlor-cli         # one front door: `parlor games`, `parlor chess|backgammon|checkers|go ...`
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

Focused retained proof:

```powershell
cargo test -p parlor-chess --test proof_surface
```

The fixture records the accepted opening FEN with 20 legal moves and a
structured `FenError::Fields` rejection for an incomplete FEN.

PARLOR also owns a compatibility proof for the exact experimental FERRIS
`validation-plan` contract used in its prior public validation-selection
control:

```
python tools/ferris-contract/check.py
```

That proof builds the exact pinned FERRIS commit in a temporary directory,
invokes its `cargo-ferris` adapter with Cargo's injected-token contract from
inside the workspace, and verifies the `parlor-go` reverse cone plus the
repository-file full-workspace fallback. Direct invocation prevents ambient
Cargo aliases from substituting another command. The proof does not execute a
FERRIS plan and does not replace any validation command above. See
[`context/waves/2026-08-ferris-adoption/WAVE.md`](context/waves/2026-08-ferris-adoption/WAVE.md)
for ownership, migration, and rollback boundaries.

PARLOR also runs a non-required
[`Ferris Go shadow`](.github/workflows/ferris-go.yml). It preserves the direct
Cargo gates above while separating quality, release build, release test-build,
and package test evidence in one verifiable receipt:

```console
python tools/ferris-go/run.py --ferris <FERRIS_BINARY> --full
python tools/ferris-go/run.py --ferris <FERRIS_BINARY> \
  --changed-path crates/parlor-go/src/lib.rs
```

A `parlor-go` change selects `parlor-go` plus its reverse consumer
`parlor-cli`; a `parlor-core` or repository-level change widens to the full
workspace. See [`tools/ferris-go/README.md`](tools/ferris-go/README.md) for the
topology and removal path. Ferris remains a shadow and does not replace required
owner validation.

## Non-goals

- Not a chess *engine*: no search, evaluation, or play strength. PARLOR is a
  rules/enumeration kernel, not a competitor to Stockfish.
- No GUI or online play.
- No networked or wagering features for any game.

## License

PARLOR uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright giodl73-repo). Original
non-software content is licensed CC BY-NC 4.0 (copyright giodl73-repo);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
