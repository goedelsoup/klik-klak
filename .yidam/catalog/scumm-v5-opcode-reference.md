---
name: SCUMM Reference Guide — V5 opcode list
description: Reverse-engineered listing of SCUMM v5 script opcodes by number and name, hosted by ScummVM.
type: other
obtained: true
retrieved: 2026-09-09
ttl_days: 3650
location:
  - kind: url
    value: https://www.scummvm.org/old/docs/specs/scrp-v5.php
used-by:
  - ../corpus/verb/pick-up.yml
  - ../corpus/interaction/acquisition-binding.yml
  - ../corpus/verb/talk-to.yml
  - ../corpus/conversation/branching-tree.yml
  - ../corpus/verb/use-with.yml
  - ../corpus/verb/walk-to.yml
---

**Reverse-engineered, and it says so.** The footer reads "All material © 2000-2002 David Given,
unless where stated otherwise", and the site states "ScummVM is not affiliated in any way with
LucasArts, Inc." This is not LucasArts' documentation and there is none — SCUMM's own
specification was never released. Claims resting on this entry are `[inference]` and stay there.

That said, it is a **different kind** of aggregator from `wikipedia-scumm.md` and the difference
matters when weighing the two. Wikipedia is an account derived from other accounts. This is
derived from the artifact itself: somebody read the bytecode and wrote down what the opcodes are.
It is closer to the thing, and `agent-conduct` is explicit that closer is still not the source —
"a republished bulk file is closer to the source than a derived presentation is, and it is still
not the source." Do not let the proximity promote it.

The document is frozen: it dates from 2000-2002 and its subject stopped changing in 1998, which
is why its TTL is ten years rather than the one and two this corpus gives its living sources.

## What was read

The opcode table, for one question: does SCUMM treat acquiring an object as its own operation?

- **`pickupObject` — opcode `$25`.** The document gives the number and the name and **no
  description**; everything this corpus says about its behaviour comes from elsewhere and is
  marked where it is used.
- **`setOwnerOf` — opcode `$29`**, listed separately. This is the load-bearing observation, and
  it is an observation about the *table* rather than about either opcode: a general
  ownership-setting operation and a dedicated pickup operation both exist. Whatever `pickupObject`
  does, it is not merely setting an owner, or it would not need to be there.
- Read a third time for verbs: `doSentence` (`$19`), `getVerbEntryPoint` (`$0B`) and
  `saveRestoreVerbs` (`$AB`) are listed, all three **without parameters or description**. The
  names are suggestive — a "sentence" is SCUMM's verb-and-objects construction — and the
  entries are empty. `verb/use-with.yml` cites this and rests nothing on it.
- Also present: `getObjectOwner` (`$10`), `getInventoryCount` (`$31`), `findInventory` (`$3D`),
  `findObject` (`$35`).

Read a fourth time, for movement, and it is the richest result the table has given. **Three**
dedicated walk opcodes — `walkActorTo` (`$1E`), `walkActorToActor` (`$0D`), `walkActorToObject`
(`$36`) — names and numbers, no descriptions, the same shape as `pickupObject`. No other verb
this corpus has assessed draws more than one.

And the observation that decided `pick-up` recurs here in the same form: **a general operation
coexists with the dedicated ones.** `putActor` (`$01`), `putActorAtObject` (`$0E`) and
`putActorInRoom` (`$2D`) place an actor somewhere, and `loadRoomWithEgo` (`$24`) changes room
carrying the player. Walking is therefore not merely a change of position, or three walk opcodes
would be redundant beside three placement opcodes. The two families mirror each other on target
kind — coordinates, an object, and in the walk family an actor as well. Also present:
`getActorWalkBox` (`$7B`), `getActorRoom` (`$03`).

`verb/walk-to.yml` rests on this half of the table.

The table was read a second time, for dialogue, and **the finding is an absence.** The only
speech-related opcodes are `print` (`$14`) and `printEgo` (`$D8`), which display a line. **There is
no opcode managing a menu or tree of selectable dialogue options.** The branching conversations
those games are remembered for were built in ordinary script, with no engine primitive underneath
them.

An absence in a reverse-engineered table is weaker evidence than a presence, and the entry says so
rather than trading on it: the table could be incomplete. What makes this one usable is that it is
an absence *alongside* a presence — the same document lists a dedicated `pickupObject`, so it is
not a document that omits gameplay operations as a matter of course.

A second, weaker source was consulted for what `pickupObject` actually does, and it is recorded
here rather than given an entry of its own because nobody here read it directly: search results
describing ScummVM's own `o6_pickupObject()` implementation report it checking whether the object
is already held, adding it, setting the owner to ego, marking it untouchable, setting its state,
and running the object's inventory script. That is a description of **ScummVM's reimplementation**,
which is a primary source for ScummVM and an inference about SCUMM. The distinction is easy to
lose and is the reason this paragraph exists.

## What this document holds that nobody here has looked at

The other roughly two hundred opcodes, the script format itself, and the v5 resource layout. The
corpus read the table for inventory and ownership and stopped.
