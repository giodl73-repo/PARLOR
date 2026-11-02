# Four-kernel Rust/WASM rules workbench

Canonical entry66ddd112827aeaff525daa0fe259f74222951bc6 and public entry
770ee14659cc82bda9ce70b1ddb47a9cc0cb6df7 diverged. The isolated feature branch
merged public master at aea809441e907a4f9de764ae2466c4305a13496b, retaining
both PITFALL doctrine and all newer metadata/verify/Go/FERRIS-shadow work.
Native crates match public entry; no kernel or shared contract changes planned.
Prior PITFALL state is preserved at pitfall/game-kernel-proof-20261102.

## Reader task

Inspect own chess FEN/legal move sequences, checkers legal path sequences,
backgammon dice/native successor selections and explicit-color Go placements.
Show native legal continuations, bounded native perft, pip/dice model, captures,
area scores and native errors. Immediately usable examples for all four games.
This is a kernel diagnostic workbench, not an engine/strength/online/wagering
product or replacement of PARLOR's retained verification/CLI/FERRIS controls.

Native private board representation remains private. Export exact request and
report, not an invented cross-game state schema or normalized FEN. Render native
records/debug text and data, not JavaScript game rules. Game metadata comes from
the current native Game contract. Proven native enumeration is separate from
cited expected benchmarks and observed/analytic/model-specific quantities.
Matching an arbitrary input or retained fixture is not complete rules validation.

## Safety and honesty boundaries

An unpublished standalone companion workspace depends on unchanged native crates.
All kernels run in a bounded worker. Browser depth0..3 and a100,000-visited-node
preflight gate bound expensive chess/checkers perft; the reported count comes
from native perft, not the admission traversal. A timeout terminates the worker.
Only moves found in the current native legal list can reach unchecked make APIs.
Chess FEN acceptance is native syntax, not full legal-position certification;
ignored clocks/additional fields are not reconstructed. Go uses explicit supplied
colors as native play accepts them, without a new turn-enforcement rule; simple
ko/area scoring is not superko/komi/adjudication. Backgammon successor indices
belong to the current native enumeration, not stable future IDs or best moves.

Requests/strings/sequence/board/dice/depth/report/memory have explicit browser
admission distinct from native errors. Input changes invalidate pending results;
original UTF-8 bytes, current editor text, explicit LF copy and exact exports
remain separate. Lifecycle/read/worker races and actual deadlines need evidence.

## Gates and ownership

Keep original crates/root Cargo.toml, all source benchmarks/tests, evidence
labels, doctrine/roles/product plan and both retained FERRIS integrations intact.
Add a committed root lock (previously ignored) for deterministic software builds,
with a separate companion lock. Native baseline initially stopped before compile
because no lock existed under --locked; offline generation enables locked checks.
Release tests/clippy/format and exact pinned FERRIS contract remain mandatory.
FERRIS shadow stays non-authoritative and is not replaced by the browser.

Require native/adapter host/WASM tests/lint; repeated exact native/WASM controls
and independent original CLI outputs; valid/invalid own inputs and retained
benchmarks with honest labels; keyboard/mobile and actual resource/recovery
tests; complete exact corresponding software and original/dependency/Rust
notices with budgets/provenance/hash inventory; outside-Git/path-with-spaces
source build and full control/behavior replay; exact-head PR/default tested
artifact deploy and fresh live proof. Child changes pushed first, separate
TRACKER snapshot only after release qualification. Other owner sessions stay intact.

Review order: Ground Truth Custodian, Rules Kernel Auditor, Cross-Game Contract
Steward, Player/Maintainer Advocate. AI-simulated software lenses do not constitute
human rules-authority approval. Status: native baseline passed; implementation
and browser/source/legal/deployment gates remain open.
