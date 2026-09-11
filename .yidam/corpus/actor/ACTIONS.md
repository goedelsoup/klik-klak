# Actions — Actor

Operations, queries and skills that apply to instances of [`actor`](../actor.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where an actor can be** — `yidam query 'actor -occupies-> room'`
- **Where an actor can stand** — `yidam query 'actor -traverses-> walkbox'`
- **Who can be talked to** — `yidam query 'conversation -held-with-> actor'`; an actor no
  conversation reaches is scenery that happens to walk
- **Whether switching the controlled actor needs vocabulary** — carried as an open question on
  [player-character](player-character.yml), which this page was restating in a spelling nothing
  reads. Not to be confused with control returning from a *mode*, which is settled and is
  `conversation -returns-control-to-> actor`.
