# Actions — Puzzle

Operations, queries and skills that apply to instances of [`puzzle`](../puzzle.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What opens a gate** — `yidam query 'puzzle -solved-by-> interaction'`
- **What holds it closed** — `yidam query 'world-flag -gates-> puzzle'`
- **Which puzzles can strand a player** — `yidam query 'failure-mode -arises-from-> puzzle'`
- **Answer it mechanically** — `cargo run -p reachability -- design/specimen/lighthouse.game`.
  The reason this page gave for the stub was that there was nothing to read
  (`.yidam/decisions/seed-scope.yml`); `design/specimen/` now holds four games.
