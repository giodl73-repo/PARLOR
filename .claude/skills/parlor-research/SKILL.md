# PARLOR Research

Use this skill before adding a new game or a shared analysis capability, to fix
the cited ground truth a kernel will be verified against.

## When to use

- Admitting a new game crate (what are the authoritative rules and benchmarks?).
- Adding an analysis surface that emits quantitative claims (what authority backs
  the numbers?).

## Workflow

1. Identify the authoritative rules source (e.g. FIDE Laws of Chess, official
   backgammon rules) and the machine-checkable benchmark (perft tables, published
   match-equity tables, endgame databases).
2. Record each source in `data/sources.md` with a stable id and the exact figures
   to be reproduced.
3. Decide the evidence label for every emitted number: `Cited` for external
   authorities, `Proven` for in-kernel exhaustive enumeration, `Heuristic` /
   `Estimated` for evaluations and rollouts.
4. Only then implement, pinning tests to the cited values.

## Output

- `data/sources.md` rows for every benchmark/table the kernel reproduces.
- A short note in the wave/pulse record naming the chosen authorities.
