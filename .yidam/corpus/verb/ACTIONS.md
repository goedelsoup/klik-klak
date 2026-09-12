# Actions — Verb

Operations, queries and skills that apply to instances of [`verb`](../verb.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Which verbs are exercised** — `yidam query 'interaction -applies-> verb'`; the
  complement is the finding
- **Which engine a verb's name is drawn from** — `yidam query 'verb -originated-in-> prior-art'`
- **Arity census** — group by the `arity` property; the binary verbs are what force the
  second-object machinery on everything downstream
- **Which constructs an author has never written** —
  `cargo test -p game-format --test coverage`, over the four specimens. This is what
  `vocabulary-coverage` was scaffolded to compute and is not how it was scaffolded to compute it:
  the question is about the language rather than about the graph of claims, and the skill is
  retired. `.yidam/decisions/calculators.yml`, `.yidam/decisions/unwritten.yml`.
