# PARLOR Invariants

## PARLOR-I-01: Proven Means Exhaustive Enumeration

**Status:** VERIFIED

**Invariant:** `EvidenceLabel::Proven` is reserved for results established by
the kernel, such as retained perft enumeration checks.

**Why it matters:** Published correctness claims collapse if estimates,
rollouts, or cited tables are represented as proof.

**Test:** `cargo test --release --workspace`.

**Evidence:** `crates/parlor-core/src/lib.rs`, `crates/parlor-chess/src/lib.rs`,
`crates/parlor-checkers/src/lib.rs`, and `PRODUCT_PLAN.md`.

## PARLOR-I-02: Chess Perft Benchmarks Stay Cited And Exact

**Status:** VERIFIED

**Invariant:** Chess verification reproduces retained startpos and Kiwipete
published perft counts exactly.

**Why it matters:** Perft is PARLOR's clearest bug oracle for legal-move
generation, including castling, en passant, and promotions.

**Test:** `cargo test --release --workspace` and
`cargo test -p parlor-chess --test proof_surface`.

**Evidence:** `README.md`, `context/waves/2026-06-chess-kernel/WAVE.md`, and
`crates/parlor-chess/src/lib.rs`.

## PARLOR-I-03: Game Crates Keep Native Rule Ownership

**Status:** VERIFIED

**Invariant:** Chess, backgammon, checkers, and Go own their native positions,
legal moves, and rule properties inside their game crates.

**Why it matters:** Cross-game uniformity is useful only while it preserves the
facts that make each game verifiable.

**Test:** `cargo test --release --workspace`.

**Evidence:** `crates/parlor-chess/`, `crates/parlor-backgammon/`,
`crates/parlor-checkers/`, `crates/parlor-go/`, and
`.roles/parliament/rules-kernel-auditor.md`.

## PARLOR-I-04: Owner Validation Is Not Replaced By FERRIS

**Status:** VERIFIED

**Invariant:** The FERRIS consumer contract proves a pinned planning shape, but
PARLOR's documented test, clippy, and formatting commands remain required.

**Why it matters:** A planner can select or describe validation; it cannot
become the correctness evidence for game kernels.

**Test:** `python tools/ferris-contract/check.py`.

**Evidence:** `README.md`, `context/waves/2026-08-ferris-adoption/WAVE.md`,
and `context/waves/2026-08-ferris-adoption/REVIEW.md`.

## PARLOR-I-05: Retained Proofs Include Accepted And Rejected States

**Status:** VERIFIED

**Invariant:** Proof fixtures cover both accepted kernel behavior and structured
failure for invalid inputs.

**Why it matters:** A kernel that only records happy paths can regress its
diagnostics and still look correct under ordinary examples.

**Test:** `cargo test -p parlor-chess --test proof_surface`.

**Evidence:** `crates/parlor-chess/tests/proof_surface.rs`,
`.roles/parliament/ground-truth-custodian.md`, and `README.md`.
