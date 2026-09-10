---
title: lighthouse — what a specimen found
status: proposal
rests_on:
  - design/bounded-expression-language.md
  - design/specimen/lighthouse.game
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/corpus/interaction.ont.yml
  - .yidam/corpus/interaction/binary-binding.yml
  - .yidam/corpus/interaction/consuming-binding.yml
  - .yidam/corpus/interaction/acquisition-binding.yml
  - .yidam/corpus/inventory-item.ont.yml
  - .yidam/corpus/conversation.ont.yml
  - .yidam/corpus/world-flag.ont.yml
  - .yidam/corpus/world-flag/latched-flag.yml
  - .yidam/corpus/world-flag/counter-flag.yml
  - .yidam/corpus/failure-mode/unwinnable-state.yml
  - .yidam/corpus/verb/give-to.yml
  - .yidam/catalog/ags-manual.md
  - .yidam/skills/reachability.md
  - .yidam/skills/vocabulary-coverage.md
derives_effects_from:
  - interaction:sets
  - interaction:opens
  - interaction:consumes
  - conversation:sets
  - conversation:returns-control-to
  - inventory-item:acquired-by
  - inventory-item:found-in
  - actor:occupies
---

# lighthouse — what a specimen found

[`lighthouse.game`](lighthouse.game) is a small adventure authored in the language proposed by
[`../bounded-expression-language.md`](../bounded-expression-language.md). Three rooms, one
actor, five items, two boolean flags, one counter, one conversation of three options, nine
hotspots, twenty-three bindings.

It was written to break the proposal. It did, once — and then the break turned out to be
four-fifths the specimen's own author, which is the more useful finding.

## What happened, in order

**First writing.** Nine sites could not be expressed and were marked `?? UNSAYABLE`: an item
leaving the inventory (4), an object leaving the room (2), an option's availability changing
(2), a conversation being left (1). This document concluded that the effect vocabulary "was the
whole problem".

**Then each of the four was weighed against what the ontology already declared**, which is the
rule AGENTS.md states and which this document had not applied. Three could be expressed:

| Marked unsayable | Verdict |
|---|---|
| an object leaves the room | **Entailed by `carried-form-of`.** `inventory-item` is a UFO role; the hotspot is what the same object is when not carried. Acquiring it *is* the room losing it. One change described twice. |
| an option's availability | **A conversation-scoped `world-flag`.** That scope value has been declared since genesis. |
| a conversation is left | **`returns-control-to`**, declared on `cutscene` since that class was seeded, and it fits without alteration. |
| an item leaves the inventory | **Real.** Nothing said an object is spent. Now `interaction -consumes-> inventory-item`. |

**Second writing.** Every `?? UNSAYABLE` is gone. The cost of the four was **one new
relationship, one new property** (`world-flag.reversible`, for the one third of option state that
does not reduce — a permanent disable) **and one existing relationship applied to a second
class.**

## The finding this specimen is actually for

**A specimen measures where its author could not write something down. That is not a measurement
of the ontology, and nothing in the method separates the two.**

Three of four sites were *I did not find the way this says it*, by the same party who wrote both
the design and the specimen — after the same party had already missed `actor -occupies-> room`
in the design's own effect table. Three misreadings of one ontology in two phases.

That is the failure this repository has now recorded four times and has as a conduct norm about
*sources*: material accurately held, then reasoned about from memory rather than from the
record. It applies to an ontology exactly as it applies to a catalog entry. `yidam
open-questions` would have gone further still — `puzzle/state-gate` had asked in prose for
`world-flag.reversible`, by name, two phases before it was added here from the other direction
entirely. It was not run.

So the honest reading of the first writing is not that the effect vocabulary was too small. It
is that **a worked example is an excellent detector of trouble and a poor diagnostician of it.**
Every site it marks is worth checking; roughly one in four was what it said it was.

## The one that was real, and why it was the one that mattered

The specimen carries a deliberate reachable defect: oil needed for the lamp is also accepted by a
rusted hinge, so a player who oils the hinge is stranded silently while every remaining
interaction resolves normally. That is
[`unwinnable-state`](../../.yidam/corpus/failure-mode/unwinnable-state.yml) by **consumption**,
the first cause it names — and consumption was the gap that survived.

Which is why the count was the wrong measure. Four of the nine sites were the single defect that
made the design's whole justification fail: a bounded language exists to make the checker
possible, and the checker could not have seen this game's only bug.

It can now. `spend oil-can` is an edge, and a checker asks the graph which bindings spend a given
item instead of asking prose.

