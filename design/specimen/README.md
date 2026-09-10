---
title: what the specimens found
status: proposal
rests_on:
  - design/bounded-expression-language.md
  - design/specimen/lighthouse.game
  - design/specimen/ferry.game
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/corpus/interaction.ont.yml
  - .yidam/corpus/interaction/binary-binding.yml
  - .yidam/corpus/interaction/consuming-binding.yml
  - .yidam/corpus/interaction/acquisition-binding.yml
  - .yidam/corpus/inventory-item.ont.yml
  - .yidam/corpus/inventory-item/stackable-item.yml
  - .yidam/corpus/conversation.ont.yml
  - .yidam/corpus/world-flag.ont.yml
  - .yidam/corpus/world-flag/latched-flag.yml
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

# what the specimens found

Two games authored in the language proposed by
[`../bounded-expression-language.md`](../bounded-expression-language.md), each written to
break it.

| | [`lighthouse.game`](lighthouse.game) | [`ferry.game`](ferry.game) |
|---|---|---|
| written for | *can this say a game at all?* | *does `count item` earn its place?* |
| committed defect | consumption | ordering |
| reachable states | 103 of 196,608 observed combinations (0.05%) | 20 of 480 (4%) |

**The single most useful thing either produced is a fact about method.** A specimen measures
where its author could not write something down, and where a tool disagrees with a game. It is
an excellent detector of trouble and a poor diagnostician of it, and every finding below had to
be weighed against what the ontology already said before it counted as a gap.

## lighthouse — the effect vocabulary

Nine sites could not be expressed on first writing: an item leaving the inventory (4), an
object leaving the room (2), an option's availability changing (2), a conversation being left
(1). This document concluded the effect vocabulary "was the whole problem".

**One of the four was real.** The other three were already expressible — an object leaving the
room is entailed by `carried-form-of`, option availability is a conversation-scoped
`world-flag`, and leaving an exchange is `returns-control-to`, which `cutscene` had declared
since it was seeded. Roughly one site in four was what it said it was. The reasoning is in
`.yidam/decisions/effect-vocabulary.yml`.

The one that was real mattered more than the count suggested. The lighthouse's deliberate
defect is a can of oil poured into a rusted hinge instead of the lamp — `unwinnable-state` by
**consumption**, the first cause it names — and consumption was the gap. So the game contained
exactly the defect a checker exists to find, and the language could not express the mechanism
producing it. `interaction -consumes-> inventory-item` closed that.

## ferry — the tally

`count item` was admitted to the design to answer *"count an inventory"*, one of the three
cases the open question named as forcing an escape hatch, and the lighthouse never reached for
it. So the ferry is a tally: three coins for a river crossing.

**It was written first with distinct items**, which is what the vocabulary had. Three coins are
three declarations carrying identical labels, the toll is a three-term conjunction, and the
verb has to pick one coin arbitrarily as its second object while the others go by side effect
— `use-with coin-a`, with coin-b and coin-c spent invisibly. Each of those grows with the tally.

**What does not reduce is "any three of five".** With distinct items and no quantifier that is a
ten-term disjunction of three-term conjunctions, growing combinatorially. The design excluded
quantifiers on the stated grounds that they need a collection and the vocabulary has none — so
`count` earns its place exactly if an item may bear a quantity, because a quantity-bearing item
**is** that collection. See
[`stackable-item`](../../.yidam/corpus/inventory-item/stackable-item.yml).

AGS had already told this corpus so, in a phrase sitting in the ledger for five phases:
`AddInventory` "adds **one quantity** of the specified item"
([ags-manual](../../.yidam/catalog/ags-manual.md)) [verified].

**And the cost is not authoring cost.** Every other primitive leaves a game's state space
finite. Quantity does not: an acquisition source that can fire twice hands out an unbounded
supply, and the analysis never terminates. Loops and recursion were excluded so that
*evaluation* terminates; this is *analysis* terminating, a different property that nothing had
been protecting. Items now declare a ceiling as counters declare a range.

## Three defects in the calculator, all found by the second specimen

A second game is worth more than a second reading of the first, and this is the evidence.

**An ending that writes nothing was filtered out as a no-op.** The ferry's ending opens a gate
and sets no state, so the search had no goal at all and reported the whole game unreachable.
The lighthouse hid it: its ending also sets a counter and spends an item, so it survived the
filter for reasons unrelated to being an ending.

