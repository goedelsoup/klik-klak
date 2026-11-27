---
name: Leisure Suit Larry 5 — game source and the designer's own documents
description: The SCI script source of a full-length, eight-act Sierra adventure from 1991, with its design document and points list — the second game this corpus can count, and the first long enough to have acts.
type: dataset
obtained: true
retrieved: 2026-11-26
ttl_days: 1825
location:
  - kind: url
    value: https://github.com/historicalsource/leisure-suit-larry-5
    description: "The repository; read at commit 341bc91b5529c3243220a7fe77cc6b041c8022d4, last pushed 2022-12-25. SRC/ holds 136 .SC script files, 83,600 lines; DOCO/ holds DESIGN.DOC (dated Feb 25 1991), HINTBOOK.DOC and POINTS.TXT"
---

**A source of record, and the second game this corpus can *count*.** *Leisure Suit Larry 5:
Passionate Patti Does a Little Undercover Work* shipped in 1991 on Sierra's SCI engine
([sci-specifications](sci-specifications.md), [sci-inhouse-docs](sci-inhouse-docs.md)). The
repository holds the game's script source, the files a Sierra programmer edited, and three of the
designer's own documents. The script headers read "Copyright 1991 by Al Lowe Associates Inc."

It was chosen for one property [delores-source](delores-source.md) lacks: **it is full length, and
it is cut into acts.** `.yidam/decisions/measured-width.yml` measured Delores and could not tell
whether its missing bottlenecks belong to games or to small games. This is the game that tells them
apart. `.yidam/decisions/second-game-width.yml` is the record of reading it.

## The licence, stated before anything else

**The repository carries no licence.** Its README is one heading and nothing else. Copyright in the
game presumably rests with whoever holds Sierra's catalogue now, and nothing in the repository
grants anybody anything. [inference]

So this entry is held to a narrower rule than the open-source entries beside it. **The source was
cloned into a scratch directory outside this repository and never committed here. This entry
reports figures computed from it, file-and-line citations into it, and quotations of a few words.**
No script, no dialogue and no document text is reproduced. A reader who wants to check a figure
clones the pinned commit and counts.

## What was measured, and how

The method is [delores-source](delores-source.md)'s, carried to a second language. **A unit is a
step an author wrote; an edge runs from a unit that writes a game-state name to a unit that reads
it.** Read-after-write is what a Puzzle Dependency Chart draws, so this graph is a chart nobody
blurred. [inference]

- **Units.** Each case of a `doVerb` method's `(switch theVerb …)` is a binding. Every other method
  outside a `Script` is a unit too. A `Script` instance is folded into whatever unit starts it,
  transitively; the eight that nothing starts are units of their own.
- **State names.** The flags (`f…`), Patti's flags (`pf…`) and inventory items (`i…`) are declared
  in the game's two headers, and twenty game-state globals besides. Room-local variables are a
  separate filter, below.
- **Writes and reads.** `Bset`/`Bclr` on a flag and the third argument of `Points` are writes. An
  assignment operator is a write, and a read too unless it is plain `=`. `get:`/`put:` on an item,
  and a setter sent to `(Inventory at: …)`, are writes. Every other mention is a read.

**Calibration** — four figures a recount should reproduce. 2001 units, of which 1262 are bindings,
across 125 files. 438 verb switches seen, against 442 that a text search for `(switch theVerb`
finds; the four missed are in forms the reader does not parse. 54 flags, 105 Patti flags, 37 items.
Bindings by verb: `verbDo` 357, `verbLook` 235, `verbUse` 101, `verbTalk` 82, `verbZipper` 31,
`verbWalk` 27, and an `else` arm 429 times. [verified]

## What it measured

**The whole game, every name, measured as Delores was:** 1694 edges over 234 names, 398 units in
the graph, **a longest chain of 7 and a maximum antichain of 191**. Layers by longest-path depth:
99, 43, 18, 3, 90, 30, 4. Out-degree where a step unblocks anything: mean 7.21, median 4, max 53;
**31% of unblocking steps open exactly one thing**. Dropping edges internal to one object: 370
units, 1579 edges, chain 7, antichain 176. [verified]

| Filter | Edges | Units | Chain | Antichain |
|---|---|---|---|---|
| every name | 1694 | 398 | 7 | 191 |
| no room-local variables | 1386 | 285 | 6 | 140 |
| flags and inventory only | 1101 | 243 | 6 | 109 |
| act sequencing removed | 1336 | 372 | 7 | 172 |
| both removed | 1005 | 251 | 5 | 115 |

"Act sequencing" is the names that route the player between acts rather than gate a puzzle:
`larryLoc`, `pattiLoc`, `destination`, `pattiDest`, `fCalledLimo`, `REMState`, `meanWhiles`,
`pattiDream`, `egoIsLarry`, and the `fBeenIn…` flags.

