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

**A module may not invent a subsystem.** A file here that implements one declares it in its first
ten lines:

```rust
//! realizes: subsystem/response-presenter
```

```yaml
---
realizes: subsystem/interaction-resolver
---
```

The gate fails when `.yidam/corpus/subsystem/<name>.yml` does not exist, and the failure is
deliberately ambiguous: **either the module is a component the corpus has not described, or the
corpus is missing one the engine has just discovered it needs.** Both are findings. Silence is not
available.

This is [`design/`](../design/)'s `derives_effects_from:` one tier down. A design does not get to
invent an effect; an engine does not get to invent a component.

## What the corpus has specified and nothing has built

The gate pins this list rather than printing it as a warning, so it fails when the list changes and
says nothing while it does not. A standing report naming unbuilt components is a line everybody
learns to scroll past — [`.yidam/decisions/inert-reports.yml`](../.yidam/decisions/inert-reports.yml).

| subsystem | what it is answerable for |
|---|---|
| [`interaction-resolver`](../.yidam/corpus/subsystem/interaction-resolver.yml) | matching a verb, target and second object to a binding, and producing the response or the refusal |
| [`verb-menu`](../.yidam/corpus/subsystem/verb-menu.yml) | given a target and a state, the verbs to display and no others |
| [`path-solver`](../.yidam/corpus/subsystem/path-solver.yml) | a route across a room's walkboxes, and reporting arrival |
| [`response-presenter`](../.yidam/corpus/subsystem/response-presenter.yml) | making a response perceptible, and not returning until it is over |

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

## What is not here yet

No code. The directory holds its contract and its gate, and the first module is the next phase's
work. A directory created with a stub inside it is a directory whose first commit is a lie about
progress.

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
