# PARLOR — Wave Phases

Waves group work into reviewable tranches. Each wave advances the parlor by one
game or one shared capability, and every pulse names its validation commands.

| Wave | Slug | Scope | Status |
|---|---|---|---|
| 1 | `2026-06-chess-kernel` | Workspace + `parlor-core` contract + chess kernel verified against cited perft | active |
| 2 | `backgammon-kernel` | Stochastic kernel: pip count, dice-constrained moves, rollout equity, cited match-equity table | planned |
| 3 | `checkers-kernel` | Perfect-information kernel with enumeration benchmark | planned |
| 4 | `cross-game-analysis` | Shared branching-factor / state-space analysis surface across games | planned |

## Operating rule

Advance one game (or one shared capability) at a time to a green validation pass
before opening the next wave. Keep `parlor-core` product-neutral.
