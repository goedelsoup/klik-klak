# collapse-signature

**Status: stub.** Approved in `.yidam/decisions/proposals.yml`; not implemented.

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

`similarity` — vendored at `.yidam/.vendor/prelude/domains/similarity/`. The Jaccard
implementation there is pinned across Rust, TypeScript and Python by shared fixtures, which is
worth more than a local reimplementation of a formula that is easy to get subtly wrong.

## Why it is a stub

With 30 instances spread across 15 classes, the ranking would be dominated by how thinly each
class is seeded rather than by how alike the classes are. A class with one instance has one
edge signature, and one signature is not a distribution.
