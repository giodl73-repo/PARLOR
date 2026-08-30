# Ferris Go Shadow Role Review

Status: local integration accepted; GitHub shadow pending

## Repository roles

| Role | Finding | Disposition |
| --- | --- | --- |
| `cross-game-contract-steward` | Cargo supplies package closure; PARLOR maps it to owner gates. The adapter adds no cross-game product abstraction. | `pass` |
| `rules-kernel-auditor` | Full release tests remain authoritative. The Go feature retains capture, suicide, ko, and scoring coverage while adding accepted legal-count cases. | `pass` |
| `ground-truth-custodian` | Receipt timings are labelled as one local observation, not a speed or savings claim. Legal-move counts are computed properties, not promoted external facts. | `pass` |
| `player-maintainer-advocate` | One command runs the shadow and reports selected packages and every terminal lane. Requiring a separately built Ferris binary remains setup friction. | `pass-with-condition` |

## Operational lenses

| Lens | Finding | Disposition |
| --- | --- | --- |
| Platform | Windows full and focused runs pass, but the pinned Ubuntu GitHub job must pass before cross-platform acceptance. | `pass-with-condition` |
| Security | The plan binds tracked files and a staged Cargo executable, uses no credentials, and verifies its receipt. The short-lived approval is not authenticated. | `pass-with-condition` |
| Removability | `.ferris/` can be deleted and direct owner gates still pass. The workflow and adapter are isolated deletion units. | `pass` |

## Blocking findings resolved

1. Direct `ferris validation-plan` requires an explicit manifest path. The
   adapter now binds PARLOR's root `Cargo.toml`.
2. Pull-request selection must come from the exact event range, not a guessed
   path. The workflow now fetches history and passes the base revision; the
   adapter derives the NUL-delimited Git path set.

## Decision

The local slice is accepted. Keep the workflow non-required until its exact
Ferris pin passes on GitHub-hosted Ubuntu. Do not remove or narrow PARLOR's
direct owner-validation job, and do not infer duration savings from this proof.
