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
ordering. That artifact does not belong in `.yidam/corpus/` — it is not a documentary claim —
and where it should live is an open structural question. See the directory conventions,
"A directory these conventions do not name".
