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

| | [`lighthouse.game`](lighthouse.game) | [`ferry.game`](ferry.game) | [`nightwatch.game`](nightwatch.game) |
|---|---|---|---|
| written for | *can this say a game at all?* | *does `count item` earn its place?* | *does `cutscene` earn its place?* |
| committed defect | consumption | ordering | consumption |
| reachable states | 103 of 49,152 (0.21%) | 20 of 120 (16.7%) | 14 of 64 (21.9%) |

Those fractions were published as 0.05%, 4% and 5.5% for several phases. They were too small by
2^(takeable objects): the metric multiplied in one bit per acquisition site for the checker's own
`taken` bookkeeping, which the item count already determines. See **Scale**, below, for how that
was found and what it cost.

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

## nightwatch — the sequence

Four rooms, two items, two latched flags, and one question: **does `cutscene` earn its place?**

[`cutscene.ont.yml`](../../.yidam/corpus/cutscene.ont.yml) seeded the class while doubting it and
named the reduction to argue against — "an interaction whose response is long and whose input is
locked". This game wants a sequence twice, in two different shapes, so the argument has something
to be about rather than being settled by taste.

**The click-triggered shape reduces, completely.** `study.case` is a whole scripted beat: the wire
goes in past the wafers, the lock turns over, the lid comes up on its own weight. It is *one
binding with one effect*. Everything that makes it a scene is in the `say`, and `say` is
presentation — [`effect-list-semantics`](../../.yidam/decisions/effect-list-semantics.yml) puts it
outside the state model and `Binding` has no field for it. [inference] So "the response is long"
is not a claim about the vocabulary. And "the input is locked" is vacuous: an atomic binding never
yields control, so there is nothing to lock.

**The arrival-triggered shape does not reduce, and what fails is the trigger.** The watchman
intercepts you for *arriving* in the hall carrying the ledger. No binding can say that: an
`interaction` site is a room and a target plus a verb, and there is no verb for "got here".

The workaround was to hang the sequence on the click that precedes each arrival — and it cost
exactly what the specimen was built to make countable:

| | as first written | with `on-enter` |
|---|---|---|
| bindings for the sequence | **2** — one per door into `hall` | 1 |
| bindings for the ordinary passage | 2 | 2 |
| copies of the sequence body | **2** | 1 |
| reachable states / transitions | 14 / 31 | 14 / 31 |

`when` gates a whole binding and `else` carries only a line, so "if the alarm is on do this,
otherwise pass through" was two bindings, not one — and that pair repeated per entrance. A room
with five doors would have carried five copies. [inference]

**The game is now written the second way**, and the last row is the point: the numbers did not
move, so the primitive is faithful to the game rather than a different game that happens to
parse. See [`arrival-trigger`](../../.yidam/decisions/arrival-trigger.yml).

**The duplication is a diagnostic cost too, which is the part that was not expected.** One
authored intent, reported once per copy:

```
  line   68  walk-to on yard.hall-door   [consumption]  spends ledger — 1 state(s)
  line  157  walk-to on study.hall-door  [consumption]  spends ledger — 1 state(s)
```

Nothing in that output says those are one sequence.

**And the copies could come apart silently.** Drifting one copy's guard — `when alarm` to
`when case-open` on the second only — genuinely changed the game: 15 reachable states instead of
14, 33 transitions instead of 31. **Every finding stayed identical.** Same two strandings, same
lines, same cause, same counts, same examples. Only the totals moved, and nothing said that a
total moving means two copies of one sequence disagree. That is
[`puzzle/state-gate`](../../.yidam/corpus/puzzle/state-gate.yml)'s complaint — "a gate depending
on three flags is stated three times, or more, and nothing checks that the statements agree" —
arriving from a direction that node did not name.

That test is gone, along with the thing it tested: there is one copy now and nothing to keep in
step. **The two strandings remain and are no longer duplication.** There are two doors into the
hall, so there are two ways for a player to make the mistake, and a report naming both is
describing the game. What changed is that neither names the sequence — the blame lands on the
door the player chose, because a forced binding is not something anybody chose.

**The verdict is in
[`cutscene/scripted-sequence`](../../.yidam/corpus/cutscene/scripted-sequence.yml).** In short:
every property the class was seeded on reduces, and what does not reduce is a trigger that
belongs to no class — the same arrival trigger
[`walk-to`](../../.yidam/corpus/verb/walk-to.yml) needed for exits two phases ago.

**What this specimen did not reach**, stated because a reduction argued from one small game is
worth exactly what it cost. The watchman moves — he crosses the hall, takes the ledger, opens the
far door — and *all of that is prose in a `say`*. Nothing in the game depends on where he is, so
this specimen never tested whether an NPC's position needs to be state, which is the one thing a
real cutscene does that nothing here does. And `cutscene.trigger`'s third case, a sequence
beginning because a counter reached a value, is untested here and undocumented in either engine.

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

Thirty-eight tests, in `crates/reachability/tests` and `crates/game-format/tests`.

- **lighthouse, winnable** — and **unwinnable by consumption** if the oil goes into the hinge.
- **ferry, winnable** — and **unwinnable by ordering** if the river is crossed without the
  lantern, because the boat does not come back.
