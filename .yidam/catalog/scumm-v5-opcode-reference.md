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

The script format itself and the v5 resource layout. The corpus read the table for inventory and
ownership and stopped; **the rest of the table has now been read and is recorded below.**

This section previously said *"the other roughly two hundred opcodes"*. The page lists **eighty-
four distinct opcodes in eighty-seven encodings**, so that figure was wrong by more than double
and nothing had counted it. `.yidam/decisions/third-population.yml`.


## The opcode table

All eighty-four distinct opcodes the page lists, with their encodings — several names carry more
than one byte value, because SCUMM sets parameter bits in the opcode itself.

**The last column is this corpus's reading, not the document's**, and it applies the partition
`.yidam/decisions/forty-eight.yml` used on AGS's action table, with one category added that AGS
had no need for:

* *predicate* — a test that branches.
* *read* — **writes a value into a variable for a later test.** SCUMM's `get*` and `find*` family.
  AGS's declarative table has no equivalent and neither does this design: a predicate here reads
  world state directly and there is nowhere to store a result, so this category is an artifact of
  a bytecode with a variable file rather than of the vocabulary.
* *control* — acts on script execution, including the arithmetic and assignment opcodes.
* *presentation* — writes nothing a predicate in this design could read.
* an effect name — writes world state, and names the effect of this design it maps onto.

Every claim drawn from this table is `[inference]`, per the standing above: the document is
reverse-engineered and the proximity does not promote it.

| opcode | encoding | this design |
|---|---|---|
| `actorFollowCamera` | $52,$52 | *presentation* |
| `actorFromPos` | $15,$15 | *read* |
| `actorSet` | $13 | *presentation* |
| `actorSetClass` | $5D | *presentation* |
| `animateActor` | $11,$11 | *presentation* |
| `cursorCommand` | $2C | *presentation* |
| `cutScene` | $40 | *control* |
| `doSentence` | $19 | *control* |
| `drawObject` | $05 | *presentation* |
| `dummy` | $67 | *control* |
| `endCutScene` | $C0 | *control* |
| `equalZero` | $28 | *predicate* |
| `expression` | $AC | *control* |
| `faceActor` | $09 | *presentation* |
| `findInventory` | $3D | *read* |
| `findObject` | $35 | *read* |
| `freezeScripts` | $60 | *control* |
| `getActorCostume` | $71 | *read* |
| `getActorElevation` | $06 | *read* |
| `getActorFacing` | $63 | *read* |
| `getActorMoving` | $56 | *read* |
| `getActorRoom` | $03 | *read* |
| `getActorScale` | $3B | *read* |
| `getActorWalkBox` | $7B | *read* |
| `getActorWidth` | $6C | *read* |
| `getActorX` | $43 | *read* |
| `getActorY` | $23 | *read* |
| `getAnimCounter` | $22 | *read* |
| `getClosestObjActor` | $66 | *read* |
| `getDist` | $34 | *read* |
| `getInventoryCount` | $31 | *read* |
| `getObjectOwner` | $10 | *read* |
| `getObjectState` | $0F | *read* |
| `getRandomNumber` | $16 | *read* |
| `getScriptRunning` | $68 | *read* |
| `getVerbEntryPoint` | $0B | *read* |
| `ifClassOfIs` | $1D | *predicate* |
| `isActorInBox` | $1F | *predicate* |
| `isEqual` | $48 | *predicate* |
| `isGreater` | $78 | *predicate* |
| `isGreaterEqual` | $04 | *predicate* |
| `isLess` | $44 | *predicate* |
| `isNotEqual` | $08 | *predicate* |
| `isSoundRunning` | $7C | *predicate* |
| `lessOrEqual` | $38 | *predicate* |
| `lights` | $70 | *presentation* |
| `loadRoom` | $72 | `move` |
| `loadRoomWithEgo` | $24 | `move` |
| `matrixOp` | $30 | *control* |
| `move` | $1A | *control* |
| `multiply` | $1B | *control* |
| `notEqualZero` | $A8 | *predicate* |
| `overRide` | $58 | *control* |
| `panCameraTo` | $12 | *presentation* |
| `pickupObject` | $25 | `give` |
| `print` | $14 | *presentation* |
| `printEgo` | $D8 | *presentation* |
| `pseudoRoom` | $CC | *structural* |
| `putActor` | $01 | `move` |
| `putActorAtObject` | $0E | `move` |
| `putActorInRoom` | $2D | `move` |
| `quitPauseRestart` | $98 | *control* |
| `resourceRoutines` | $0C | *presentation* |
| `roomOp` | $33 | *presentation* |
| `saveRestoreVerbs` | $AB | *presentation* |
| `setCameraAt` | $32 | *presentation* |
| `setObjectName` | $54 | *presentation* |
| `setOwnerOf` | $29 | `give / spend` |
| `setState` | $07 | `set-flag, clear-flag` |
| `setVarRange` | $26 | *control* |
| `soundKludge` | $4C | *presentation* |
| `startMusic` | $02 | *presentation* |
| `startObject` | $37 | *control* |
| `startSound` | $1C | *presentation* |
| `stopMusic` | $20 | *presentation* |
| `stopObjectCode` | $00 | *control* |
| `stopObjectScript` | $6E | *control* |
| `stopScript` | $62 | *control* |
| `stopSound` | $3C | *presentation* |
| `subtract` | $3A | *control* |
| `wait` | $AE | *control* |
| `walkActorTo` | $1E | *presentation* |
| `walkActorToActor` | $0D | *presentation* |
| `walkActorToObject` | $36 | *presentation* |
