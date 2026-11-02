# Pulse01 — unchanged native kernels in a bounded WASM worker

Public770ee14 contained newer metadata/Go/verify/FERRIS-shadow work than the
canonical66ddd11 PITFALL snapshot. The isolated branch reconciles both histories
at aea8094 without native changes relative to public entry. Retained doctrine,
source benchmarks, source/tests, owner commands and both FERRIS integrations are
protected. A new root lock (formerly ignored) enables locked builds of the same
six native packages; standalone companion lock does not change native membership.

Baseline33 native release tests, formatting/strict lint and exact pinned FERRIS
contract check pass. Four adapter tests and strict host/WASM lint pass. Fifty-four
repeated native/WASM responses agree, and36 checks independently compare original
CLI perft/legal counts, exact legal UCI lists, Go point/legal counts, backgammon
roll counts and invalid-FEN exit/diagnostics. The initial helper used package name
parlor-cli instead of declared binary parlor; failed log is retained and correcting
the helper path changes no kernel/output assertion.

Six real browser cases pass: all54 control outcomes and actual memory-growth
refusal beyond64MiB; all four kernel examples; own FEN/moves/checkers paths/
backgammon successor selections and explicit Go colors; native invalid inputs,
exact input/report exports, original BOM/CRLF bytes/explicit LF copy; mobile and
keyboard; unavailable-WASM input/source/build routes. Citations/metadata come
from native records. No invented universal game state, best move, chess strength,
Go turn enforcement or private-board getter is added.

Native limitations remain explicit. FEN syntax is not complete position/history
validation and can ignore clocks/extra fields. Go explicit colors need not
alternate; area score is not adjudication/komi/superko. Backgammon successors are
deduplicated resulting boards, not checker paths or stable future indices. A
perft admission traversal bounds visited nodes, while native perft supplies the
count. Debug text is not a portable save format.

Checkers' retained benchmark source string is generic rather than a named
external authority. The browser displays a repository expected reference and
source field, without strengthening it into newly verified external ground truth.
Native exhaustive enumeration proves only this kernel's move tree; retained
benchmark agreement does not establish complete rules correctness or repair
missing attribution. Existing native metadata/labels/benchmark values stay intact.

Before browser testing, review identified a pending example/file completion that
could overwrite current input during a newer Run intent. Run now invalidates
pending reads/examples; report validation also checks the expected game, and any
render/report failure clears partial results/exports. Expanded fault/race tests
must prove those fences. No native rule was changed.

Remaining gates: actual lifecycle/read/resource bounds and admitted maxima,
named expected-kind controls and budget refusal; complete exact corresponding
software/notices/budgets/provenance; outside-Git/path-with-spaces source replay;
exact final/default hosted qualification and fresh live proof; separate TRACKER
snapshot. Synthetic QA alone is uploaded until complete source/legal packaging.
