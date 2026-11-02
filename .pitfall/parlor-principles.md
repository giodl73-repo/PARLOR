# PARLOR Principles

## PARLOR-P-01: Evidence Labels Are Claim Boundaries

**Status:** ACTIVE

**Statement:** Every quantitative claim is labelled by how it is known:
proven enumeration, cited fact, heuristic, or estimate.

**Decision rule:** New benchmarks, rollout results, tables, and CLI claims must
name their evidence class and must not upgrade heuristic or cited material into
proof.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `crates/parlor-core/src/lib.rs`,
and `.roles/parliament/ground-truth-custodian.md`.

## PARLOR-P-02: Rules Kernels Beat Game Products

**Status:** ACTIVE

**Statement:** PARLOR implements verifiable rules and enumeration kernels, not
engines, GUIs, online play, wagering, or play-strength products.

**Decision rule:** A new feature may expose verification or legal-state
behavior, but product surfaces outside the kernel scope need explicit review
and cannot borrow benchmark success as readiness.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, and
`.roles/stakeholders/player-maintainer-advocate.md`.

## PARLOR-P-03: One Game Does Not Define The Contract

**Status:** ACTIVE

**Statement:** `parlor-core` contains only concepts that survive across
deterministic, stochastic, perfect-information, and board-state games.

**Decision rule:** Shared traits and CLI uniformity must not erase native game
rules or force a game-specific abstraction into every crate.

**Evidence:** `PRODUCT_PLAN.md`, `.roles/parliament/cross-game-contract-steward.md`,
and `crates/parlor-core/src/lib.rs`.

## PARLOR-P-04: Published Ground Truth Is A Defect Gate

**Status:** ACTIVE

**Statement:** A cited benchmark mismatch is a kernel defect, not a new local
baseline.

**Decision rule:** Perft, pip-count, dice-distribution, and rule-property
failures must be fixed or explicitly bounded before any correctness claim is
published.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`.roles/parliament/rules-kernel-auditor.md`, and release tests.

## PARLOR-P-05: Compatibility Pins Do Not Transfer Ownership

**Status:** ACTIVE

**Statement:** The FERRIS compatibility proof is PARLOR-owned evidence for one
exact consumer contract; it does not make FERRIS PARLOR's validator or support
surface.

**Decision rule:** PARLOR validation commands remain mandatory, FERRIS remains
read-only and non-authoritative, and pin changes require the documented
consumer proof.

**Evidence:** `context/waves/2026-08-ferris-adoption/WAVE.md`,
`context/waves/2026-08-ferris-adoption/REVIEW.md`, and
`tools/ferris-contract/check.py`.
