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
  - ../corpus/world-flag/object-state.yml
  - ../corpus/world-flag/latched-flag.yml
  - ../corpus/data-format/interaction-table.yml
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
commented out. 46 distinct verbs are used. **110 of the 203 — 54% — are `lookat`.**

| | count | share |
|---|---|---|
| exactly one statement | 121 | 59.6% |
| …and that statement is a `sayLine` | 91 | 44.8% |
| contains a conditional | 62 | 30.5% |
| contains a blocking or timed call | 34 | 16.7% |
| calls another verb slot on the same object | 18 | 8.9% |
| starts a thread | 12 | 5.9% |
| contains a loop | 7 | 3.4% |
| **free of loops, threads, blocking and delegation** | **155** | **76.4%** |

The 155 call 28 distinct functions between them, and the census is short: `SAY` 219, `sayLine`
180, `ONCE` 22, `objectState` 9, `exitRoomFromDoor` 8, `pickupObject` 7, `inInventory` 4,
`removeInventory` 3, and a tail of sound, animation and camera calls.

**Objects.** 164 `DEFINE_OBJECT` declarations. **46 (28%) carry no verb slot at all** — a name,
an article, some properties, and nothing that happens. 82 carry exactly one. Two carry five or
more. Alongside them, 290 bare props (`name = {}`): positioned art with no name and no
behaviour.

**Exits.** 20, and **every one of them is a declaration**: `to_door = @Room.doorObject` plus
`door_verbs = YES`, with no code. The engine synthesises the verbs — `_initObject` "sets up door
verbs if `object.door_verbs` is set". The two door objects that carry any verb slot carry
`lookat` only. **No exit in the game is ever gated**: no `to_door` object is made untouchable or
hidden anywhere.

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
of which four pages were read. And `Scripts/Inventory.dinky`, read only as a source of binding
counts.
