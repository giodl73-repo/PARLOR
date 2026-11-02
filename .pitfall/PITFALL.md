# PARLOR PITFALL index

PARLOR uses PITFALL as a compact index over game-kernel doctrine: exact cited
benchmarks, evidence labels, role-reviewed rules boundaries, and the isolated
FERRIS consumer contract. The index cites existing repo docs and tests without
moving local sources of truth.

| Namespace | Kind | Path | Owner |
|---|---|---|---|
| `parlor` | `principles` | [parlor-principles.md](parlor-principles.md) | PARLOR maintainers |
| `parlor` | `invariants` | [parlor-invariants.md](parlor-invariants.md) | PARLOR maintainers |
| `parlor` | `pitfalls` | [parlor-pitfalls.md](parlor-pitfalls.md) | PARLOR maintainers |

## Integration

- ROLES: `.roles/ROLE.md` routes benchmark, kernel, shared-contract, CLI, and
  game-addition changes to the matching reviewers.
- VTRACE: PARLOR does not yet carry repo-local VTRACE docs; PITFALL cites waves,
  role reviews, product docs, retained proof tests, and compatibility checks.
- Tests: release workspace tests, strict clippy, formatting, chess proof
  surface tests, FERRIS contract checks, and PITFALL validators are the
  executable evidence hooks.