**By act.** Rooms were assigned to the eight act regions — Hollywood, FBI, New York, Atlantic City,
Miami, Baltimore, Philadelphia, Washington — and to three hubs that every act passes through: the
airport, the limousine and the aeroplane. Antichain per act, measured inside the whole-game
closure: [verified]

| Act | every name | no locals | both removed | on the act's own edges, both removed |
|---|---|---|---|---|
| Hollywood | 35 | 13 | 13 | 13 |
| FBI | 8 | 8 | 8 | 8 |
| New York | 22 | 15 | 14 | 14 |
| Atlantic City | 21 | 13 | 13 | 12 |
| Miami | 8 | 6 | 7 | 4 |
| Baltimore | 17 | 21 | 20 | 25 |
| Philadelphia | 14 | 11 | 11 | 13 |
| Washington | 3 | 3 | 3 | 2 |
| **sum** | **128** | **90** | **89** | **91** |

Longest chain inside an act: 2 to 6. The hubs add 33 (airport), 10 (limousine) and 5 (aeroplane)
on every name. **The sum over every region is 198 on every name and 148 without locals, against
whole-game antichains of 191 and 140.** [verified] The whole-game figure is the act widths added
together, to within a few units.

**Edges between act regions are almost absent.** Hollywood to New York carries one item over two
edges; Hollywood and Atlantic City share one; FBI to Baltimore is one item over three edges; FBI and
Washington share one. New York and Miami are joined by `fCalledLimo` and `larryLoc`, and
`fCalledLimo` is one flag reused for every city's limousine — set where the limousine is called
(`280.SC:1706–1788`, `510.SC:316`, `905.SC:941`) and cleared on arrival (`200.SC:1465`,
`900.SC:118`). There is no edge at all between Baltimore and Philadelphia, or among New York,
Atlantic City and Miami beyond that flag. [verified]

## The act gates, read in the source

The acts are not an inference from the graph. They are enforced, and the code that enforces them
was read: [verified]

- **The way out of a city closes behind you.** The limousine's `init` sets the city's `fBeenIn…`
  flag from the room the player arrived from (`200.SC:87–112`). The airport's ticket machine lists
  only cities whose flag is still clear (`258.SC:512–552`), and the limousine refuses a city already
  done, three times in three spellings of "no need to return" (`200.SC:1796`, `1831`, `1864`).
- **The finale opens on a conjunction.** When both Patti cities are done, Patti's destination
  becomes Washington (`200.SC:239–242`).
- **The protagonist is decided by the room.** Entering a room of Larry's or of Patti's releases the
  inventory and switches the player character (`000.SC:703–726`), so the two halves of the game
  cannot share a held item.
- **The halves alternate, and a dream is the switch.** Larry's dream on the aeroplane
  (`320.SC:180–234`, `410–440`) dispatches on `REMState` and on Patti's destination, and each dream
  ends by moving the game to a Patti room (`320.SC:1024`, `1570`, `1974`, `2094`).

So the game is a strict alternation of a Larry act and a Patti act, each act's exits close when it
is left, and the last act opens when two earlier ones are both closed.

## The designer's own count

`DOCO/POINTS.TXT` lists 126 scored steps. By region: Hollywood 12, FBI 8, New York 15, Atlantic
City 13, Miami 9, Baltimore 13, Philadelphia 17, Washington 1; the airport 15, the limousine 3, the
aeroplane 2; and 18 attached to the inventory, the battery charger and Patti rather than to a room.
The two finales score 100 each, at `rm390` and `rm460`. [verified] **That is an upper bound on an
act's width in the designer's own units — puzzles rather than bindings — of 1 to 17.**

## Width in scored steps

`phase/puzzle-width` turned that bound into a width. **A step is a `Points` award, and it is
ordered before another step when some unit that awards it reaches some unit that awards the other
in the unit graph above.** The source makes 139 `Points` calls, which award 124 distinct steps.
The unit graph is the one measured above, and a recount reproduces its 2001 units and 1694 edges
before anything is changed. [verified]

**One change was needed first, and it is about direction.** In the graph above, an item is written
when it is taken and again when it is given up, and a counter is written when it rises and again
when it falls. So a step that spends a quarter and a step that wins one each reach the other, and
the cycles through money and inventory hold up to 12 of an act's 15 steps. Counting only
acquisition as a write — `get:`, `++` and `+=`, and not `put:`, `--` or `-=` — leaves 1495 edges
and draws the graph in the get-it-then-use-it direction a dependency chart draws. The largest cycle
left inside an act holds 9 awarding units, in Atlantic City, carried by silver dollars and the
rollerblades: winning and spending at the casino. [verified]

Width of each act, in steps, over four readings. A step counts in the act whose rooms award it.
"Some" orders a step after another when any of its awarding units is reached. "Every" orders it
only when all of them are. [verified]

