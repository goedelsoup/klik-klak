---
name: Three Sierra AGI games — logic source of Police Quest, King's Quest III and the first Leisure Suit Larry
description: The AGI logic source of three 1986–87 Sierra adventures by three different lead designers, one of them Al Lowe. Read to tell one designer's shape from the house's, against Leisure Suit Larry 5.
type: dataset
obtained: true
retrieved: 2026-11-27
ttl_days: 1825
location:
  - kind: url
    value: https://github.com/historicalsource/police-quest-1
    description: "Read at commit 16fdc5439e0d863ea3031626ac2a2ec066c85700, last pushed 2022-12-25. SRC/ holds 119 .CG logic files, 54,144 lines, with GAMEDEFS and FLAG_VAR.H"
  - kind: url
    value: https://github.com/historicalsource/kings-quest-3
    description: "Read at commit 15e158ecbcd7754e494c811d397cf1b2378b1f49, last pushed 2022-12-25. SRC/ holds 124 .CG logic files, 43,522 lines, with GAMEDEFS.H and per-programmer headers"
  - kind: url
    value: https://github.com/historicalsource/leisure-suit-larry-1
    description: "The original AGI game, not the SCI remake. Read at commit fb619fd0af37c3f846c162f5ae1185e4b0e1e729, last pushed 2022-12-25. SRC/ holds 46 .CG logic files, 26,516 lines, with GAMEDEFS.H and FLAG_VAR.H"
---

**Three sources of record, chosen to break one confound.** [lsl5-source](lsl5-source.md) measured
one game by one designer on one engine. A shape found there could be Al Lowe's, SCI's or Sierra's.
These three are on the engine before SCI, AGI ([agi-specifications](agi-specifications.md)). Two
of them are by other lead designers, and one is Lowe's own game from four years earlier. Holding
the engine fixed, they compare designers. Holding the designer fixed, Lowe's two games compare
engines and years. `.yidam/decisions/house-width.yml` is the record of reading them.

## The licence, stated before anything else

**None of the three repositories carries a licence.** Each README is a single heading. The rule is
[lsl5-source](lsl5-source.md)'s. The clones stayed in a scratch directory outside this repository
and nothing from them was committed. This entry reports figures, file-and-line citations and
quotations of a few words. No logic, message text or header text is reproduced.

## Who wrote what, as far as the source says

- **Leisure Suit Larry** (1987). `RM0.CG`'s header credits the game to Al Lowe. [verified]
- **Police Quest** (1987). The `GAMEDEFS` header lists five programmers: Greg Rowland, Jim
  Walls, Scott Murphy, Ken Williams and Al Lowe. [verified] Room headers name Rowland, Murphy and
  Walls, and some add Lowe. [verified] Walls as the designer is the usual credit and is not
  stated in these files. [inference]
- **King's Quest III** (1986). The source does not name a designer in what was read. `GAMEDEFS.H`
  splits flag numbers among programmers, with "Al's flags" at 90–169 and "Bob's" at 170–219.
  There are separate `.AL` headers. [verified] The header's staff list includes Al Lowe, so "Al"
  is very likely him. [inference] That header also lists the staff's private phone numbers and
  addresses. This entry does not repeat them and nothing derived from this source should. Roberta
  Williams as the designer is the usual credit. [inference]

**So Lowe's hand is in all three, and the comparison is designer against designer only at the
level of design.** He programmed parts of the other two games. A shape shared by all four games
therefore cannot separate "Lowe" from "Sierra" cleanly. It can only separate the lead designer's
choices from everything else. [inference]

## What AGI logic looks like to the extractor

A `.CG` file is one room's or one module's logic in Sierra's own source language, before
compilation. This is not the reconstructed LOGIC language of the specification.
[verified] The things the extractor relies on:

- **State is numbered.** `%flag`, `%var` and `%object` bind names to numbers in the headers.
  Numbers 0–29 belong to the interpreter in all three games, as their headers state. [verified]
  The score is var 3 (`score` in two games, `currentScore` in Larry). [verified]
