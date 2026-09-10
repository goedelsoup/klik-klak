---
name: Delores — game source, and the Dinky engine's own manual
description: The complete authored source of a shipped point-and-click adventure by Ron Gilbert and David Fox, with the engine's reference manual — the first artifact this corpus can count rather than quote.
type: dataset
obtained: true
retrieved: 2026-09-10
ttl_days: 1825
location:
  - kind: url
    value: https://github.com/grumpygamer/DeloresDev
    description: "The repository; read at commit 419ce5eb99a6a58e85a0030ae8d7023980535f20"
  - kind: url
    value: https://grumpygamer.com/delores_dev/
    description: "\"Delores Source Code Released!\", the announcement"
used-by:
  - ../corpus/data-format/dialogue-script.yml
  - ../corpus/conversation/topic-list.yml
  - ../corpus/conversation/branching-tree.yml
  - ../corpus/verb/walk-to.yml
  - ../corpus/prior-art/thimbleweed-park.yml
  - ../corpus/data-format/interaction-table.yml
  - ../corpus/hotspot/static-region.yml
  - ../corpus/cutscene/scripted-sequence.yml
  - ../corpus/actor/non-player-character.yml
  - ../corpus/world-flag/object-state.yml
  - ../corpus/world-flag/latched-flag.yml
  - ../corpus/data-format/interaction-table.yml
  - ../corpus/interaction/default-binding.yml
  - ../corpus/interaction/unary-binding.yml
---

**A source of record, and the first one this corpus can *count*.** *Delores: A Thimbleweed Park
Mini-Adventure* shipped in 2020, programmed by Ron Gilbert and David Fox on the engine built for
Thimbleweed Park ([twp-dev-diary](twp-dev-diary.md)). The repository is the game's complete
authored source — every room, every binding, every line of dialogue — plus `HelpDocsHtml/`, the
engine's own reference manual.

Every other prior-art source here is somebody describing an authoring surface. This *is* one, in
full, with a game written in it. Claims about what the vocabulary contains and how it was used
may be `[verified]`, and the counts below are reproducible: clone the repo at the pinned commit
and run the same greps.

**Licence, and what it permits.** Copyright 2020 Terrible Toybox, Inc., under a custom licence:
"The purpose of making the Software available is for others to learn from", personal and hobby
use only, no repackaging or redistribution. Reading it and reporting figures is the intended
use; **nothing from it is vendored into this repository** and nothing here reproduces more than
the short quotations a claim rests on. The engine binary and Wimpy are *not* in the repo and
remain proprietary — so the authoring surface is fully readable and the thing that executes it
is not.

## What the artifact is

| | |
|---|---|
| room scripts | 21 files, 5,743 lines (`Scripts/Rooms/*.dinky`) |
| shared helpers | 15 files, 4,034 lines (`Scripts/Helpers/`) |
| dialogue | 11 files, 692 non-blank lines (`Scripts/Yacks/*.yack`) |
| inventory | one 23 KB file |
| room geometry | 22 `.wimpy` files, 307 KB total, written by the Wimpy editor |
| authoring templates | 5 (`ROOMNAME`, `OBJECTNAME`, `PROPNAME`, `TRIGGERNAME`, `ROOMNAME.wimpy`) |
| manual | 34 HTML pages |

**Dinky** is the language: Squirrel with a macro preprocessor, renamed. `#macro VERB($a) verb_$a
<- function(noun2=null)` — a binding is a field on an object table named `verb_<verb>`, holding a
closure that takes an optional second noun.

## What was counted — 2026-09-10

**Bindings.** 203 live `VERB(...)` bindings across the rooms and the inventory; six more are
commented out. 46 distinct verbs are used. **107 of the 203 — 52.7% — are `lookat`.**

**That figure read 110 until it was recounted.** There are 111 `VERB(lookat)` occurrences in
`Scripts/`; three are commented out and one is in `Defines/DefineActors.dinky`, which the 203
excludes. 111 − 1 = 110 was the arithmetic, and it counted three of the six commented-out bindings
this same entry records. A figure and its own caveat, in one entry, disagreeing.

| | count | share |
|---|---|---|
| exactly one statement | 121 | 59.6% |
| …and that statement is a `sayLine` | 91 | 44.8% |
| contains a conditional | 62 | 30.5% |
| contains a blocking or timed call | 34 | 16.7% |
| calls another verb slot | 18 | 8.9% |
| starts a thread | 12 | 5.9% |
| contains a loop | 7 | 3.4% |
| **free of loops, threads, blocking and delegation** | **155** | **76.4%** |