| Act | steps | every name, some | every name, every | locals and routing removed, some | removed, every |
|---|---|---|---|---|---|
| Hollywood | 11 | 9 | 9 | 9 | 9 |
| FBI | 6 | 4 | 4 | 4 | 4 |
| New York | 15 | 7 | 7 | 8 | 8 |
| Atlantic City | 13 | 5 | 5 | 5 | 5 |
| Miami | 9 | 4 | 5 | 4 | 5 |
| Baltimore | 13 | 3 | 9 | 10 | 11 |
| Philadelphia | 17 | 5 | 5 | 7 | 7 |
| Washington | 1 | 1 | 1 | 1 | 1 |
| **sum** | **85** | **38** | **45** | **48** | **50** |

The longest chain of steps inside an act is 2 to 4. Five of Hollywood's eleven steps are ordered
with no other step in the game: they read no game state and enable nothing. **Summing 2^w over the
eight acts gives 746, 1,266, 1,986 and 3,026.** [verified]

**The hint book was opened for this, and it is not a chart.** `DOCO/HINTBOOK.DOC` is 131
questions, grouped under eleven numbered places in two halves, Larry's and Patti's. Each place
lists its questions with graded answers, and no question says what it depends on. By act: Hollywood
8, New York 12, Atlantic City 27, Miami 12, the FBI 5, Philadelphia 13, Baltimore 10 and Washington
2, with 18 for the limousine and airports in Larry's half and 7 in Patti's. [verified] A question is
a place a player got stuck, and some are jokes, so this is a count of stuck points by act, not a
width.

## The wide acts by room

`phase/latched-close` cut the four acts wider than 5 by the room that awards each step, on the same
order and the same four readings. A room number below is the room's script. [verified]

| Act | steps | act's width | rooms | widest room | rooms' widths summed | sum of 2^w |
|---|---|---|---|---|---|---|
| Hollywood | 11 | 9 | 4 | 4 (room 170, 5 steps) | 10 | 30 |
| New York | 15 | 7–8 | 5 | 6 (room 535, 6 steps) | 10–11 | 72–74 |
| Baltimore | 13 | 3–11 | 3 | 1–5 (rooms 640 and 660) | 3–11 | 6–66 |
| Philadelphia | 17 | 5–7 | 4 | 2–4 (room 890) | 5–7 | 10–22 |

**No room in a wide act is wider than 6.** [verified] Baltimore's and Philadelphia's widths are the
sum of their rooms'. Hollywood's and New York's are one to four less, so a few steps in different
rooms are ordered. [verified]

**The whole game cuts the same way.** Sixteen of the 124 steps are awarded outside any room, 13 in
the inventory script and 3 in the camcorder charger's, and as one group they are 15 wide. Cut by
the item that awards them, none is wider than 1. Over the whole game, by room and by item, that is
56 parts, none wider than 6, and a sum of 2^w of 214 to 296, against a whole-game width of 49 to
70. The parts' widths add up to 75 to 90, so the parts are ordered across the cut. [verified]

## What this does not say — read this before citing

- **Units over-count puzzles.** A binding that reads a flag to choose a reply is a unit, and so is a
  look at an object. The scored-steps count is the correction in the designer's own units, and it
  is an upper bound rather than a width.
- **Cycles collapse units.** Money, quarters, credit cards and the gold card are written and read in
  the hubs and in several acts, and a cycle through them is one node when the antichain is counted.
  The largest strongly connected component is 71 units, 24 of them in Atlantic City, where the
  casino is a loop by design. Some per-act widths are therefore lower bounds. [verified]
- **Gated exits were not counted exit by exit.** Delores's "2 of 20" was a count of exits. Here the
  act gates were found and read in the source instead, which says more about structure and gives no
  comparable ratio.
- **A dependency carried by anything but a named variable is invisible**, as it was in Delores —
  an actor's position, a timer, a cel.
- **The width in steps is an upper bound one way and a lower bound the other.** An invisible
  dependency leaves two steps unordered that a player must do in order, which widens an act. A cycle
  that survives the acquisition rule makes its steps one node, which narrows it. Baltimore moves from
  3 to 11 between readings for the second reason. A scored step is the designer's unit, and it
  includes looks and drinks of water that no chart would draw.

## What the repository holds that nobody here has looked at

`DOCO/DESIGN.DOC` was opened for its date and not otherwise studied. It is
the designer's statement of the game before it was built, and a comparison of its act list with the
shipped gates is the obvious next reading. `DOCO/HINTBOOK.DOC` was expected to be a dependency chart
written for players. It was opened in `phase/puzzle-width` and is not one; see above. The script
source beyond what the extractor parsed, and the four verb switches it missed, were not studied by
hand.

The same account publishes the source of other Sierra adventures — the first three Leisure Suit
Larry games and the remake of the first, King's Quest III, Police Quest and The Black Cauldron.
None was opened. Each is a further game to measure by the same method, and the SCI ones would test
whether the act structure here is one designer's or the engine's. Three of them — Police Quest, King's
Quest III and the first Larry on AGI — have since been read, in
[sierra-agi-source](sierra-agi-source.md). The SCI games and The Black Cauldron have not.
