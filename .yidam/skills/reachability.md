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

- **Consumption** — an object needed later is spent earlier. Detected as a stranding
  transition carrying a `spend` effect.
- **Ordering** — a gate closes with something still needed behind it. Any other stranding
  transition.

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

## Against the specimen

`design/specimen/lighthouse.game`, which carries one deliberate reachable defect:

```
103 reachable state(s), 193 transition(s).
The ending is reachable.

16 reachable state(s) can no longer reach the ending.
Stranded by:
  line  125  use-with oil-can on hinge  [consumption]
            spends oil-can, which is needed later — 4 state(s)
            e.g. in base; carrying crowbar, brass-key, wick; spent oil-can; lamp-prep=0
```

That is the trap, found by machine, four phases after `unwinnable-state` described the
mechanism in prose and could not express it.

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

The claim is not yet *tested* in the sense that matters. One specimen with one latching flag
and one monotone counter shows the declaration can be checked, not that monotonicity buys the
speedup argued for. That wants a game large enough for the difference to show, and this one
runs in a millisecond either way.

## What it does not do

- **It does not exit nonzero on a stranded state.** A stranding is a finding about the game,
  not a failure of the tool; the specimen is deliberately trapped, and a permanently red
  fixture is a fixture somebody switches off. The caller decides.
- **It does not read the corpus.** It reads a `.game` file. Whether a computation over a
  specimen and one over corpus instances are the same thing with two inputs is `[open]` — see
  `vocabulary-coverage`, which is blocked on the same question.
- **It says nothing about whether a game is good.** Reachable and winnable is a floor.
