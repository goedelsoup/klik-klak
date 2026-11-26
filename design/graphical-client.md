---
title: A graphical client for the bounded game
status: proposal
reconciled: dc79f7f98d77d20e75379f6f28e89f14184d4e35
rests_on:
  - .yidam/corpus/subsystem/verb-menu.yml
  - .yidam/corpus/subsystem/interaction-resolver.yml
  - .yidam/corpus/subsystem/path-solver.yml
  - .yidam/corpus/subsystem/response-presenter.yml
  - .yidam/corpus/data-format/room-manifest.yml
  - .yidam/corpus/data-format/save-state.yml
derives_effects_from:
  - actor:occupies
  - interaction:sets
---

# A graphical client for the bounded game

Compose the existing engine components in a loopback browser client. Keep game transitions in
Rust and use the browser for drawing, pointer input and animation. Exercise
[patrol](specimen/patrol.game) without changing its bindings or its intentional trap.

The [runtime boundary correction](../.yidam/decisions/runtime-boundaries.yml) is the prerequisite.
The client implementation is [screen](../engine/play/src/screen.rs), with a separate
[drawing file](../engine/play/assets/patrol.scene).

## The attempt with the existing vocabulary

Room changes, actor movement, item acquisition, refusals and endings already have bindings.
A target rectangle and an approach point can be supplied beside those bindings, using their
canonical names. No new verb, effect, subsystem or per-frame authoring hook is needed.

A screen introduces a reader the search does not have: the inventory display. An acquired item
that no predicate reads must still appear in a pocket. Keep the existing reduced `World::new`
for analysis and explicitly request `with_inventory` for this client. This is a different
observation set, not a different evaluator. The save reader checks the complete field set, so
snapshots from different observation sets are refused when they differ.

## Boundaries

- Selecting an interaction first walks to the target's approach point. On completion, recheck
  the offered binding through the existing session and commit it once. Conversation choices
  do not require walking.
- A click off floor or on disconnected floor leaves the player in place and gives a reason.
  Do not silently substitute the nearest reachable destination.
- While walking or reading a response, reject gameplay actions and saves in Rust as well as
  disabling their controls. Animation measures real time but does not run a world clock.
- Responses require dismissal. No inferred reading speed or new timing primitive is needed.
- Restore at a choice boundary without running arrival bindings. Restore the player to the
  room's drawing-defined spawn; within-room position is transient and is deliberately unsaved.
- Bind HTTP only to loopback. Validate Host and Origin, serve a fixed asset set, and accept
  bounded request bodies. Save downloads and load uploads carry snapshot contents, never
  arbitrary server filesystem paths.

## Drawing data

The scene uses a 960 × 540 coordinate space. A named `floor` is a simple polygon, shared by
`room` declarations with independent spawn points. A `target` joins a canonical game entity to
its rectangle, approach point and reusable artwork kind. A `figure` places an actor without
inventing a clickable binding. Validate both directions: every spatial binding has a target,
and every drawn target names a binding in that room. Validate actor destinations as well.

These are drawing declarations under the [drawn boundary](../.yidam/decisions/drawn-boundary.yml),
not additions to `.game`. The schematic art shares drawing routines rather than copying one
renderer for each prop or room. The shared floor likewise has one declaration.

The initial slice does not specify a general room editor, art asset pipeline, audio, actor
scaling, scrolling or timed speech. Add those when a worked game supplies a concrete need.
