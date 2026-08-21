# PARLOR Review Panel

Use this panel for shared game contracts, rules kernels, cited benchmarks, and
cross-game CLI behavior.

## Active Roles

| Role | Protects | Invoke when |
|---|---|---|
| [Rules Kernel Auditor](parliament/rules-kernel-auditor.md) | Legal moves and game invariants | Changing any game kernel |
| [Ground Truth Custodian](parliament/ground-truth-custodian.md) | Cited quantitative claims | Adding benchmarks, verification, or evidence labels |
| [Cross-Game Contract Steward](parliament/cross-game-contract-steward.md) | Honest shared abstractions | Changing `parlor-core` or CLI uniformity |
| [Player and Maintainer Advocate](stakeholders/player-maintainer-advocate.md) | Usable diagnostics and bounded scope | Changing commands, errors, or adding a game |

## Core Tensions

| Pulls | Against | Because |
|---|---|---|
| Rules Kernel Auditor | Cross-Game Contract Steward | Different games should not be flattened to satisfy one interface. |
| Ground Truth Custodian | Player and Maintainer Advocate | Exact verification can be expensive or difficult to explain. |
| Cross-Game Contract Steward | Player and Maintainer Advocate | Uniform commands can hide game-specific concepts. |

## Review Order

1. Ground Truth Custodian establishes the claim and source.
2. Rules Kernel Auditor proves legal behavior.
3. Cross-Game Contract Steward evaluates shared surface.
4. Player and Maintainer Advocate checks commands and diagnostics.
