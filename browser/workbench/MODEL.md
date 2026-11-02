# Native quantities and limits

Every request runs native game APIs; browser JavaScript formats results only.
Chess accepts native FEN syntax and selects supplied UCI moves from native legal
lists before calling unchecked make. Checkers does likewise for native path
indices from its opening. Private board representations stay private: native
debug strings are diagnostics, not portable interchange/save formats.

Perft0..3 has a100,000-visited-node admission preflight using native legal/make
APIs. The final count and divides come from native perft, not that admission
walk. Proven describes exhaustive enumeration of this kernel's move tree; it
does not certify all chess/draughts rules or the historical legality of an input.
Retained benchmark expectations/sources are Cited data from the repository;
only exact request/depth matches are marked. Deeper retained native checks are
not executed in the browser. Mismatches must never be silently re-baselined.

Native FEN parsing does not validate complete game legality, kings or clocks;
additional fields can be ignored by native parsing and remain in exact inputs.
No FEN/state reconstruction is invented. Native legal behavior, warnings/absence
of validation and errors remain visible, not repaired by browser rules.

Backgammon uses native pip_count/legal_plays/Dice. Supplied players/rolls and
successor selections are explicit, not randomized or alternating-turn enforced.
Results are distinct successor boards, not exposed checker-move paths, stable
future IDs, recommendations, sampled odds or match equity. Dice floats retain
native values. Go runs explicit-color native placements/captures/legal moves and
area scoring: native play does not enforce turn alternation; simple ko, no new
pass/komi/superko/adjudication. Placement events are not an invented board getter.

Request admission256KiB, FEN4KiB, sequences128 (backgammon16), Go odd1..19,
dice1..6, report2MiB, memory64MiB. Fixed-width transport indices avoid native64/
WASM32 parsing differences. File/read/startup/computation limits and generation/
revision fences protect current input. Original bytes, current/LF editor copy,
executed request and report remain separate. Source/build links remain independent
of engine startup. No online play, strength, wagering or FERRIS substitution.

Citation-strength boundary: the checkers benchmark source string is generic,
not a named external authority. It is displayed as a retained repository
reference; this browser does not independently verify or strengthen it. Native
exhaustive counts establish only this kernel’s move tree. Benchmark agreement
does not promote missing source attribution into external rules certification.