- **Room-local state is a number range, not a scope.** Police Quest and King's Quest III start
  room-local and dynamic flags and vars at 220. Larry's `FLAG_VAR.H` declares them from 200, though
  a comment in `GAMEDEFS.H` gives 220. The extractor follows the declarations. [verified]
- **An award is a statement.** `score += n` (or `currentScore += n`) adds points with no flag
  naming the step. So **a step is an award site**, and every step has exactly one awarding unit.
  The maxima are set in code: 245 for Police Quest, 210 for King's Quest III, 222 for Larry.
  [verified]
- **A parser binding is an `if` on `said(...)`.** A unit is a said-binding with everything
  under it. Each other `if` branch is a unit holding its own statements, reading its own condition
  and every condition above it. A file's loose top-level statements are one more unit. [verified]

## Width in scored steps

The method is the one [lsl5-source](lsl5-source.md) used. Edges run read-after-write between
units. Only acquisition counts as a write, so `-=`, decrement, `drop` and `put` give no edge.
The graph is projected onto steps: one step is before another when its awarding unit reaches the
other's. Width is the largest antichain on the condensation, and chain is the longest path.

**One reading the SCI source did not need.** AGI rooms run their animations and timed sequences
through a few shared vars: `script`, `work`, `scriptDone`, `ego.x` and the like. They are written
in 20 to 80 files. [verified] SCI keeps the same state on each Script object, where the LSL5
extractor never saw it. Counted as global here, these names join most of each game into one
cycle: 78 of Police Quest's 111 steps, 43 of King's Quest III's 67. [verified] So a name written
in at least *k* files is scoped to its own room, and the figures are given for *k* = 16, 8 and 4.
This choice is the measurement's, not the game's. [inference]

| | steps | readings | width | chain | largest cycle | ordered with nothing |
|---|---|---|---|---|---|---|
| Police Quest | 111 | k=16, 8, 4; locals kept | 41, 42, 48 | 4, 4, 9 | 61, 56, 11 | 12, 12, 18 |
| | | k=8, 4; locals removed | 53, 74 | 6, 5 | 29, 5 | 22, 44 |
| King's Quest III | 67 | k=16, 8, 4; locals kept | 31, 39, 50 | 5, 5, 3 | 21, 14, 6 | 11, 17, 28 |
| | | k=8, 4; locals removed | 56, 64 | 3, 2 | 2, 2 | 45, 61 |
| Leisure Suit Larry | 49 | k=16, 8, 4; locals kept | 25, 25, 25 | 6, 6, 8 | 9, 9, 3 | 5, 5, 6 |
| | | k=8, 4; locals removed | 37, 37 | 4, 4 | none | 28, 28 |
| *LSL5, whole game* | 124 | every name / locals and routing removed | 49 / 67 | 6 / 5 | 28 / 9 | 22 / 28 |

All figures are `[verified]`: they are counts from the pinned commits by the method above. The
LSL5 row reruns the same projection on [lsl5-source](lsl5-source.md)'s units over the whole game,
not by act.

**Width as a share of steps** is 0.37–0.67 for Police Quest, 0.46–0.96 for King's Quest III,
0.51–0.76 for Larry, and 0.40–0.54 for LSL5. **The longest chain is 2 to 9 in every game.**
[verified] All four games are wide and shallow in their scored steps, and LSL5 is at the narrow end
of the range. [inference]

## Acts

**Bottleneck steps do not find acts in any of the four games.** A test for steps ordered with every
other non-isolated step found none in LSL5 either, on either reading. [verified] LSL5's eight acts
were found from its routing names (`larryLoc`, the been-in flags) and the code that closes a
city, not from its scored steps. So this test cannot say that a game has no acts. [inference]

