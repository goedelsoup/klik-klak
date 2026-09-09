# Actions — Actor

Operations, queries and skills that apply to instances of [`actor`](../actor.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where an actor can be** — `yidam query 'actor -occupies-> room'`
- **Where an actor can stand** — `yidam query 'actor -traverses-> walkbox'`
- **Who can be talked to** — `yidam query 'conversation -held-with-> actor'`; an actor no
  conversation reaches is scenery that happens to walk
- *Open:* whether control transfer needs vocabulary of its own, or falls out of `world-flag`.
