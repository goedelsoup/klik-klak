# engine

Where this repository stops describing an engine and builds one.

[`.yidam/corpus/`](../.yidam/corpus/) is documentary and [`design/`](../design/) proposes. This
directory is neither: it holds the artifact those two are about. AGENTS.md named the place before
it existed — *"the engine under study is described by `subsystem` and `data-format` nodes, and its
code, when it exists, is not the corpus's to hold. A renderer does not belong in `crates/`."*

## The one rule this directory relaxes

**It holds material that makes no claims.** Every other directory here is prose that asserts
something, and every assertion carries `[verified]`, `[inference]` or `[open]`. Code asserts
nothing and carries no tag.

That is not a softening of the tag discipline; it is an exit from it, and it is a sharper
relaxation than `design/`'s. A proposal may assert intent and is still held to provenance. Code is
outside the scheme entirely — which means **the three tags cannot protect this directory**, and
the gate below is a different kind of check for that reason.

## The rule it does not

**A module may not invent a subsystem.** A **source file** here — `.rs` or `.md` — declares one of
two things in its first ten lines:

```rust
//! realizes: subsystem/verb-menu                 // it IS that component
//! composes: verb-menu, interaction-resolver     // it USES those components
```

```yaml
---
realizes: subsystem/interaction-resolver
---
```

`composes:` is for a **client**: a file that drives components and is none of them. It invents
nothing either — every name in it is checked against the corpus exactly as `realizes:` is — and a
file declaring neither still fails. [`playable`](../.yidam/decisions/playable.yml) added it when
the first driver turned out to have nowhere to say what it was.

The gate fails when `.yidam/corpus/subsystem/<name>.yml` does not exist, and the failure is
deliberately ambiguous: **either the module is a component the corpus has not described, or the
corpus is missing one the engine has just discovered it needs.** Both are findings. Silence is not
available.

**A manifest is not a module.** `Cargo.toml`, a lockfile and a `.gitignore` make no claim about a
subsystem and have nowhere to put one. The rule asked them for a doc comment until
[`two-components`](../.yidam/decisions/two-components.yml), because it was written over an empty
directory — which is the shape of a rule that has never met the thing it governs.

**And a component may not invent a dependency.** A cargo dependency from one engine crate to
another must be a `depends-on` edge between the subsystem nodes. One direction only: the resolver
also declares `depends-on` on `path-solver` and `response-presenter`, and requiring those to be
cargo dependencies would require building them.

This is [`design/`](../design/)'s `derives_effects_from:` one tier down. A design does not get to
invent an effect; an engine does not get to invent a component.

## What the corpus has specified, and what is built

**One crate per subsystem**, named for the node. The directory listing is the roadmap, and the gate
pins this table rather than printing it as a warning: it fails when the list changes and says
nothing while it does not. A standing report naming unbuilt components is a line everybody learns
to scroll past — [`.yidam/decisions/inert-reports.yml`](../.yidam/decisions/inert-reports.yml).

| subsystem | what it is answerable for | |
|---|---|---|
| [`interaction-resolver`](../.yidam/corpus/subsystem/interaction-resolver.yml) | matching a verb, target and second object to a binding, and producing the response or the refusal | **built** |
| [`verb-menu`](../.yidam/corpus/subsystem/verb-menu.yml) | given a target and a state, the verbs to display and no others | **built** |
| [`path-solver`](../.yidam/corpus/subsystem/path-solver.yml) | a route across a room's walkboxes, and reporting arrival | **built** |
| [`response-presenter`](../.yidam/corpus/subsystem/response-presenter.yml) | making a response perceptible, and not returning until it is over | **built** |

And one client, which is not a subsystem and does not claim to be:

| crate | | |
|---|---|---|
| [`play`](play/) | the menu, a choice, the response, and again | `composes: verb-menu, interaction-resolver, response-presenter` |

The first two were built before this directory existed, as private methods of
`crates/reachability`. [`two-components`](../.yidam/decisions/two-components.yml) moved them; what
that phase built was not the components but the place they say they are — and `crates/reachability`
now depends on them rather than keeping a second copy of the language's semantics.

**`path-solver` went the same way and the table is four of four for the first time.** Its region
half was already written — portals, the adjacency graph, a flood fill over it — as private helpers
to `crates/design-gate/tests/adjacency.rs`, a test about whether adjacency needs a relationship.
That is the third time a component turned out to exist somewhere that was not saying so.
[`unbuilt-solver`](../.yidam/decisions/unbuilt-solver.yml).

**Every row of this table is built and one of them has no client.** `play` is a terminal and has
nothing to cross a room with, so nothing composes `path-solver`. That is the honest state rather
than a gap to close by wiring: a text slice with a path solver bolted to it would report a
capability this engine does not have.

