# Wave: Ferris Go Shadow

Status: local proof complete; GitHub shadow pending

## Frame

PARLOR already owns three working validation commands: release-workspace tests,
workspace Clippy, and formatting. The missing shared capability is one
inspectable execution receipt that separates quality, build preparation, and
focused test execution without making Ferris the owner of Rust semantics.

The V1 topology is:

```text
format
  -> clippy
    -> release product build
      -> release test build (--no-run)
        -> one test lane per selected package
```

Cargo remains authoritative for workspace membership, compilation, test
discovery, and artifact reuse in the shared target directory. PARLOR owns the
commands, package inventory, requiredness, GitHub workflow, and receipt
retention. Ferris owns immutable plan execution and receipt integrity.

## Selection scenarios

| Change | Expected package set | Purpose |
| --- | --- | --- |
| `crates/parlor-go/src/lib.rs` | `parlor-go`, `parlor-cli` | narrow leaf plus reverse consumer |
| `crates/parlor-core/src/lib.rs` | all six packages | shared-contract fan-out |
| `README.md` | full workspace fallback | conservative repository-level change |

Formatting and Clippy remain full-workspace owner gates in every scenario.
Build and test lanes use Ferris's selected package closure.

## V1 boundaries

- The GitHub job is a visible, non-required shadow; branch policy is unchanged.
- No shell command strings, credentials, retries, provider APIs, or publication.
- Build and tests remain sequential in GO-WP-003 execution even though package
  tests are represented as independent descendants of the test-build barrier.
- The receipt is integrity-checked, not signed or provider-authenticated.
- No duration or prevented-iteration claim follows from this integration.

## Deletion target

Delete `.github/workflows/ferris-go.yml`, `tools/ferris-go/`, this wave, and the
README integration section. PARLOR's three direct Cargo commands remain
unchanged and sufficient.

## Disproof

Abandon or redesign the integration if Ferris selects outside Cargo's reverse
dependency closure, loses a required owner gate, produces a success receipt
after a failed lane, requires credentials, or prevents the direct Cargo commands
from passing after complete removal.

## Local evidence

At integration revision `86ca1fee5498bd191d9a76398e268489d17b7e68`,
the full topology produced ten successful lanes:

- format and Clippy;
- release product build;
- release test build with `--no-run`; and
- one test lane for each of the six workspace packages.

Receipt
`sha256:1bdb427c928e35ce2a2772c5019b3cd1b6c7a618975ec32f5fdd3954985aff1b`
verified successfully.

The test-build barrier completed in 294 ms after the product build. The six
package test lanes then completed independently in the model while reusing
Cargo's local target outputs. These timings describe one warm local run and are
not a performance claim.

## Feature scenario

PARLOR added a public `Board::legal_move_count` API and
`parlor go legal --size <N>` command. The feature changes `parlor-go` and its
CLI consumer without changing other game kernels.

At revision `6c1b52ccc416c0d238005a386b2f4c917953b2e5`, selecting
`crates/parlor-go/src/lib.rs` produced exactly:

- `parlor-go`;
- `parlor-cli`;
- two quality lanes;
- two build lanes; and
- two package test lanes.

All six lanes passed and receipt
`sha256:9dffc06718ad1d1e547d449ae31700cf96a3c391170161d5aea2089ef6d0916c`
verified successfully. Selecting `parlor-core` produced all six packages;
selecting `README.md` produced the named `full_workspace_fallback`.
