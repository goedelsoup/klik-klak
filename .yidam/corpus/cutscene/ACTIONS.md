# Actions — Cutscene

Operations, queries and skills that apply to instances of [`cutscene`](../cutscene.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where a sequence plays** — `yidam query 'cutscene -plays-in-> room'`
- **What it writes** — `yidam query 'cutscene -sets-> world-flag'`
- **The skip problem** — read the `skippable` property; a sequence that writes state and
  can be cut short is the classic route into an inconsistent save
- **Whether the class collapses into `interaction`** — it does, on every ground it was seeded
  on, and the condition the verdict was hedged on has since been met by
  [arrival-trigger](../../decisions/arrival-trigger.yml).
  [scripted-sequence](scripted-sequence.yml) carries it; the class is kept unwithdrawn because the
  argument is worth more than the entry. That nothing points *at* a cutscene is not evidence
  either way — it is a source class, so nothing can.
