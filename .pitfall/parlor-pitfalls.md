# PARLOR Pitfalls

## PARLOR-PF-01: Benchmark Pass Becomes Engine Strength

**Status:** MITIGATED

**Pattern:** Exact legal-move or perft success is presented as evidence of
search quality, evaluation strength, or player-ready product behavior.

**Domain:** README claims, CLI output, release notes, demos, and customer-facing
game descriptions.

**Detection difficulty:** Correct move generation is impressive and easy to
overread as a complete game or engine.

**Structural solution:** Keep non-goals explicit and route product-surface
changes through the Player and Maintainer Advocate.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, and
`.roles/stakeholders/player-maintainer-advocate.md`.

## PARLOR-PF-02: Evidence Labels Drift Upward

**Status:** MITIGATED

**Pattern:** A cited table, rollout, estimate, or property test is described as
`Proven` because the implementation passes locally.

**Domain:** `EvidenceLabel`, benchmarks, verification output, docs, and future
cross-game analysis layers.

**Detection difficulty:** The wrong label can look harmless until downstream
users rely on the claim as exact.

**Structural solution:** Preserve `EvidenceLabel` distinctions and require the
Ground Truth Custodian on claim or benchmark changes.

**Evidence:** `crates/parlor-core/src/lib.rs`, `PRODUCT_PLAN.md`, and
`.roles/parliament/ground-truth-custodian.md`.

## PARLOR-PF-03: Cross-Game Contract Flattens A Game

**Status:** MITIGATED

**Pattern:** `parlor-core` or CLI uniformity forces stochastic, territory,
capture, or mandatory-jump behavior into a chess-shaped abstraction.

**Domain:** `parlor-core`, per-game crates, CLI commands, and new game
adoption.

**Detection difficulty:** A uniform command can hide a missing native rule until
a later game exposes the abstraction leak.

**Structural solution:** Keep game rules in game crates and review shared
abstractions through the Cross-Game Contract Steward.

**Evidence:** `.roles/parliament/cross-game-contract-steward.md`,
`PRODUCT_PLAN.md`, and workspace crate layout.

## PARLOR-PF-04: FERRIS Pin Becomes Validation Authority

**Status:** MITIGATED

**Pattern:** The exact FERRIS `validation-plan` compatibility proof is treated
as a replacement for PARLOR's release tests or as a general FERRIS support
claim.

**Domain:** `tools/ferris-contract/`, README validation guidance, CI, and
dependency-adoption discussions.

**Detection difficulty:** A passing planner proof can appear to bless a
validation path even though it executes no selected plan.

**Structural solution:** Preserve PARLOR owner validation commands and keep
FERRIS pinned, read-only, non-authoritative, and reproducible from a clean
temporary checkout.

**Evidence:** `context/waves/2026-08-ferris-adoption/WAVE.md`,
`context/waves/2026-08-ferris-adoption/REVIEW.md`,
`tools/ferris-contract/check.py`, and the PITFALL adoption fix that enables
`core.longpaths` for the temporary FERRIS checkout.

## PARLOR-PF-05: Roadmap Lags Implemented Roster

**Status:** MITIGATED

**Pattern:** Product planning prose still describes a game as future work after
the README, crates, and CLI list it as implemented.

**Domain:** `PRODUCT_PLAN.md`, README roster, CLI `games`, wave closeouts, and
customer-facing status summaries.

**Detection difficulty:** The code can be correct while planning docs quietly
mislead maintainers about what needs proof next.

**Structural solution:** Keep roadmap status synchronized with the implemented
roster and leave future evidence work explicitly labelled.

**Evidence:** PITFALL adoption updated `PRODUCT_PLAN.md` to match implemented
backgammon, checkers, and Go crates while retaining future equity/table
evidence labels.