- **nightwatch, winnable** — and **unwinnable by consumption** if the hall is entered carrying
  the ledger, reported once per door because there are two ways in.
- **two rooms that throw each other**, reported as a chain that never settles rather than
  chased — the decidability guard `on-enter` needed.
- **lighthouse + a two-line mutation** for ordering, applied in memory: `do clear door-open` on
  the quay door's `walk-to` and `when door-open` on the far side's.
- **ferry + a one-line deletion** for an unbounded supply — an omission rather than an edit,
  since `effect-list-semantics` made the old set-then-clear version a dead write.
- **a cleared latch**, reported as contradicting its declaration — cleared from a *different*
  binding, because within one list it is a dead write and no latch is broken.

The first two carry exactly one defect and assert `strandings.len() == 1`; a fixture with two
makes a checker's output ambiguous about which it found.

**`nightwatch` asserts two, and it means something different now than when it was written.** It
used to mean "one authored defect, reported once per copy of a duplicated sequence". It means
"one authored sequence, and two doors a player can walk through to reach it". The number is the
same and the reason is not, which is worth noticing: an assertion that survives a change to what
it is about was not testing what it looked like it was testing.

## Scale — the claim these three could not test, tested

Three games small enough to enumerate by hand cannot say whether enumeration scales, and this
document said so for six phases. `crates/reachability/tests/scaling.rs` **generates** the large
games instead. That costs realism and buys a curve: one game is one point, and tractability is a
question about growth.

Two families, both legitimate adventure-game shapes.

| | **corridor** — door *i* needs the key behind door *i-1* | **hub** — *n* tokens, no order, all wanted at one gate |
|---|---|---|
| reachable states | 3n+1 | **2ⁿ** |
| fraction of declared space | → 0 | **100.000%, at every size** |
| n = 16 | 49 states | 65,536 states |
| n = 22 | — | 4.2 million states, 36 seconds |
| n = 128 | 385 states, 16 ms | — |

**The collapse is a property of the game's dependency structure, not of the language.** Same
language, same checker; the difference is entirely whether the author ordered the puzzles. And a
hub is not a contrived shape — *find the three map pieces*, *collect the four keys*.

**The sharper half: both families are made entirely of monotone variables.** Nothing is ever
unset in either. So monotonicity is not what makes the search cheap —
[`latched-flag`](../../.yidam/corpus/world-flag/latched-flag.yml) had said it was, and that claim
is withdrawn. Monotonicity bounds the *path*, not the size of the set, and a lattice over *n*
independent monotone variables is every combination.

**And the metric could not have found this.** Counting `taken` gave the hub 2ⁿ states against a
reported 4ⁿ space — a "collapse" to 2⁻ⁿ that was pure redundancy, and which looked exactly like
the corridor's. The old number reported ~0% for a linear family and an exponential one alike;
the corrected one separates them completely. A number that measures the representation rather
than the game is worse than no number, because it looks like evidence.

**What survives.** Decidability is untouched — the space is finite and the search terminates,
which is what a bounded predicate layer buys and what no escape hatch can offer.

### And the narrower question has an answer

*How ordered are a real game's puzzles?* Ron Gilbert invented the notation for arranging them —
the Puzzle Dependency Chart — and states its shape as a rule: "Solving one puzzle should open up
2 or 3 new ones", each act ending in "a bottle neck to the next act", with **both** extremes
named as failures, "too linear, or too unwieldy with choice (also bad)"
([grumpy-gamer-pdc](../../.yidam/catalog/grumpy-gamer-pdc.md)). [verified]

A third generated family is that shape — *k* acts of *w* parallel puzzles, each act gated on all
of them. Holding the puzzle count at forty-eight and varying only the width:

| acts × width | reachable states |
|---|---|
| 48 × 1 | 97 |
| 24 × 2 | 97 |
| **16 × 3** | **129** |
| 12 × 4 | 193 |
| 8 × 6 | 513 |
| 6 × 8 | 1,537 |
| 4 × 12 | 16,385 |
| 3 × 16 | 196,609 |

Same forty-eight puzzles, a factor of two thousand. **The reachable set is linear in the number
of puzzles and exponential in the chart's width.** At the prescribed width the count stops
mattering: six hundred puzzles in acts of three is 1,601 states and 73 milliseconds — larger than
any published adventure game, and cheaper to check than the lighthouse.

So the checker is tractable exactly on games whose charts are the shape their designers already
aim for. That is contingent where the collapse claim was pretending not to be, and it is a
better position for being so. [inference]

What is still missing is a **measured** width from a shipped game. Gilbert prescribes 2–3, and
reports 4–5 for the unfinished boxes in Thimbleweed Park's chart; neither is a count off a
finished one, and he does not have Monkey Island's. [open]

## What these specimens are not

- **Not evidence the language is pleasant to write.** All three were written by the party that
  proposed it, which is the weakest possible authoring trial.
- **Not a test of scale, and the test of scale has since been run without them.** See below.
- **They do not run.** No engine reads them. Every claim here is about what could be written
  down and what could be computed from it.
