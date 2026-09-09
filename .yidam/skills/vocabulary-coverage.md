# vocabulary-coverage

**Status: stub.** Approved in `.yidam/decisions/proposals.yml`; not implemented.

## What it computes

Two lists, which together are the central question's own arithmetic:

1. **Unexercised verbs** — every `verb` instance that no `interaction` instance `applies`.
   A verb nothing exercises is a primitive with no demonstrated need, which is the exact
   shape of evidence this corpus was built to collect.
2. **Undeclared verbs reached for** — every `applies` edge on an `interaction` whose target
   is not a declared `verb` instance. This is the vocabulary being too *small*, and it is
   the finding that argues for adding a primitive rather than removing one.

The two lists are reported together and never summed. They are opposite findings, and a
single "coverage percentage" over them would report neither.

## What it reads

- `.yidam/corpus/verb/*.yml` — the declared vocabulary
- `.yidam/corpus/interaction/*.yml` — the bindings
- the `applies` relationship between them

Nothing else. In particular it does not read `prior-art`: what some other engine's
vocabulary contained is a different question, answered by traversing
`prior-art -implements-> verb`.

## What it returns

Two lists of node paths, and the counts. No score.

## Prelude domain

None, deliberately. This is a set difference over verb names, and reaching for a shared
library to subtract two sets would be naming a domain because it sounds adjacent. See
`.yidam/decisions/proposals.yml`.

## Why it is a stub

It would run today and the answer would mean nothing. Three verbs are seeded against four
interactions; with a vocabulary that small, the uncovered set is a fact about how the corpus
was seeded rather than about the vocabulary. It becomes informative somewhere north of a
dozen verbs and enough interactions to have opinions about them.