The 155 call 28 distinct functions between them, and the census is short: `SAY` 219, `sayLine`
180, `ONCE` 22, `objectState` 9, `exitRoomFromDoor` 8, `pickupObject` 7, `inInventory` 4,
`removeInventory` 3, and a tail of sound, animation and camera calls.

**Delegation, recounted — 2026-09-10.** That row read *"calls another verb slot on the same
object"* when it was first written, and the count is right while the description is not. 34
`verb_<x>(` call sites appear in `Scripts/`; **20 are inside an authored binding** and 14 are
engine helpers, room enter and exit code, cutscene bodies and Yack effect blocks. The 20 sit in
the 18 bindings above, two of which carry two calls each.

**Six of the 18 delegate to the object the binding is on.** Twelve name a different object, and
two of those name one in another room. Seven distinct shapes are involved and only two of them are
a response running another response on the same thing; the taxonomy and what it decided are in
`.yidam/decisions/delegation.yml`.

**Objects.** 164 `DEFINE_OBJECT` declarations. **46 (28%) carry no verb slot at all** — a name,
an article, some properties, and nothing that happens. 82 carry exactly one. Two carry five or
more. Alongside them, 290 bare props (`name = {}`): positioned art with no name and no
behaviour.

## Touchable against hidden — 2026-09-10

**34 `objectTouchable` calls on 18 receivers; 11 `objectHidden` on 6.** Only three receivers get
both, and one of those three is the literal `this`.

Where both are used they move in lockstep, adjacent, saying one thing twice:

    objectHidden(SheriffsOffice.jailKey, NO)      objectHidden(SheriffsOffice.jailKey, YES)
    objectTouchable(SheriffsOffice.jailKey, YES)  objectTouchable(SheriffsOffice.jailKey, NO)

**Fifteen receivers change targetability and never hiddenness** — `jailDoor` four times, `ladder`,
`sheriff`, `coroner`, `trashcan`, `trafficCone1` and the rest. Those objects stay drawn and the
cursor is switched on and off over them.

**Three receivers change hiddenness and never targetability**, and all three are one gag:
`greenFireHydrantTrigger` flickers `objectHidden(fireHydrant, randomfrom(YES, NO))` under a
`quiet_glitch` sound and then restores it. Briefly invisible and still clickable, for a moment, as
an effect.

**So drawing is not what `objectHidden` mostly governs — `objectState` is.** 146 `objectState`
calls choose which art an object shows, against 11 blunt on/off hides. Targetability is
`objectTouchable`'s alone, and it is the load-bearing one.

## The verb menu, and why there is no default binding — 2026-09-10

**This game has no fall-through, because it has no fixed verb bar.** Clicking an object opens a
popup, and `openVerbsFor` builds it by walking the master verb list and keeping only what passes
`verb_id in object._known_verbs && _checkVerbValid(object, verb_id)`.

- **`_known_verbs` is reflection over the object's own fields.** `registerVerbs` iterates the
  object's keys, keeps every one beginning `verb_` that is not a `_text` or `_desc`, and warns if
  any is missing from the master list. An object with none gets no `_known_verbs` and is not
  interactable at all — which **46 of the 164 objects** are.
- **`_checkVerbValid` hides rather than refuses.** Binary verbs are never shown (`if (verb in
  ROOT(verb_usewith_list)) return NO`); a verb with no `okverb_` is shown; a verb with one is shown
  only if it returns true. So a failing precondition removes the verb from the menu, and no line of
  refusal is said.

**So no (verb, object) pair the author did not write is reachable by a player.** The distinctions
*never authored*, *authored and empty* and *authored to refuse* do not collapse here — the first is
unreachable, the second is reachable and silent, and the third does not exist as a category, because
what would have refused is simply absent from the menu.

**`default_verb` is in this file and means something else.** Not "what happens when nothing
matched" but *which of this object's authored verbs is the single-click action*, resolved by a fixed
preference order — `lookat`, `examine`, `read`, `lookatfar`, `instant`, `walkto` — with a boot-time
warning for an object that has verbs and none of those, and another if a hand-written `default_verb`
names a verb the object does not have.

**What it costs is a leak, and it is the same leak recorded elsewhere as a design variable.** A
verb's presence in the menu tells the player the object affords it, and its disappearance tells them
a precondition is not met — information a refusal line would have controlled the wording of. That is
the trade this engine takes in exchange for never authoring a default.

## Arity — 2026-09-10

**26 of the 203 bindings take a second object — 12.8%** — counted as those whose body or guard
mentions `noun2`. They use **16 distinct binary verbs**, and the 30 verbs used unarily do not
overlap them at all: **not one verb in the game is used both ways.**

    fill 4   empty 4   giveto 2   loosen 2   tighten 2   repair 2
    takephoto, toss, burnin, dropin, puton, jab_with, dissolve, scrape, vandalize_with, unlock_with — 1 each

