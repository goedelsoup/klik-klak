# Actions — Cutscene

Operations, queries and skills that apply to instances of [`cutscene`](../cutscene.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where a sequence plays** — `yidam query 'cutscene -plays-in-> room'`
- **What it writes** — `yidam query 'cutscene -sets-> world-flag'`
- **The skip problem** — read the `skippable` property; a sequence that writes state and
  can be cut short is the classic route into an inconsistent save
- *Open:* the collapse into `interaction`. Nothing points *at* a cutscene in this ontology,
  which is itself a hint about its standing.
