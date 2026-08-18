# Wave: FERRIS Consumer Contract

Status: Complete after Pulse 01; merge requires the consumer CI proof

## Product outcome

Give PARLOR one consumer-owned compatibility proof for the exact FERRIS
`validation-plan` contract that previously measured PARLOR's leaf-versus-core
validation boundary, without replacing PARLOR's documented full validation
commands.

## Frame

PARLOR already owns a clear validation contract:

```console
cargo test --release --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

FERRIS already owns a read-only package-closure planner. The missing shared
capability is a consumer-side pin proving that one exact FERRIS release still
maps a `parlor-go` Rust edit to `parlor-go` plus `parlor-cli`, and maps a
repository-level edit to the full-workspace fallback.

PARLOR retains all required validation, correctness, release, and workflow
semantics. FERRIS retains command, schema, discovery, and planning semantics.
The compatibility proof observes both and grants neither owner new authority.

The current workaround is an unprotected research result recorded only in the
FERRIS repository. A later FERRIS change could drift from PARLOR's measured
shape without any consumer-owned signal.

The deletion target is manual reconstruction of the PARLOR control. The thesis
is disproved if the proof requires PARLOR source changes, copies FERRIS schemas,
executes a selected plan, replaces owner validation, or cannot be rolled back
by removing the isolated compatibility surface.

## Product Value Governor

Disposition: `continue-within-budget`

Approved budget:

- exactly one consumer pulse and one implementation attempt;
- one exact public FERRIS commit pin;
- one machine-readable consumer contract;
- one compatibility checker and one consumer CI workflow;
- one all-eleven-role closeout; and
- no PARLOR product-source, manifest, lockfile, or validation-command change.

Completion condition:

- build exact FERRIS commit
  `5cd1aa99727a23de25c79d067090e7444bdfb5e8`;
- invoke the exact built `cargo-ferris` adapter with Cargo's injected `ferris`
  token from a nested PARLOR directory;
- prove the accepted `parlor-go` reverse cone and repository-file fallback;
- bind `ferris.command-result/v2`, `ferris.validation-plan/v0`, command version
  `0.1.0`, and portable workspace ID `org.giodl73/parlor`;
- document migration and rollback; and
- preserve PARLOR's owner validation as mandatory and separate.

Abandonment condition:

Stop `stop-value-exhausted` without a successor if the exact pin does not
reproduce the measured shape, if the proof needs a copied FERRIS schema or
PARLOR source change, or if consumer CI cannot run it without hidden state.

## Compare

| Analogue | Classification | Use |
|---|---|---|
| PARLOR README validation commands | reuse | remain the only required owner validation |
| FERRIS PERF-Q35 PARLOR control | adapt | move the stable behavioral projection into the consumer repository |
| Full FERRIS JSON snapshot | avoid | owner output digests and result identities are evidence, not a portable consumer API |
| Copied FERRIS schema | avoid | would create two owners |
| Selected-plan execution | avoid | FERRIS planning remains non-executable |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Exact FERRIS compatibility proof | Complete | Consumer-owned pin, accepted result, fallback, migration, and rollback |

## Non-goals

- replacing, deleting, or narrowing PARLOR validation;
- running Cargo checks or tests selected by FERRIS;
- changing game kernels, CLI behavior, manifests, dependencies, or lockfiles;
- claiming FERRIS support, correctness, performance, or stable general API;
- publishing a crate or copying a FERRIS schema; and
- automatically advancing the FERRIS pin.

## Migration

To evaluate a later FERRIS release, change the exact `ferris.commit` and any
intentionally adopted command/schema versions in
`tools/ferris-contract/contract.json`, then run the compatibility checker
against that exact checkout:

```console
python tools/ferris-contract/check.py --ferris-source <EXACT_FERRIS_CHECKOUT>
```

The change is acceptable only when both the `parlor-go` accepted result and
the repository-file fallback remain explicit and PARLOR's three documented
owner validation commands remain present. A passing FERRIS proof does not
remove the requirement to run those commands.

## Rollback and removal

Rollback restores the previous `contract.json` pin and checker behavior, then
reruns the proof against the prior exact FERRIS checkout. Complete removal
deletes `.github/workflows/ferris-contract.yml`,
`tools/ferris-contract/`, this wave, and the README compatibility paragraph.
Neither operation changes a PARLOR manifest, lockfile, crate source, game
contract, CLI behavior, or owner validation command.

## Closeout

The implementation and all-eleven-role disposition are recorded in
[`pulses/pulse-01.md`](pulses/pulse-01.md) and
[`REVIEW.md`](REVIEW.md). The branch may merge only after its own
`FERRIS consumer contract` workflow proves the fetched exact pin on the
consumer CI host.