`consuming-binding` also says something the specimen only hinted at: this is **the only binding
shape whose success is the mistake.** Every other shape resolves correctly or refuses, and a
refusal is a message. This one resolves correctly on the wrong target and reads as a success,
because at that moment it is one.

## A new finding from the rewrite: predicates delete two of AGS's eight commands

Once availability is a conversation-scoped flag, the option in the rewritten specimen reads

```
option lamp "The lamp is out."
  when  not asked-lamp
```

— a `when` re-read each time the list is displayed. There is no `disable` effect and none is
wanted.

**`option-on` and `option-off` exist because the command list cannot ask a question.**
[ags-manual](../../.yidam/catalog/ags-manual.md) documents all eight and none of them reads
anything [verified]; an option whose availability depends on the world therefore has to be
switched from outside, imperatively, at every site that changes the world. [inference] Given a
predicate layer, both commands are unnecessary — availability is a function of state rather than
a copy of it.

`option-off-forever` is the exception and is the reason `world-flag.reversible` exists. A
permanent disable is not a value but a constraint on one, and no predicate re-read at display
time can express *and this can never come back*.

That closes a loop with the design's first finding. The bounded command list had to write
because it could not read, and three of its eight commands are the bill for that.

## Grammar coverage, and why it is weak evidence

Six constructs went unexercised across both writings: `or`, `count item`, `!=`, `<=`, `>`, and
grouping parentheses.

`count item` is the one that stings. The design admitted it specifically to answer *"count an
inventory"* — one of the three cases the open question named as forcing the escape hatch — and a
whole game never reached for it. By
[`vocabulary-coverage`](../../.yidam/skills/vocabulary-coverage.md)'s own logic that is a
primitive with no demonstrated need.

**One specimen is much better evidence that a primitive *was* needed than that one is not.**
Twenty-three bindings in a deliberately small game is a thin basis for removing anything, and a
second specimen — a shop, a tally, a collect-five puzzle — would settle `count` where this
cannot.

## The verb vocabulary, measured

**Used and not declared: `walk-to`**, at four sites. Movement is a verb in both systems this
corpus has read — AGS ships Mode 0 (Walk) and its Tumbleweed template carries `eMG_WalkTo`
([ags-manual](../../.yidam/catalog/ags-manual.md)) [verified] — and no `verb` node declares it.
That is the vocabulary being too small, which is the finding that argues for *adding* a
primitive. Unchanged by the rewrite.

**Declared and not used: `give-to`.** No natural site arose in either writing. That node already
carries `subsumed-by: use-with` from an assessment made on documentary grounds; this agrees from
a direction that assessment did not use.

## What the corpus has no class for

The specimen needs two markers belonging to the game rather than to any part of it: `start`,
where play begins, and `ending`, the binding that finishes it. Fifteen classes model rooms,
verbs, bindings, flags and failures, and none models the artifact they compose into. `puzzle` is
a near-miss for `ending`; nothing is a candidate for `start`.

Noted rather than proposed — and noted more carefully after this phase, since the last four
things this document called missing were mostly not.

## Using it as a fixture

[`reachability`](../../.yidam/skills/reachability.md) can now describe this game. Three cases:

- **Winnable.** Crowbar, then the crate for oil; the keeper for the wick and then the key; the
  door, the stair, the match; then wick, oil and match into the lamp in that order.
- **Unwinnable by consumption.** Oil the hinge in `base`. Reachable from the winning path, and
  the game continues answering normally forever. Expressible for the first time.
- **Unwinnable by ordering** — a two-line mutation rather than committed. Add `do clear
  door-open` to the quay door's `walk-to`, and `when door-open` to `door-inner`'s.

The ordering case stays a mutation so the committed specimen has exactly one defect. A fixture
with two makes a checker's output ambiguous about which it found.

One flag latches (`crate-open`) and one counter does (`lamp-prep`), which is what makes the
partial-order walk in [`latched-flag`](../../.yidam/corpus/world-flag/latched-flag.yml) testable
against something rather than argued in the abstract.

## What this specimen is not

- **Not evidence the language is pleasant to write.** It was written by the party that proposed
  the language, which is the weakest possible authoring trial, twice over now.
- **Not a test of scale.** Twenty-three bindings is small enough to check by hand, which was the
  requirement, and far too small to say anything about a table with the combinatorics
  [`binary-binding`](../../.yidam/corpus/interaction/binary-binding.yml) describes.
- **It does not run.** No engine reads it. Every claim here is about what could be written down.
