---
name: reachability
description: Whether every reachable world-state configuration can still reach the ending, and which binding stranded the ones that cannot
---

# reachability

**Status: implemented.** `crates/reachability`. Approved in `.yidam/decisions/proposals.yml`
over a stated objection; see the rationale there, the account in
`.yidam/decisions/seed-scope.yml`, and `.yidam/decisions/domain-computer.yml` for the phase
that built it.

```
cargo run -p reachability -- design/specimen/lighthouse.game
```

## What it computes

Given an authored game, whether every world-state configuration reachable by play can still
reach the ending — and where it cannot, the binding after which it cannot. This is
`failure-mode/unwinnable-state.yml` detected mechanically rather than by walking into it.

The two causes are different problems and the output distinguishes them rather than summing
them:

- **Consumption** — an object needed later is spent earlier.
- **Ordering** — a gate closes with something still needed behind it.

They are told apart by a **counterfactual**, not by which effects the stranding binding
carries. A binding that spends *and* moves carries both kinds, and reading the cause off the
effect list picks one by fiat: a ferry toll spends three coins and crosses a river, and the
coins are not why the player is stranded. So the spending is taken away and the state asked
again — rescued means consumption, still stranded means something else.

The corpus used to hold that the first was "checkable where the binding is written" and the
second was not. That is wrong and `unwinnable-state` now records the correction: a consuming
binding is locally correct and stays correct. Both causes are the same computation — a
reverse search from the endings over the reachable graph — and neither is visible at the site.

## What it reads

An authored game's state graph, derived from a `.game` file by `crates/game-format`: the
declared flags and counters, the bindings that write them, and the predicates that read them.
State is a room, a mode, per-item `unclaimed | carried | spent`, the flags, the counters, and
which options have been chosen.

**It only works because the language is bounded.** Every precondition is a finite predicate
over declared names, so the state space is enumerable and the question is decidable. A game
with a general-purpose escape hatch is not a slower input here — it is not an input at all.

## What it returns

The reachable and stranded counts, whether the ending is reachable, and one entry per
stranding binding: its line, the verb and objects, the cause, how many states it strands, and
an example of one, described in the terms the game was written in.

## Against the specimens

Two games, one defect each, one cause each.

```
$ reachability design/specimen/lighthouse.game
103 reachable state(s), 193 transition(s).
103 of 196608 combinations of the values these variables take (0.052%).
The ending is reachable.

16 reachable state(s) can no longer reach the ending.
Stranded by:
  line  132  use-with oil-can on base.hinge  [consumption]
            spends oil-can, which is needed later — 4 state(s)
            e.g. in base; carrying crowbar, brass-key, wick; lamp-prep=0
```

```
$ reachability design/specimen/ferry.game
20 reachable state(s), 34 transition(s).
20 of 480 combinations of the values these variables take (4.167%).
The ending is reachable.

2 reachable state(s) can no longer reach the ending.
Stranded by:
  line   64  use-with coin on ferryman  [ordering]
            a gate closed with something still needed behind it — 2 state(s)
            e.g. in far; carrying nothing
```

Those are the traps, found by machine, four phases after `unwinnable-state` described the
mechanisms in prose and could not express one of them.

**The second game found three defects in this tool and the first had found none.** An ending
that writes nothing was filtered out as a no-op, so a game whose ending sets no state reported
as entirely unreachable. The cause classifier read the effect list rather than asking the
counterfactual. And the search did not terminate at all once items bore quantities, because an
acquisition source that can fire twice hands out an unbounded supply. A second specimen is
worth more than a second reading of the first.

## Prelude domain

`graph-topology` was named for this in `.yidam/decisions/proposals.yml` and **is not used.**
The computation is a forward BFS and a reverse BFS over an explicit edge list, in a crate with
no dependencies at all — reaching for a vendored layer to do two breadth-first searches would
have been naming a domain because it sounds adjacent. The approval stands as a record of what
was expected; this note is what happened. Whether the domain earns its place in this
repository at all is `[open]`.

## Monotonicity, measured rather than argued

`world-flag/latched-flag` claims that a flag which only moves one way makes the reachable
state space a lattice rather than a graph, so the search walks a partial order instead of
every combination. The tool reports, per counter, whether it ever falls across a reachable
transition — `lamp-prep` does not — and reports a **latch violation** when a flag declared
`latched` is cleared anywhere. That second one is what makes `world-flag.reversible` worth
declaring: a property nothing checks is a comment.

It also reports **how many combinations exist of the values the variables were observed to
take**, beside how many are reachable. That is the collapse claim as a number: 103 of 196,608
for the lighthouse and 20 of 480 for the ferry — 0.05% and 4%. The gap is how much structure
the game has.

The claim is still not *tested* in the sense that matters. Two small games show the collapse is
large and that a declaration can be held against a game; neither shows that exploiting
monotonicity beats enumerating, because both run in a millisecond either way. That wants a game
large enough for the difference to show.

And a related check arrived from the other direction: **an item may declare a ceiling, and the
tool reports where a game hands out more than the declaration has room for.** Without ceilings
the state space is not finite at all — which is a limit on this whole computation and was found
by a test that hung, not by anyone reasoning about it.

## What it does not do

- **It does not exit nonzero on a stranded state.** A stranding is a finding about the game,
  not a failure of the tool; the specimen is deliberately trapped, and a permanently red
  fixture is a fixture somebody switches off. The caller decides.
- **It does not read the corpus.** It reads a `.game` file. Whether a computation over a
  specimen and one over corpus instances are the same thing with two inputs is `[open]` — see
  `vocabulary-coverage`, which is blocked on the same question.
- **It says nothing about whether a game is good.** Reachable and winnable is a floor.
