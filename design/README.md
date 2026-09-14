# design

Where this repository stops recording and starts proposing.

Everything in [`.yidam/corpus/`](../.yidam/corpus/) is documentary: a node says what some
engine did, what some source states, or what follows from those. That discipline is the
corpus's whole value, and `agent-conduct` guards it — a claim carries `[verified]`,
`[inference]` or `[open]`, and asserting intent is not one of the three.

This corpus's central question eventually demands something the corpus cannot hold. Asking
*what is the smallest set of authoring primitives that can express a classic point-and-click
adventure* is answerable by comparison for as long as somebody else has built the thing being
compared. Where nobody has, the only way to find out what a primitive costs is to design one
and see what it cannot say. That is a proposal, and a proposal is not a documentary claim.

So it lives here rather than widening the corpus to admit it. The directory conventions are
explicit that this is the right shape: *"A new top-level directory is fine. Widening
`.yidam/corpus/` to hold something that is not a documentary claim is not."*

## The one rule this directory relaxes

**It may assert intent.** A document here says what a design *should* contain and why, which
is a claim about a thing that does not exist. Nothing in the corpus is permitted to do that.

## The rules it does not

**Provenance is unchanged, and the gate enforces it.** A design is free to propose and is not
free to invent evidence. Every claim here about what an existing system does carries the same
tag it would carry in the corpus and cites the same catalog entry, by link. `[verified]` still
means a committed source supports it. An aggregator still cannot carry one.

**Premises stay attached.** A design document declares `rests_on:` — the corpus nodes it is
derived from — and the gate fails when one of them is gone. This is the check the directory
exists to have. A design premised on a node the corpus has since withdrawn is standing on
nothing, and it will not look any different while it does.

**The effect vocabulary is read off the ontology, not invented here.** A document declares
`derives_effects_from:` as `class:relationship` pairs, and each must be a relationship that
class actually declares. When one is not, the failure is deliberately ambiguous: either the
design named an effect that does not exist, or the ontology is missing a relationship the
design has just discovered it needs. Both are findings. Silence is not available.

## Front matter

```yaml
---
title: A bounded expression language for authored interaction
status: proposal          # proposal | adopted | withdrawn
reconciled: 02cf125a52…   # the commit the premises below were last read against
rests_on:
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/catalog/ags-manual.md
derives_effects_from:
  - interaction:sets
  - interaction:opens
---
```

`status` is required because a reader cannot otherwise tell a live proposal from one the
corpus has already refuted, and a directory of undated ambition is worth less than nothing.

`reconciled` is required for the same reason one step further out. A premise moving is not a
defect and will never look like one, so what is checked is that somebody read the document
since it moved — and the gate fails naming the premise and the count. Not a date and not an
interval: **an event**, and the event is the one that should send a reader back.
[`.yidam/decisions/moved-ground.yml`](../.yidam/decisions/moved-ground.yml) built it and
[`second-document`](../.yidam/decisions/second-document.yml) made it required, after it went on
the one document of two that somebody happened to be reading.

## Specimens

A `design/*/` subdirectory holds a worked example authored in a proposed language — a
specimen. It is part of the proposal it exercises rather than a thing beside it, because a
design nothing exercises cannot be wrong. See
[`.yidam/decisions/specimen-placement.yml`](../.yidam/decisions/specimen-placement.yml).

A specimen's `.game` file is checked for one property: every name a binding reaches for must
be declared by the specimen that uses it. That is the predicate layer's own admission rule —
*an operand names something declared* — applied to a game's declarations instead of the
corpus's, and it is the strongest property available without an engine to run.

The gate checks names and kinds. `crates/reachability` separately checks whether an ending is
reachable and reports stranded states; specimens deliberately retain the traps they exercise.

## The gate

```
mise run design-check      # cargo test -p design-gate --manifest-path crates/Cargo.toml
```

Runs inside `mise run ci` as the Rust `design-gate` tests. The earlier shell gate was replaced
when the domain workspace appeared; [domain-computer](../.yidam/decisions/domain-computer.yml)
records that transition.

## What this gate does not catch

Stated because a gate mistaken for complete is worse than no gate.

- **It checks a document, not a sentence.** The provenance check fires when a document uses
  `[verified]` and cites no catalog entry at all. A document that cites one source correctly
  and sources a second claim to nothing passes. Only a reader catches that.
- **It cannot tell a live premise from a stale one, and it now says when one moved.**
  `rests_on:` verifies that the file exists, not that it still says what the design thought it
  said — that half is unchanged and unfixable by a path check. What the watermark adds is a
  baseline: the gate fails when a premise has a commit after `reconciled:`, so a node revised
  out from under a design is loud rather than silent. It still takes a reader to say whether
  the revision mattered, and both times this repository overturned a conclusion by re-reading
  material it already held, a reader is what did it.
- **It does not read the design.** Whether the thing proposed is coherent, minimal, or
  possible is not a property any script here inspects.
