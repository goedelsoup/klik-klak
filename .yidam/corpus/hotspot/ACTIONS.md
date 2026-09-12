# Actions — Hotspot

Operations, queries and skills that apply to instances of [`hotspot`](../hotspot.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What can be done to a region** — `yidam query 'interaction -targets-> hotspot'`
- **Which regions no interaction reaches** — a hotspot nothing targets is a region the
  player can point at and get only a default response from
- **Whether a hotspot is really an actor that cannot move** — the live collapse; see
  `.yidam/decisions/ontology.yml`
- **Whether the collapse is decidable by a score** — it is not, and `collapse-signature` is
  retired for saying so on its own page: a similarity over declared vocabularies cannot see the
  relationship that is missing from *both* classes, which is what decided the first collapse.
  `.yidam/decisions/calculators.yml`.
