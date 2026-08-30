# PARLOR Ferris Go adapter

PARLOR owns this adapter and its CI topology. Ferris executes the resulting
immutable Action Plan but does not define PARLOR's required gates.

## Full shadow

```console
python tools/ferris-go/run.py --ferris <FERRIS_BINARY> --full
```

The full topology is:

```text
format
  -> clippy
    -> release product build
      -> release test build (--no-run)
        -> one release test lane per workspace package
```

The explicit test-build barrier lets Cargo compile test artifacts once in the
shared target directory before the package test lanes execute them.

## Changed-path shadow

```console
python tools/ferris-go/run.py \
  --ferris <FERRIS_BINARY> \
  --changed-path crates/parlor-go/src/lib.rs
```

Ferris `validation-plan` supplies Cargo's reverse-dependency closure. PARLOR
then maps that package set into its owner-defined build and test lanes.
Formatting and Clippy remain full-workspace gates. Unknown or repository-level
paths widen to all six packages.

## Evidence and removal

Generated plans, approvals, staged tools, and receipts live under `.ferris/`
and are ignored by Git. The approval is short-lived and local; it is not an
authenticated identity assertion.

Delete `.ferris/` to remove every generated runtime artifact. Delete this
directory and `.github/workflows/ferris-go.yml` to remove the integration.
PARLOR's direct Cargo validation remains unchanged.