**The cause classifier was reading the effect list.** A binding that spends *and* moves carries
both kinds of effect, and reading the cause off the list picks one by fiat — the ferry's toll
spends three coins and crosses a river, and the coins are not why the player is stranded. It
now takes the spending away and asks whether that rescues the state. Both specimens classify
correctly, one cause each.

**The search did not stop.** A mutation clearing the flag that guarded an acquisition source
made it unbounded, and a test hung for two minutes. That is the finding above, discovered by
running rather than by reasoning.

## Grammar coverage

Unexercised across both games: `or`, `!=`, `<=`, `>`, and grouping parentheses. `count` is no
longer on that list.

**One specimen is much better evidence that a primitive was needed than that one is not**, and
two is only a little better. Forty-odd bindings across two deliberately small games is a thin
basis for removing anything.

## The verb vocabulary

**Used and not declared: `walk-to`**, at **six** sites across the two games — four in the
lighthouse, two in the ferry. This document said eight for two phases; the number was never
counted from the files and is corrected here by counting them.

Movement is a verb in both systems this corpus has read — AGS ships Mode 0 (Walk) and its
Tumbleweed template carries `eMG_WalkTo` ([ags-manual](../../.yidam/catalog/ags-manual.md))
[verified] — and no `verb` node declared it. That was `vocabulary-coverage`'s second list, and it
had one member for three phases.

**Now assessed, and the answer is that these six sites are not verb bindings at all.**
[verb/walk-to](../../.yidam/corpus/verb/walk-to.yml) finds that no engine lets an author write a
response to walking: AGS's `unhandled_event` matrix has fallback rows for cursor modes the manual
never names and no row for Walk. [verified]

The specimens say the same thing from this side, and it is checkable rather than documentary.
**Every one of the six carries `do move <room>` and nothing else.** The verb contributed no effect
— `move` is not its own, and the ferry's toll moves the player under `use-with coin`. It
contributed a *trigger*. What these sites are writing is an **exit**, which
[`room -connects-to-> room`](../../.yidam/corpus/room.ont.yml) has declared since genesis and this
game format has no syntax for. [inference]

One of the six is the reason `connects-to` is not simply the answer:

```
    walk-to:
      when  door-open
      do    move base
      else  "Locked, and the salt has taken the hinges."
```

A guard and a refusal, on an edge between two rooms. That is a binding whose trigger is arrival
rather than a verb applied to a target — and the interaction table indexes on verb and target, so
it has nowhere to put one. `[open]`, and it is the live proposal this assessment leaves behind.

**Declared and not used: `give-to`.** No natural site arose in either game, from a direction the
documentary assessment in that node did not use.

## What the corpus has no class for

Both specimens need `start` and `ending`, which belong to the game rather than to any part of
it. Fifteen classes model rooms, verbs, bindings, flags and failures, and none models the
artifact they compose into. Noted rather than proposed — and noted more carefully after
`effect-vocabulary`, since most of what this document last called missing was not.

## Using them as fixtures

Twenty tests, in `crates/reachability/tests` and `crates/game-format/tests`.

- **lighthouse, winnable** — and **unwinnable by consumption** if the oil goes into the hinge.
- **ferry, winnable** — and **unwinnable by ordering** if the river is crossed without the
  lantern, because the boat does not come back.
- **lighthouse + a two-line mutation** for ordering, applied in memory: `do clear door-open` on
  the quay door's `walk-to` and `when door-open` on the far side's.
- **ferry + a one-line mutation** for an unbounded supply.
- **a cleared latch**, reported as contradicting its declaration.

Each committed game carries exactly one defect, and the tests assert `strandings.len() == 1`. A
fixture with two makes a checker's output ambiguous about which it found.

## What these specimens are not

- **Not evidence the language is pleasant to write.** Both were written by the party that
  proposed it, which is the weakest possible authoring trial.
- **Not a test of scale.** 103 and 20 states. The collapse against the observed space — 0.05%
  and 4% — is the argument for the whole approach, and two small games do not establish it
  holds for a large one.
- **They do not run.** No engine reads them. Every claim here is about what could be written
  down and what could be computed from it.
