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

Where the specimen itself should live is still open. `design/` now exists and is where
non-documentary material goes, which answers half of it — but that directory holds prose
proposals with front matter, and a worked game is neither. Whether a specimen is part of the
proposal it exercises, or wants somewhere of its own, is not settled here.
