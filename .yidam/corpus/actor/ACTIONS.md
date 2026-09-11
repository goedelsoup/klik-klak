# Actions — Actor

Operations, queries and skills that apply to instances of [`actor`](../actor.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Where an actor can be** — `yidam query 'actor -occupies-> room'`
- **Where an actor can stand** — `yidam query 'actor -traverses-> walkbox'`
- **Who can be talked to** — `yidam query 'conversation -held-with-> actor'`; an actor no
  conversation reaches is scenery that happens to walk
- **Where control goes when a mode ends** — `yidam query 'conversation -returns-control-to-> actor'`,
  and the same from `cutscene`. It needed no vocabulary of its own and does not fall out of
  `world-flag`: control transfer is a *mode in the state vector*, entered and left by those two
  edges ([effect-list-semantics](../../decisions/effect-list-semantics.yml)), which is an answer
  to neither branch this page offered.
