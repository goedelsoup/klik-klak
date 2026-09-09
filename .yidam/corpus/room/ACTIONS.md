# Actions — Room

Operations, queries and skills that apply to instances of [`room`](../room.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Which rooms a hotspot could belong to** — `yidam query 'hotspot -located-in-> room'`
- **Which rooms an actor can reach** — follow `connects-to` transitively; a room no chain
  reaches from the starting room is a room no player sees
- **What a room costs to encode** — `yidam neighbors room/<instance>.yml` and read the
  `data-format` nodes that `encodes` it
- *Open:* whether `extent` belongs on the room or on the camera. No node argues it yet.
