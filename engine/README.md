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

**And a component may not invent a dependency.** A dependency between engine components must
match a `depends-on` edge between their subsystem nodes. The gate checks both directions for built components. Clients declare `composes:` instead;
the resolver depends only on the verb menu, and the client coordinates paths and presentation.

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

One client crate provides a terminal binary and a graphical binary:

| crate | | |
|---|---|---|
| [`play`](play/) | the menu, a choice, the response, and again | `composes: verb-menu, interaction-resolver, response-presenter, path-solver` |

The first two were built before this directory existed, as private methods of
`crates/reachability`. [`two-components`](../.yidam/decisions/two-components.yml) moved them; what
that phase built was not the components but the place they say they are — and `crates/reachability`
now depends on them rather than keeping a second copy of the language's semantics.

**`path-solver` went the same way and the table is four of four for the first time.** Its region
half was already written — portals, the adjacency graph, a flood fill over it — as private helpers
to `crates/design-gate/tests/adjacency.rs`, a test about whether adjacency needs a relationship.
That is the third time a component turned out to exist somewhere that was not saying so.
[`unbuilt-solver`](../.yidam/decisions/unbuilt-solver.yml).

**Every component now has a client.** The terminal drives menu, resolution and presentation;
the `screen` binary adds within-room movement using the path solver. Both use the same session.

The solver retains its borrowed *Delores* evidence and also runs on independently authored
scene data. A `.game` file still does not contain polygons. The distinction is between game
bindings and drawing data, not between testable and untestable components.

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

## Presentation and persistence

The terminal's interactive sink holds each line until the player dismisses it. Scripted runs
collect lines immediately. A timed, interruptible presentation belongs to a graphical client;
no authoring command for elapsed time is introduced.

At a choice prompt, `save <path>` writes a named snapshot and `load <path>` restores it. Loading
validates the complete snapshot before replacing state and does not replay arrival effects.
Snapshots accept reordered top-level declarations but refuse changed game content; there is no
migration policy yet. Save between turns, before the game has ended.

The path solver accepts simple polygon rings with exact shared-edge adjacency. Its returned
segments stay on connected floor, including concave regions. Invalid rings are distinct from
unreachable destinations and clicks off the floor. It is exercised independently of the terminal.

## Runtime boundaries

Counter initialization follows the tracked declaration names. Arrival fires at startup and on
an atomic action that changes the room; a stationary action does not re-enter it. Opening lines
are retained and presented before the first choice. The correction and its evidence are recorded
in [runtime-boundaries](../.yidam/decisions/runtime-boundaries.yml).

## Playing on screen

From the repository root:

```sh
cargo run -p play --bin screen --manifest-path engine/Cargo.toml -- design/specimen/patrol.game engine/play/assets/patrol.scene
```

Open `http://127.0.0.1:8787`. An optional third argument chooses another port (`0` allocates one
and prints its address). Stop the local server with Ctrl-C. Nothing is deployed.

Select a target, then a contextual action. Select an inventory item to offer its applicable
`use-with` action. Click empty floor to walk. Responses wait for Continue; keyboard users can
Tab through targets and press Enter on actions. Save game downloads a snapshot; Load game
reads one back. Loading resets the player's within-room position to the room's spawn.

The [graphical proposal](../design/graphical-client.md) states the observation and timing
boundaries. The client composes all four components; it adds no authoring effect. Its schematic
room data is [patrol.scene](play/assets/patrol.scene), separate from the unchanged specimen.

The optional [browser acceptance check](play/scripts/browser-check.mjs) needs Node with built-in
`fetch` and `WebSocket` and a Chrome debugging endpoint. It drives the visible winning route,
checks response holds and restores a snapshot, then checks the mobile layout. It adds no npm
packages. Engine tests exercise the same client boundaries without needing a browser.
