# Actions — Walkbox

Operations, queries and skills that apply to instances of [`walkbox`](../walkbox.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Which room a surface belongs to** — `yidam query 'walkbox -lies-in-> room'`
- **Who moves on it** — `yidam query 'actor -traverses-> walkbox'`
- **What solves paths across it** — `yidam query 'subsystem -realizes-> walkbox'`
- **Whether adjacency needs a relationship** — carried as an open question on
  [scaled-polygon](scaled-polygon.yml). The reason this page gave for not answering it — one
  instance, nothing for an edge to join — stopped being true when
  [room-geometry](../../decisions/room-geometry.yml) measured 84 polygons across 15 rooms.