**Arity is declared rather than inferred.** `Scripts/Helpers/VerbHelpers.dinky` holds
`ROOT(verb_usewith_list)` — 18 verb ids — beside `verb_no_walkto`, `verb_no_reach` and
`verb_not_close`, and a boot-time loop warns if any member is missing from the master list. The
master list declares 57 verbs.

**There is no generic "use with" in it.** No `VERB(use)` appears anywhere in `Rooms/` or
`Inventory.dinky`; the only `verb_use` in the game is the one `door_verbs = YES` synthesises for
going through a door, which is unary. Every binary interaction is a *named* verb — `unlock_with`,
`jab_with`, `puton`, `scrape` — so this game kept arity and multiplied verbs rather than routing
them through one. That is the opposite of the reduction
`.yidam/decisions/binary-dimension.yml` was testing, and further from it than this corpus's own
six-verb design.

## The dependency graph — 2026-09-10

**What Gilbert draws as a Puzzle Dependency Chart, read off the source instead.** Every unit that
can read or write named state was extracted — 203 bindings, 50 room `enter`/`exit` bodies, 101
Yack topics — and an edge drawn from a unit that writes a name to a unit that reads it. `VERB_OK`
guards are folded into the binding they guard.

| | |
|---|---|
| units | 354 |
| dependency edges | 115, carried by 41 distinct state names |
| units touched by any edge | 94 |
| longest dependency chain | **5 steps** |
| maximum antichain (Dilworth width) | **54** |
| layer sizes by longest-path depth | 31, 39, 19, 3, 2 |
| out-degree where a step unblocks anything | mean **2.45**, median **2**, max 9, n = 47 |

Out-degree distribution: 1 → 20 steps, 2 → 12, 3 → 5, 4 → 3, 5 → 3, 6 → 2, 8 → 1, 9 → 1. So
**43% of unblocking steps open exactly one thing and 36% open two or three.**

The names carrying the most edges are what an adventure game's state looks like:
`YACK.asked_for_coroner_time` 12, `carrying.glass` 9, `GLOBAL.hired_lenore_state` 9,
`MainStreet.fireHydrant.isWaterSource` 8, `carrying.screwdriver` 8, `Alleyway.valve.position` 8,
`Bank.open` 6.

**What this is not.** A Puzzle Dependency Chart is a designer's list of puzzles; this is every
read-after-write over a named variable. It over-counts — `looked_at`, `seen`, `tried_switch` and
dialogue loop counters are in it — and it under-counts, because a dependency carried by anything
other than a named variable is invisible to it. Removing edges internal to one object leaves 77
units, 90 edges, a longest chain of 4 and an antichain of 47; the shape does not change.

**Reproducing it.** The extractor is not committed, because its input cannot be. Structure comes
from indentation rather than braces — 8,592 lines here start with a tab against 211 with a space,
`DEFINE_OBJECT` sits at one tab in 163 of 164 cases and `VERB`/`VERB_OK` at two — and any
reimplementation should reproduce these four figures before its dependency numbers are trusted:
**203 bindings, 46 distinct verbs, 107 live `lookat` bindings, 141 Yack bracket expressions.**

**Exits.** 20, and **eighteen of them are a bare declaration**: `to_door = @Room.doorObject` plus
`door_verbs = YES`, with no code. The engine synthesises the verbs — `_initObject` "sets up door
verbs if `object.door_verbs` is set". The two door objects that carry any verb slot carry
`lookat` only.

**Two exits are gated, and this entry said none were.** It read *"No exit in the game is ever
gated: no `to_door` object is made untouchable or hidden anywhere"*, and both halves are false:

- `Nickel.door`, the way out of the newspaper office, carries `VERB_PRE(open)` returning `NO`
  unless the camera is in inventory — *"I should find the camera before setting out."* It is the
  gate that starts the game.
- `MainStreet.bankDoor` runs `objectTouchable(this, Bank.open ? YES : NO)` in its own `enter`.
  A `to_door` object, made untouchable, conditionally.

Both are invisible to a reader looking for gating *on the exit verb*, which is what the earlier
reading looked for. Neither is written as a `VERB_OK`: one is a `VERB_PRE` on a binding the engine
generates from `door_verbs = YES` — a precondition on a response nobody wrote — and the other is a
touchability write in an unrelated function. The archetype hides the seam.

Separately, 11 `VERB(walkto)` slots exist and every one is an exit or a refusal to exit — 8 are
a single `exitRoomFromDoor(...)` between outdoor screens with no door object, and 3 are a
`sayLine` saying why you cannot go ("The street looks blocked off.").