**And one of them is exercised against somebody else's game.** A `.game` file cannot state a
polygon — [`drawn-boundary`](../.yidam/decisions/drawn-boundary.yml) — so `path-solver` is the only
component whose evidence is not `design/specimen/`. Its test reads two of *Delores*'s rooms out of
the instrument's transcription rather than copying them, and re-derives the adjacency counts that
file pins, which is what holds two implementations of one rule equal across a boundary neither may
cross.

## The gate

```
mise run engine-check      # cargo test -p design-gate --test engine
```

Runs inside `mise run ci`. It lives in `crates/design-gate` because that crate already reads the
corpus; the crate's name is narrower than its job and
[`.yidam/decisions/engine-directory.yml`](../.yidam/decisions/engine-directory.yml) records the
condition under which it should be renamed.

## What this gate does not catch

Stated because a gate mistaken for complete is worse than no gate.

- **It checks a name, not a component.** That a module declares `realizes: subsystem/path-solver`
  says nothing about whether it finds paths, or whether it does what that node says a path solver
  is answerable for. Only a reader catches that.
- **It cannot tell a live specification from a stale one.** The node behind the name may have been
  revised out from under the module. `design/`'s watermark —
  [`.yidam/decisions/second-document.yml`](../.yidam/decisions/second-document.yml) — is the
  available remedy and is not applied here yet, because nothing here rests on a node's prose so
  far.
- **It does not run the engine.** Whether the thing built plays a game is not a property any check
  here inspects.

## Playing one

```
cargo run -p play -- ../design/specimen/lighthouse.game
cargo run -p play -- ../design/specimen/lighthouse.game play/scripts/lighthouse-won.txt
cargo run -p play -- ../design/specimen/patrol.game    play/scripts/patrol-lost.txt
```

With a script it prints a transcript; without one it reads choices from stdin — and the two read
differently on purpose. A **key** is what a script writes and a transcript records
(`use-with crowbar on crate`); a **phrase** is what a player reads
(`Use a short crowbar with a nailed-shut crate`), built from six templates in `verb-menu` and the
labels the author wrote. [`verb-names`](../.yidam/decisions/verb-names.yml). The transcript of a
winning run is pinned by `play/tests/transcript.rs` — thirteen turns, and three of them say nothing
and are not silent, because the world moved and the movement is the response.

**And one losing run per specimen**, pinned by `play/tests/lost.rs`. Each walks into the trap
`crates/reachability` reports and names the line it blames. What they show is the half of
`failure-mode/unwinnable-state` a checker cannot: **after the mistake the game goes on answering
normally.** Read `patrol-lost.txt` if you read one — the strongroom opens for the first time on the
way out, because the porter has left the corridor, and every line reads as progress.
[`walked-in`](../.yidam/decisions/walked-in.yml).

**The state is the client's and no component's.** All four subsystems declare none across frames,
so `World` and `State` live in `crates/game-format` and the loop threads a `State` through itself.
A component that held it would be a component the other two had to ask.

## What is not here yet

`path-solver`, and **a clock**.

Every line a player is told goes through `response-presenter`, which returns when its sink
returns — and this client's sink writes a line and comes back at once, so it holds the player for
nothing at all. The component reports the weight and the client discards it. That is honest rather
than fixed: a terminal with no clock and no input between turns has nowhere to put an interval, and
**the first client that can hold a player is the first one with a frame.**
[`held`](../.yidam/decisions/held.yml).

The number is real even where the holding is not. A winning run of `lighthouse.game` asks the
player to read **471 characters across thirteen turns**, the longest single hold is 74, and three
turns hold for nothing because a movement is a response that takes no reading.

`path-solver` has neither code nor data and stays that way for a reason rather than by neglect: a
text slice has nothing to walk. `walk-to` moves the player between rooms and never across one, and
**the first thing that would need a path is a screen**.

The first thing that had to happen before a module could do anything useful has happened.
`game-format` used to parse `say` and declaration labels and discard both — `"say" | "otherwise"
=> {}` — so a game's entire player-facing text was dropped at the parse, which meant no engine
built on that tree could emit a word. **52% of a specimen**, measured across the four in
`crates/game-format/tests/authored.rs`.

The tree keeps all of it now, and the state model goes on ignoring it:
[`authored-text`](../.yidam/decisions/authored-text.yml) separated the two claims that had been
written as one. A `Binding` carries `says` and `refusals`, a `Game` carries `labels`, and not one
number `crates/reachability` publishes moved when those fields were filled — which is the
evidence that `effect-list-semantics`'s position was about the state model rather than about the
reader.
