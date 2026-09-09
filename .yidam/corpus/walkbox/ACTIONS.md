# Actions — Walkbox

Operations, queries and skills that apply to instances of [`walkbox`](../walkbox.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Which room a surface belongs to** — `yidam query 'walkbox -lies-in-> room'`
- **Who moves on it** — `yidam query 'actor -traverses-> walkbox'`
- **What solves paths across it** — `yidam query 'subsystem -realizes-> walkbox'`
- *Open:* whether adjacency between walkboxes needs its own relationship. With one instance
  seeded there is nothing yet for such an edge to join.
