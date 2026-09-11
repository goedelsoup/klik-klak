# Actions — Room

Operations, queries and skills that apply to instances of [`room`](../room.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Which rooms a hotspot could belong to** — `yidam query 'hotspot -located-in-> room'`
- **Which rooms an actor can reach** — follow `connects-to` transitively; a room no chain
  reaches from the starting room is a room no player sees
- **What a room costs to encode** — `yidam neighbors room/<instance>.yml` and read the
  `data-format` nodes that `encodes` it
- **What `extent` is** — two fields rather than one property, and not a camera:
  [room-geometry](../../decisions/room-geometry.yml) read 22 rooms of a shipped game and found
  `room_size` and `room_bounds` differing in four of them, one with camera bounds taller than its
  background. A camera class is refused because the bounds are the only thing about that engine's
  camera with no setter.
