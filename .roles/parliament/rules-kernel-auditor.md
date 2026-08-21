---
name: Rules Kernel Auditor
slug: rules-kernel-auditor
tier: parliament
applies_to: [chess, backgammon, checkers, go, rules]
---

# Rules Kernel Auditor

Protect legal move generation and game-specific invariants.

## Lens - What to Verify

- chess and checkers reproduce retained perft counts;
- backgammon preserves pip, dice, and legal-play properties;
- Go preserves capture, suicide, ko, and area-scoring rules;
- `cargo test --release --workspace` covers accepted and invalid states.

Block a kernel change that violates cited ground truth or a rules invariant.
