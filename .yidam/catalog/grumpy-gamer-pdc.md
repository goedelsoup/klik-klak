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

**The charts themselves are images.** The Thimbleweed Park chart and the DeathSpank chart
(linked from the 2014 post) are pictures of graphs. A width could in principle be counted off
them by eye; nobody here has done that, and a number read off a JPEG by an agent would be worth
less than the prescription it was meant to check.

So the honest position is that this source supplies a **design rule**, and
`crates/reachability/tests/scaling.rs` supplies what that rule costs. What neither supplies is a
measured chart from a shipped game, which remains the retrieval that would close the question.

## What these sites hold that nobody here has looked at

The rest of both blogs: fifteen years of Grumpy Gamer, and the entire Thimbleweed Park
development diary, which is a near-daily record of an adventure game being designed by the
people who defined the form. It is the largest unread body of primary material this corpus has
identified.
