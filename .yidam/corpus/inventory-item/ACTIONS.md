# Actions — Inventory Item

Operations, queries and skills that apply to instances of [`inventory-item`](../inventory-item.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where an item comes from** — `yidam query 'inventory-item -found-in-> room'`
- **What an item is the carried form of** — `yidam query 'inventory-item -carried-form-of-> hotspot'`
- **Which bindings need an item** — `yidam query 'interaction -requires-> inventory-item'`
- **Which items nothing reaches for** — the complement of `interaction -requires-> inventory-item`
  and `interaction -consumes-> inventory-item`. It is empty here, which is why whether such a thing
  is an item at all is carried as an open question on
  [portable-object](portable-object.yml) rather than answered on this page.
