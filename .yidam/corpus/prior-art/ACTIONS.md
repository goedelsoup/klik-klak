# Actions — Prior Art

Operations, queries and skills that apply to instances of [`prior-art`](../prior-art.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What a system offered** — `yidam query 'prior-art -implements-> verb'`
- **What it let authors write wrong** — `yidam query 'prior-art -exhibits-> failure-mode'`
- **What it read from disk** — `yidam query 'prior-art -reads-> data-format'`
- **Upgrade a claim** — retrieve the system's documentation, add a `.yidam/catalog/` entry,
  cite it from the node, and promote `[inference]` to `[verified]`. Nothing proposes a
  promotion; it is a decision.
