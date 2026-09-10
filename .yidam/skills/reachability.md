---
name: reachability
description: Whether every reachable world-state configuration can still reach the ending; blocked on an authored game to read
---

# reachability

**Status: stub, and blocked on an input that does not exist.** Approved in
`.yidam/decisions/proposals.yml` over a stated objection; see the rationale there and the
account in `.yidam/decisions/seed-scope.yml`.

## What it computes

Given an authored game, whether every world-state configuration reachable by play can still
reach the ending — and where it cannot, the earliest interaction after which it cannot. This
is `failure-mode/unwinnable-state.yml` detected mechanically rather than by walking into it.

The two ordinary causes are different problems and the computation should distinguish them:

- **Consumption** — an object needed later is spent or destroyed earlier. A property of a
  single binding, checkable where the binding is written.
- **Ordering** — a gate closes with something still needed behind it. A property of the graph
  of gates, checkable nowhere in the current authoring vocabulary. This is the one that needs
  a real reachability computation.

## What it reads

An authored game's state graph: the world flags, the interactions that write them, and the
preconditions that read them.

**That graph does not exist in this repository.** This corpus holds the vocabulary a game
would be authored in and holds no game. The input is not thin — it is absent. Nothing about
this stub's silence is a defect in the stub, and this section exists so a reader meeting it
later does not conclude otherwise.

## What it returns

Reachable and stranded state sets, and for each stranded set the binding that stranded it.

## Prelude domain

`graph-topology` — vendored at `.yidam/.vendor/prelude/domains/graph-topology/`. Reachability
over a directed state graph is exactly what that layer holds.

## What would unblock it

Something authored in this vocabulary: a `data-format/interaction-table` instance filled in
with real bindings and real flags, small enough to check by hand and large enough to have an
ordering.

**And it has to be authored in a language where this question is decidable, which is a
constraint this section did not previously state.** Reachability over a state graph is
computable; reachability over a state graph whose preconditions may be arbitrary script is not.
So a game written in a vocabulary with a general-purpose escape hatch is not a missing input to
this calculator — it is a counterexample to it, and no amount of it arriving would help.
`failure-mode/unwinnable-state.yml` marks itself `detectable_at: authoring` on the grounds that
its causes are "decidable from the authored content alone", which holds exactly while the
authored content is decidable.

That makes this stub's blockage narrower and more actionable than "no game exists". It is
waiting on a decidable authoring language, and a proposal for one is
`design/bounded-expression-language.md`.

Where the specimen lives is settled: `design/specimen/`, as part of the proposal it exercises
rather than as a purpose beside it. See
[`.yidam/decisions/specimen-placement.yml`](../decisions/specimen-placement.yml).

One exists — `design/specimen/lighthouse.game` — and for a while this stub could not have run
against it even so. Its one deliberate defect is a **consumption** trap, the first of the two
causes named above, and the effect vocabulary had no way to say an item leaves the inventory. A
checker reading that game would have reported it winnable.

`interaction -consumes-> inventory-item` closes that, so **the input to this calculator now
exists in principle**: a game whose one defect is expressible, whose flags declare whether they
latch, and small enough to check the answer by hand. What is still missing is the calculator.

Two things about that game are worth knowing before writing one:

- **Monotonicity is the whole performance story.** `world-flag/latched-flag` argues that a flag
  which only moves one way makes the reachable state space a lattice rather than a graph, so the
  search is a walk over a partial order rather than over every combination. The specimen declares
  one latching flag and one latching counter, which is enough to test that claim rather than
  assert it.
- **Neither cause is checkable at the site.** `failure-mode/unwinnable-state` used to hold that
  consumption was "checkable where the binding is written" and now records the correction: a
  consuming binding is locally correct and stays correct. Both causes want this computation.

See `design/specimen/README.md`, and note that its first version reported four gaps where one
was real — a worked example detects trouble well and diagnoses it badly.