**Police Quest declares an act counter, and the other two do not.** `GAMEDEFS` declares var 31
`progress`. The logic sets it to named stages that only increase: the briefings, a narcotics case
from 4, an undercover case from 100, two poker games in the 160s–190s, and 200 at the end. It is
set in 14 files and tested in 26. [verified] Neither King's Quest III nor Larry declares a var
whose name refers to progress, an act, a chapter, a stage, a part, a day or a section. [verified]
That a name search finds nothing is weak evidence that the game has no sequence. [inference]

**Lowe's 1987 game reads as one open act.** Larry has no act counter. [verified] Nothing found
so far divides its steps, though that was looked for only by name. [inference] Its whole-game width, 25 to 37, is larger than any LSL5 act, whose largest is
11. [verified] Read as one act, it is far wider than the 2–5 the tractability argument assumes,
and its cost of 2^25 or more is the size that argument rules out. [inference]

## Police Quest by region

`.yidam/decisions/pq-acts.yml` tried to cut Police Quest's steps by its `progress` stages, as LSL5's
were cut by city. **The stages do not reach the steps.** Only 3 of the 111 award sites sit under a
condition on `progress`: one in a briefing room and one in each poker room. [verified] The stage
var decides which scene a room sets up, not which awards it offers. So the cut was made by room
instead, with the regions named in `ROOMNUMS.H`, and each region's steps were measured on the
whole game's order.

| region | rooms | steps | width, k=16, 8, 4, locals kept | width, k=8, 4, locals removed | chain |
|---|---|---|---|---|---|
| street and squad car | maps 10–25, the three traffic stops, the cafe, the biker bar, the murder scene | 25 | 15, 16, 17 | 18, 22 | 2–3 |
| station | halls, offices, locker room, briefings, narcotics office | 33 | 8, 9, 16 | 14, 26 | 3–5 |
| park and Cotton Cove | 56, 57, 60 | 14 | 4, 4, 4 | 6, 9 | 2–4 |
| hotel | 63–71, lobby to the card room | 10 | 4, 5, 5 | 7, 7 | 3–4 |
| jail and court | 40–41, 53–55 and the jail logics | 10 | 2, 2, 3 | 3, 4 | 2–4 |
| Blue Room | 33, 34 | 3 | 2, 2, 2 | 2, 3 | 1–2 |
| other logics | 0, 38, 79, 94, 103, 109, 110, 112 | 16 | 7, 7, 9 | 12, 15 | 2–3 |

All figures are `[verified]` counts by the method above. **Only the hotel is a stage as well as a
region.** Its rooms are the ones that move `progress` through the values from 101 to 190, the
undercover case and both poker games. [verified] The street is where the uniformed patrol
happens, and the squad car's room tests `progress` against the second briefing. That makes it the
patrol stage. [inference] The station is visited in every stage, and nothing in the source divides
its steps among them. [verified]

## What this does not say

- **Police Quest's acts were measured by region, not by stage.** The stage var guards three
  awards, so a cut by stage would have needed the order in which the dispatcher and the timers
  send the player to each room, and that order is carried by timers this extractor cannot see.
- **The scratch threshold moves every figure.** Police Quest's width runs from 41 to 74 across
  readings. The table gives the range, and no single value is offered as the game's width.
- **AGI's units are finer than SCI's.** One `if` branch is a unit, so there are 2 to 4 times as
  many units per step as in LSL5, and edge counts are not comparable. Widths in steps are the
  figure meant to survive the change of engine. [inference]
- **A dependency carried by position, a timer or a room exit is invisible,** as in LSL5. Missing
  edges widen a game, and the surviving cycles narrow it. [inference]

## What these repositories hold that nobody here has looked at

Everything outside `SRC/*.CG` and the headers the extractor reads. Police Quest's `Fastback Backup`
and Larry's are dated archives of the working tree and were not opened. The `.MSG` files, which hold
each room's text, were read only where a binding was checked by hand. The picture, view, sound and
vocabulary resources were not opened. Police Quest's acts, the next reading `house-width` named,
have since been read, by region rather than by stage, in `pq-acts`; see above. The dispatcher in
rooms 96 and 119, which decides which call the patrol answers next, was not read.
