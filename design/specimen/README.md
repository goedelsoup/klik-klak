---
title: lighthouse — what a specimen found
status: proposal
rests_on:
  - design/bounded-expression-language.md
  - design/specimen/lighthouse.game
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/corpus/interaction.ont.yml
  - .yidam/corpus/interaction/binary-binding.yml
  - .yidam/corpus/interaction/acquisition-binding.yml
  - .yidam/corpus/inventory-item.ont.yml
  - .yidam/corpus/conversation.ont.yml
  - .yidam/corpus/world-flag/counter-flag.yml
  - .yidam/corpus/failure-mode/unwinnable-state.yml
  - .yidam/corpus/verb/give-to.yml
  - .yidam/catalog/ags-manual.md
  - .yidam/skills/reachability.md
  - .yidam/skills/vocabulary-coverage.md
derives_effects_from:
  - interaction:sets
  - interaction:opens
  - inventory-item:acquired-by
  - inventory-item:found-in
  - actor:occupies
---

# lighthouse — what a specimen found

[`lighthouse.game`](lighthouse.game) is a small adventure authored in the language proposed by
[`../bounded-expression-language.md`](../bounded-expression-language.md). Three rooms, one
actor, five items, two boolean flags, one counter, one conversation of three options, nine
hotspots and twenty-three bindings, in 187 lines.

It was written to break the proposal, and it did. **This document is the result, not the
advertisement.**

## The headline: the predicate layer held and the effect layer did not

Every precondition in the specimen is expressible in Layer 1. Not one needed a construct the
grammar lacks, and the guarded list — Layer 3, which the design argued was not a layer at all —
carried the whole burden of "pick among several lines" without complaint.

**Nine effects could not be expressed at all.** Each is marked `?? UNSAYABLE` at its site:

| What the game needs to say | Sites | Status in the ontology |
|---|---|---|
| an item leaves the inventory | 4 | nothing declares it |
| an object leaves the room | 2 | known gap; `acquired-by` names the binding only |
| an option's availability changes | 2 | known gap; no relationship writes option state |
| a conversation is left | 1 | `opens` enters one; nothing exits |

That inversion is worth sitting with. The design spent most of its length on the grammar,
excluded three constructs from it with arguments, and read the effect vocabulary off the
ontology in a table of four rows it treated as settled. The grammar needed no defending. **The
table of four was the whole problem.**

## The finding that undermines the design's own justification

The specimen has one deliberate, reachable defect. Oil is needed to fill the lamp, and a rusted
hinge on the ground floor will also accept it — so a player who oils the hinge has stranded
themselves, silently, with every remaining interaction still resolving normally. That is
[`unwinnable-state`](../../.yidam/corpus/failure-mode/unwinnable-state.yml) by **consumption**,
the first of the two causes it names.

The design's entire argument for building a bounded language was that it makes the checker
possible: *"A bounded predicate language is what makes the checker possible. A general-purpose
escape hatch is what makes it impossible."*

**But consumption is one of the nine things the effect vocabulary cannot say.** The specimen
contains exactly the defect the checker exists to find, and the language cannot express the
mechanism that produces it. A checker reading this game would find the oil acquired once and
never spent, and report the game winnable.

So the bounded language is not yet sufficient for the argument that motivated it. The escape
hatch was never the only thing standing between this corpus and a checker; the effect vocabulary
being too small does the same damage, quietly, and no amount of grammar fixes it.

That is the most useful thing this specimen produced and it is an argument against its own
proposal.

## Corrections to the design

**The effect table missed one, and it is not an ontology gap.** The specimen needs to move the
player between rooms, at four sites. `actor -occupies-> room` has been declared since genesis;
the design's table read it as a static fact about where an actor is rather than as something a
binding writes. Five effects, not four. Found by needing it rather than by re-reading.

