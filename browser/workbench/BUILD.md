# Build the local rules-workbench candidate

Use Rust1.95.0/wasm32-unknown-unknown, wasm-bindgen CLI0.2.127, Python3.11+
and Node22/Playwright1.61.1. The unpublished standalone browser/workbench crate
uses unchanged native crates from the same source repository. Root and companion
locks are committed; no TRACKER path or FERRIS runtime dependency is introduced.

Run build-prototype.py --bindgen <absolute binding executable>. It stages
target/kernel-prototype/PARLOR and private synthetic native controls. Serve the
parent directory under /PARLOR/. Source/legal packaging and deployment are not
qualified by this candidate build. Use an absolute CARGO_TARGET_DIR for shared
build cache custody. Native owner tests/format/lint and pinned FERRIS controls
remain independent mandatory checks.
