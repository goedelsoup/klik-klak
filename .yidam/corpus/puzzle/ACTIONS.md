# Actions — Puzzle

Operations, queries and skills that apply to instances of [`puzzle`](../puzzle.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What opens a gate** — `yidam query 'puzzle -solved-by-> interaction'`
- **What holds it closed** — `yidam query 'world-flag -gates-> puzzle'`
- **Which puzzles can strand a player** — `yidam query 'failure-mode -arises-from-> puzzle'`
- *Stub:* `reachability` would answer this mechanically. It has nothing to read — see
  `.yidam/decisions/seed-scope.yml`.
