# Actions — Inventory Item

Operations, queries and skills that apply to instances of [`inventory-item`](../inventory-item.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where an item comes from** — `yidam query 'inventory-item -found-in-> room'`
- **What an item is the carried form of** — `yidam query 'inventory-item -carried-form-of-> hotspot'`
- **Which bindings need an item** — `yidam query 'interaction -requires-> inventory-item'`
- *Open:* whether an item that is never the second object of any binding is an item at all,
  or scenery with an icon.
