---
name: collapse-signature
description: Similarity between class pairs' relationship vocabularies — the falsifiable form of 'is cutscene just an interaction?'
status: stub
---

# collapse-signature

**Status: retired.** Approved in `.yidam/decisions/proposals.yml` at genesis; never implemented;
retired after 69 of this repository's 86 phases because **the question it was approved to answer
has been answered, by an artifact it does not read** — and because the section below already
argued that a similarity score is not the instrument that answers it.

`corpus/cutscene/scripted-sequence.yml` settled *is `cutscene` just an `interaction`?* against a
specimen written to want a sequence twice. The page is kept rather than deleted, the way
`verb/give-to.yml` is kept: a list that grows by accident needs a record of what it declined.
`.yidam/decisions/calculators.yml`.

## What it computes

For each pair of classes, the similarity between their relationship vocabularies and between
their instances' edge signatures — Jaccard over the set of `(relationship, target-class)`
pairs each participates in. High similarity between two classes is evidence that one is the
other wearing a different name.

This makes falsifiable the two collapses `.yidam/decisions/ontology.yml` records as live and
deliberately unjudged:

- **`cutscene` into `interaction`** — a cutscene may be an interaction whose response is long
  and whose input is locked. Note before running it that `cutscene` is a source class in this
  ontology: nothing points at it, which is a hint the metric will not see.
- **`hotspot` into `actor`** — a hotspot may be an actor that cannot move. The counter-argument
  is in `actor/non-player-character.yml`: a collapse gives every targetable region a position,
  a behaviour slot and a conversation slot it never uses, which is a larger vocabulary wearing
  a smaller class count.

## What it reads

- every `.yidam/corpus/*.ont.yml` — the declared relationship vocabulary per class
- every `.yidam/corpus/*/*.yml` — each instance's actual edge signature

Both, and the disagreement between them is worth reporting on its own: a class whose declared
vocabulary is broad and whose instances exercise little of it is a class that has not been
tested rather than one that is redundant.

## What it returns

Ranked class pairs with a similarity score, and per pair the relationships they share and the
ones only one of them has. **The score is not a verdict.** Whether two classes should be one
is a judgement about the domain; this narrows where to spend it.

## Prelude domain

`similarity` — vendored at `.yidam/.vendor/domains/similarity/`. The Jaccard
implementation there is pinned across Rust, TypeScript and Python by shared fixtures, which is
worth more than a local reimplementation of a formula that is easy to get subtly wrong.

## Why it stayed a stub

With 30 instances spread across 15 classes, the ranking would be dominated by how thinly each
class is seeded rather than by how alike the classes are. A class with one instance has one
edge signature, and one signature is not a distribution.

## The first class collapse was argued without it

`cutscene` has been assessed and reduces into `interaction` on every ground it was seeded on
(`.yidam/corpus/cutscene/scripted-sequence.yml`). This skill was scaffolded at genesis to make
exactly that argument — "the falsifiable form of *is cutscene just an interaction?*" — and it
was not what produced the answer. A **specimen** was: four rooms written to want a sequence
twice, in two shapes, so that each of the class's properties had something concrete to fail at.

That is worth recording here rather than only in the corpus, because it bears on whether this
skill should be built at all.

What the ranking would have compared is relationship vocabularies: `cutscene` declares
`plays-in`, `sets` and `returns-control-to`, and `interaction` declares `applies`, `targets`,
`requires`, `sets`, `opens` and `consumes`. A similarity score over those two sets would have
reported a modest overlap and could not have told anyone *why* — that `plays-in` reduces because
a binding is already sited in a room, that `returns-control-to` reduces because an atomic binding
never took control, or that what survives is a **trigger neither class declares an edge for.**
The decisive observation is about a relationship that is missing from both, and a comparison of
what each declares cannot see it.

So the stub stood, and for a sharper reason than "the classes are thinly seeded". The question
this skill was built to answer is answerable, and a similarity score over declared vocabularies is
not the instrument that answers it.

**That is now the reason it is retired rather than something left unsettled.** A skill whose own
page argues it would not answer its question is not waiting for more data; it is waiting for
somebody to say so. `.yidam/decisions/calculators.yml`.
