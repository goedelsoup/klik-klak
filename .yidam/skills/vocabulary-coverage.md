---
name: vocabulary-coverage
description: Which verbs no interaction exercises, and which interactions reach for a verb the vocabulary does not declare
---

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

## Both lists have been computed once, by hand, and not from the corpus

`design/specimen/lighthouse.game` is a game authored in this vocabulary — twenty-three bindings
across three rooms — and it produced both lists on the first artifact this repository has held
that could produce either.

- **Undeclared and reached for: `walk-to`**, at four sites. Movement is a verb in both systems
  this corpus has read, and no `verb` node declares it. This is the list that argues for adding
  a primitive, and it has its first member.
- **Declared and unexercised: `give-to`.** No natural site for it arose, which is independent of
  and agrees with the documentary assessment already in that node.

Two things follow for this skill and neither is that it should now run.

The specimen is **not corpus input**. This calculator reads `verb` and `interaction` instances,
and a specimen is a `design/` artifact — so the answer above was reached by reading a file this
skill does not and should not read. Whether a calculator over the corpus and a coverage report
over a specimen are the same computation with two inputs, or two different questions, is
`[open]`.

And the second list is the harder one to compute either way: a binding reaching for an
undeclared verb is only visible where bindings exist, and the corpus's `interaction` instances
all cite verbs that exist because they were written to. The specimen found `walk-to` because it
was written to be a game rather than to be an example.
