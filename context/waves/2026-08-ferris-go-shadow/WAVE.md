# Wave: Ferris Go Shadow

Status: implementation

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
