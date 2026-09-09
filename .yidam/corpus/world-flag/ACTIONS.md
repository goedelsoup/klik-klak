# Actions — World Flag

Operations, queries and skills that apply to instances of [`world-flag`](../world-flag.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What a flag holds closed** — `yidam query 'world-flag -gates-> puzzle'`
- **What writes it** — `yidam query 'interaction -sets-> world-flag'` and the same from
  `conversation` and `cutscene`
- **Save surface** — read `save_semantics`; a flag omitted from a save is a flag whose
  puzzle reopens on load
- *Open:* whether `scope` is a real axis or an optimization. A room-local flag is a global
  flag nobody reads elsewhere.
