# Build the native rules workbench

Use Rust1.95.0 with wasm32-unknown-unknown and rust-docs, wasm-bindgen CLI0.2.127,
Python3.11+ and Node22/Playwright1.61.1. Root and companion locks are committed;
native workspace contains six unchanged game/CLI packages. Browser companion
uses explicit sibling source paths inside this repository, never TRACKER paths.
Dependencies require network access and are not vendored. An absolute
CARGO_TARGET_DIR selects a shared build cache.

From extracted source or repository root:

```powershell
python browser/workbench/build-prototype.py --bindgen C:/path/to/wasm-bindgen.exe
cd browser/workbench
npm ci
npx playwright install chromium
$env:PARLOR_SITE_DIR='../../target/kernel-prototype'
npm run test:pages -- --grep-invert "packaged delivery|source/legal"
```

Native owner command is cargo test --release --locked --workspace. Companion
release tests and host/WASM lint run with its --manifest-path. The builder repeats
76 native controls and36 independent original CLI count/list/diagnostic controls.
Seventeen behavior cases need no Git or external control archive. This text
client renders native records and private-board debug diagnostics; no game rules
run in JavaScript. FERRIS remains a full-checkout owner-control concern, not a
build/runtime dependency; see SOURCE-SCOPE.md for exact scope and original grants.

Release maintainers require clean Git, verify-foundation.py, prototype build,
package-site.py and verify-site.py <site> --commit <full HEAD>. Full nineteen
browser cases use default target/kernel-release (unset PARLOR_SITE_DIR). Finally
verify-source.py --site target/kernel-release/PARLOR --bindgen <path> rebuilds
source outside Git/path with spaces, verifies first-party paths, repeats33 native/
four adapter tests, both76-outcome sets,36 original CLI controls and17 behavior
cases. Binary identity across paths/platforms is not claimed. Complete source,
notices/budgets/hashes must pass before binary upload; default deploy uses the
same verified artifact. Exact pinned FERRIS consumer check and retained owner
validation remain mandatory in full-checkout CI and are not replaced by this UI.

Full-checkout release guards require full Git history including the preserved
baseline/public-pitfall-reconciled-20261102 branch at aea8094 and the original
PITFALL branch at66ddd11. Ordinary extracted-source builds require no ancestry.
