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

## The gate

```
mise run design-check      # or: ./design/check.sh
```

Runs inside `mise run ci` and as a step in the corpus CI job. It is bash, awk and grep — no
toolchain, so it behaves identically in a fresh clone. The honest reason it is a shell script
rather than a crate is in
[`.yidam/decisions/design-directory.yml`](../.yidam/decisions/design-directory.yml), along
with the condition under which it should stop being one.

## What this gate does not catch

Stated because a gate mistaken for complete is worse than no gate.

- **It checks a document, not a sentence.** The provenance check fires when a document uses
  `[verified]` and cites no catalog entry at all. A document that cites one source correctly
  and sources a second claim to nothing passes. Only a reader catches that.
- **It cannot tell a live premise from a stale one.** `rests_on:` verifies that the file
  exists, not that it still says what the design thought it said. A node revised out from
  under a design passes every check here. This repository has overturned two conclusions by
  re-reading material it already held; neither would have been caught by a path check.
- **It does not read the design.** Whether the thing proposed is coherent, minimal, or
  possible is not a property any script here inspects.