**The verb vocabulary is a list, and it is meant to be edited.**
`Scripts/Helpers/VerbHelpers.dinky` opens: "When adding a new verb, it must be added to the
following three lists, and the forth if it is a use-with verb." `master_verb_list` holds **63**
verbs in menu order; `master_verb_names` gives each a display string with noun slots (`"Unlock
$NOUN1$ with $NOUN2$"`); `verb_usewith_list` marks the **18** that are binary; and three more
lists declare per-verb behaviour — `verb_no_walkto` (4), `verb_no_reach` (15), `verb_not_close`
(1, `verb_lookat`).

`registerVerbs` finds an object's bindings by reflecting over keys beginning with `verb_`, and
**warns if a verb is not in `master_verb_list`** — a load-time check that the authored table
draws only on the declared vocabulary.

**Preconditions exist in the vocabulary and the game never authors one.** `VERB_OK($a)` declares
`okverb_<verb>`, a predicate the engine consults before offering a verb. The only uses anywhere
are engine-supplied defaults for declarative doors in `SystemHelpers.dinky` — a door may be
opened only if `state != "open"`, closed only if `state == "open"`. Every precondition the game's
authors wrote is an `if` inside a response.

**Triggers.** `Templates/TRIGGERNAME.dinky` declares `triggerEnter(actor)` and
`triggerLeave(actor)`. The game uses 11 and 13 of them across four rooms — arrival at, and
departure from, a region *within* a room.

**Room entry code.** 20 of 21 rooms have `enter = function(from_door=null)`; 237 lines between
them, mean 11.8, and three rooms hold 36-44 of them.

## Yack — a bounded declarative dialogue language, shipped

`.yack` is a second language, for conversations only. 692 non-blank lines across 11 files:

| line kind | count |
|---|---|
| actor line — `willie: SAY(12318,"Business is booming.")` | 206 |
| section label — `=== late ===` | 115 |
| player option — `1 SAY(12316,"You're closing up late.") -> late` | 113 |
| brace block / statement | 113 |
| jump — `-> main` | 91 |
| `if [once]` … `endif` | 28 |
| directive — `parrot ON` / `parrot OFF` | 15 |

**141 guards in square brackets.** 62 are bare modifier tags — `once` 47, `showonce` 13,
`temponce` 2 — and **79 are real expressions**. The operators they use, counted:

`==` 39 · `!` 16 · `&&` 7 · `>` 4 · `and` 3 · `||` 3 · `!=` 2 · `>=` 2

and the readable functions in them: `YACK(x)` 47 (a conversation-scoped variable), `isDone()` 6,
`GLOBAL(x)` 5, `inInventory(x)` 5, `hasHeardAbout()` 3, `totalCompletedCount()` 3, and four
others once each. Guards also read object fields across rooms directly —
`MainStreet.bankDoorSign.took_photo`, `Bathroom.tried_switch`, `Bank.open`. One example entire:

> `2 SAY(12051,"There is a job opening at the bank.") -> job_opening [YACK(knows_about_invicted) and !Bank.open and MainStreet.bankDoorSign.seen]`

**No guard anywhere escapes to Dinky.** The escape is elsewhere and it is delimited: `{ … }`
blocks, 46 of them, holding assignments (`{ YACK(heard_about_job) = YES }`) and, in a minority,
arbitrary calls — `inputOff`, `startthread`, `actorWalkTo`, `verb_open()`, `saveGame`,
`startEnding`.

## What was looked for and not found — read this before citing

**Yack's grammar is not documented.** `HelpDocsHtml/yacks.html` documents the *API* — the Dinky
functions that start and end a conversation and render its choices — and its description of the
file format is the single word "TBD". Everything this entry says about the grammar is read off
eleven files, and a construct those files never use is a construct this corpus cannot know
about. `wimpy.html` is likewise "TDB".

**No design commentary.** The source says what was built and never why. Any claim about a
motivation must come from the diary or from Grumpy Gamer, not from here.

**One small game.** Delores is 21 rooms and about two hours; Thimbleweed Park is five times the
size and its source is not public. Every proportion above is a proportion of *this* game, and
the shape of a mini-adventure built to show off an engine is not guaranteed to be the shape of a
full one. Where a figure is offered as evidence about adventure games generally rather than
about this one, it is `[inference]`.

## What this artifact holds that nobody here has looked at

The `.wimpy` files themselves, in detail — 307 KB of room geometry, walkboxes, lighting and
object placement, which is the declarative half of the surface and has been read here only
through one quoted example. `Scripts/Helpers/SystemHelpers.dinky` (1,258 lines) and
`Options.dinky` (659), which are where the authors' own abstractions live. The 34-page manual,
of which four pages were read. And `Scripts/Inventory.dinky`, read for binding
counts and for its one delegation site, and not otherwise.
