# Pulse 01: Exact FERRIS Compatibility Proof

Status: Complete
Implementation authority: Bounded to this pulse
Successor authority: None

## Implemented slice

- exact public FERRIS commit
  `5cd1aa99727a23de25c79d067090e7444bdfb5e8`;
- consumer contract `parlor.ferris-consumer-contract/v1`;
- exact FERRIS command/result versions without a copied owner schema;
- isolated temporary fetch, checkout, and build;
- direct exact `cargo-ferris` invocation with Cargo's injected `ferris` token,
  immune to ambient Cargo aliases;
- accepted `parlor-go` anchor plus `parlor-cli` reverse dependency;
- repository-file full-workspace fallback over all six packages;
- preservation checks for PARLOR's three documented owner validation commands;
- consumer CI at the immutable event commit; and
- explicit migration, rollback, removal, and non-goals.

## Measured result

Both exact-pin modes passed:

```console
python tools/ferris-contract/check.py --ferris-source C:\src\FERRIS
python tools/ferris-contract/check.py
```

The compatibility proof also passed with an injected
`CARGO_ALIAS_FERRIS=metadata`, proving ambient Cargo aliases cannot substitute
another command. PARLOR's release tests, Clippy gate, formatting gate, and
diff check remained successful.

## Boundaries retained

The proof executes no FERRIS plan and no FERRIS-selected Cargo activity.
PARLOR's README commands remain the only required validation contract.
FERRIS remains experimental and unsupported; the pin is an exact consumer
compatibility boundary, not a support or stability claim.

## Decision

Complete the pulse without a successor. Merge only after the pull request's
consumer workflow passes against the exact event revision.
