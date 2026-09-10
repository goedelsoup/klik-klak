---
name: Ron Gilbert on Puzzle Dependency Charts
description: The designer of Maniac Mansion and Monkey Island describing the tool he invented for arranging an adventure game's puzzles, and what shape a good one has.
type: other
obtained: true
retrieved: 2026-09-10
ttl_days: 1825
location:
  - kind: url
    value: https://grumpygamer.com/puzzle_dependency_charts/
    description: "\"Puzzle Dependency Charts\", 10 August 2014 — the primary text"
  - kind: url
    value: https://blog.thimbleweedpark.com/act_123_puzzles.html
    description: "\"Act 1, 2 and 3 Combined Puzzles\", 20 April 2015 — the chart for a game he then shipped"
used-by:
  - ../corpus/puzzle/state-gate.yml
  - ../corpus/world-flag/latched-flag.yml
---

**A source of record, and about a narrower thing than it first appears.** Ron Gilbert designed
Maniac Mansion and The Secret of Monkey Island, invented the Puzzle Dependency Chart, and these
are his own posts on his own sites. For *what the technique is and how he uses it* there is no
closer source and no reason to want one — claims about that may be `[verified]`.

For **what shipped adventure games actually look like**, this is a designer's prescription and
recollection, not a measurement of any game. He says so himself: "I don't have the Puzzle
Dependency Chart for Monkey Island, or I'd post it." Claims of that kind stay `[inference]`, and
the distinction is load-bearing here, because this corpus reached for these posts wanting a
number about real games and got a design rule instead.

This is also the first source here that is neither an engine's documentation nor a reverse
engineering of one. The corpus had read three systems' manuals and never the people who wrote
games in them.

## What was read

**`puzzle_dependency_charts` (2014).** What the tool is: "A Puzzle Dependency Chart is a list of
all the puzzles and steps for solving a puzzle in an adventure game. They are presented in the
form of a Graph with each node connecting to the puzzle or puzzle steps that are need to get
there." Its provenance: invented "probably right before or during the development of The Last
Crusade adventure game", with David Fox and Noah Falstein contributing, and "They reached their
full potential during Monkey Island where I relied on them for every aspect of the puzzle
design."

**The three passages this corpus came for**, and they are about the *shape* of a good chart:

> "There is nothing (NOTHING!) worse than linear adventure games and these charts are a quick
> visual way to see where the design gets too linear or **too unwieldy with choice (also bad)**."

> "Solving one puzzle should open up 2 or 3 new ones, and then those collapses down (but not
> necessarily at the same rate) to a single solution that then opens up more non-linear puzzles."

> "I tend to design adventures games in "acts", where each act ends with a bottle neck to the
> next act. I like doing this because it gives players a sense of completion, and they can also
> file a bunch of knowledge away and (if need) the inventory can be culled)."

Read together those are a **bounded width and a bottleneck**, stated as an aesthetic rule by the
person who invented the notation. Both extremes are named as failures: too linear, and too
unwieldy with choice.

There is also a negative case in his own work: "Gary and I didn't have Puzzle Dependency Charts
for Maniac Mansion, and in a lot of ways it really shows. The game is full of dead end puzzles
and the flow is uneven and gets bottlenecked too much."

**`act_123_puzzles` (2015).** The combined Act 1–3 chart for Thimbleweed Park, which shipped in
2017 — so this is the tool applied to a real game rather than described. The usable line is a
second, independent figure for the same quantity: "Most of the red boxes will explode into 4 or
5 new boxes, many with parallel puzzles to solve."

## What was looked for and not found — read this before citing

**No number describing any finished chart.** Not a puzzle count, not a width, not a count of
acts, not how many puzzles are open at once. The 2014 post prescribes "2 or 3"; the 2015 post
says an unfinished box becomes "4 or 5". Neither is a measurement of a completed game, and this
entry was read twice looking for one.

**The charts themselves are images, and that is not the obstacle.** The Thimbleweed Park chart
and the DeathSpank chart (linked from the 2014 post) are pictures of graphs, and this entry
recorded that a width could in principle be counted off them by eye. It cannot. The development
diary states why: "Everything is blurred out, so there are no spoilers"
([twp-dev-diary](twp-dev-diary.md)). The published charts were **produced so the boxes cannot be
read**, which is a stronger and more permanent obstacle than a JPEG's resolution, and it applies
to every chart of a shipped game either author has posted.

So the honest position is that this source supplies a **design rule**, and
`crates/reachability/tests/scaling.rs` supplies what that rule costs.

**What neither supplies is a measured chart — and that turned out not to be a retrieval.** A
dependency chart is a graph of what must happen before what, which is a read-after-write relation
over game state, and [delores-source](delores-source.md) is a game this author shipped whose source
this corpus already held. Drawn from it: a step that unblocks anything opens **2.45 on average,
median 2**, which is this page's prescription measured — and **43% open exactly one**, which is not.
The acts-with-bottlenecks half of the rule is not exhibited there at all: two gated exits in twenty
and no layer that narrows. `.yidam/decisions/measured-width.yml`.

The blurred charts are still blurred. They are no longer the only way to the number.

## What these sites hold that nobody here has looked at

**The Thimbleweed Park development diary has since been read, in part**, and has its own entry
([twp-dev-diary](twp-dev-diary.md)) — twenty-two posts of 258, aimed at the engine's authoring
surface rather than at the charts. It did not supply a measured width and it explained why one
cannot be had from these sources.

What is still unread is fifteen years of Grumpy Gamer, and 236 of the diary's posts. One thing
named there and not read is worth more than the rest together: the essay these design rules are
a restatement of. "It will follow my rules of adventure game design" is a reference to *Why
Adventure Games Suck*, Gilbert's 1989 piece, which is the document the prescription in this
entry descends from and which nobody here has retrieved.