**`response` has to split, and the class does not.**
[`interaction.ont.yml`](../../.yidam/corpus/interaction.ont.yml) gives `response` as one prose
property, and [`binary-binding`](../../.yidam/corpus/interaction/binary-binding.yml) folds the
state write into it: *"An action performed on the target using the carried item, and a write to
world state recording that it happened."* A specimen cannot do that. Effects are a closed list
and prose is not one, so every binding here splits into `say` and `do`. The class describes a
world in which the two are one field, and the proposal requires that they are two.

## Grammar coverage, and why it is weak evidence

Six constructs went unexercised: `or`, `count item`, `!=`, `<=`, `>`, and grouping parentheses.

`count item` is the one that stings. The design admitted it specifically to answer *"count an
inventory"* — one of the three cases the open question named as forcing the escape hatch — and a
whole game never reached for it. By
[`vocabulary-coverage`](../../.yidam/skills/vocabulary-coverage.md)'s own logic that is a
primitive with no demonstrated need.

**One specimen is much better evidence that a primitive *was* needed than that one is not.**
Twenty-three bindings in a deliberately small game is a thin basis for removing anything, and
the honest reading is that a second specimen — a game with a shop, a tally, or a collect-five
puzzle — would settle `count` properly and nothing here does. Recorded as a coverage result, not
as a verdict.

## The verb vocabulary, measured for the first time

The specimen produces `vocabulary-coverage`'s two lists by hand, on the first artifact this
corpus has held that could produce them at all.

**Used and not declared: `walk-to`**, at four sites. Movement is a verb in both systems this
corpus has read — AGS ships Mode 0 (Walk) and its Tumbleweed template carries `eMG_WalkTo`
([ags-manual](../../.yidam/catalog/ags-manual.md)) [verified] — and this corpus declares no
node for it. That is the vocabulary being too small, which is the finding that argues for
adding a primitive rather than removing one.

**Declared and not used: `give-to`.** No natural site for it arose. That corpus node already
carries `subsumed-by: use-with` from an earlier assessment made on documentary grounds, and this
is independent confirmation from a direction that assessment did not use: not *the manual says
giving is using-on*, but *a game got written and never wanted it*.

## What the corpus has no class for

The specimen needs two markers that belong to the game rather than to any part of it: `start`,
where play begins, and `ending`, the binding that finishes it. Fifteen classes model rooms,
verbs, bindings, flags and failures, and none models the artifact they compose into. `puzzle` is
a near-miss for `ending` — a final gated step — and nothing at all is a candidate for `start`.

Noted rather than proposed. This corpus does not add classes speculatively, and one specimen
needing two markers is not yet a class asking to exist.

## Using it as a fixture

`reachability` cannot run against this yet, for the reason above. When the effect vocabulary can
express consumption, this game is a three-case fixture:

- **Winnable.** Crowbar, then the crate for oil; the keeper for the wick and then the key;
  the door, the stair, the match; then wick, oil and match into the lamp in that order.
- **Unwinnable by consumption.** Oil the hinge in `base`. Reachable from the winning path, and
  the game continues answering normally forever.
- **Unwinnable by ordering** — not in the file, a two-line mutation. Add `do clear door-open` to
  the quay door's `walk-to`, and `when door-open` to `door-inner`'s. The door closes behind the
  player and the oil is on the wrong side of it.

The ordering case is left as a mutation rather than built in so that the committed specimen has
exactly one defect. A fixture with two makes a checker's output ambiguous about which it found.

## What this specimen is not

- **It is not evidence the language is pleasant to write.** It was written by the same party
  that proposed the language, which is the weakest possible authoring trial.
- **It is not a test of scale.** Twenty-three bindings is small enough to check by hand, which
  was the requirement, and far too small to say anything about a table with the combinatorics
  [`binary-binding`](../../.yidam/corpus/interaction/binary-binding.yml) describes.
- **It does not run.** No engine reads it. Every claim here is about what could be written down,
  not about what happened when it was played.
