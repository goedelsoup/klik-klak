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
  - ../corpus/interaction/default-binding.yml
  - ../corpus/interaction/unary-binding.yml
  - ../corpus/data-format/room-manifest.yml
  - ../corpus/walkbox/scaled-polygon.yml
  - ../corpus/room/scrolling.yml
  - ../corpus/interaction/entry-binding.yml
  - ../corpus/interaction/binary-binding.yml
  - ../corpus/subsystem/interaction-resolver.yml
  - ../corpus/inventory-item/portable-object.yml
  - ../corpus/prior-art/ags.yml
  - ../corpus/prior-art/scumm.yml
  - ../corpus/verb/use-with.yml
  - ../corpus/interaction/binary-binding.yml
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

**Objects.** 164 `DEFINE_OBJECT` lines, of which **four are commented out** — the live figure is
**160**, and the recount is under "The room geometry" below. **45 of the 160 (28%) carry no verb
slot at all** — a name, an article, some properties, and nothing that happens. 82 carry exactly
one. Two carry five or more. Alongside them, 290 bare props (`name = {}`): positioned art with no
name and no behaviour. The share is unchanged by the recount; both halves of it were inflated by
the same four lines.

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
  interactable at all — which **45 of the 160 objects** are.
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

**How much of that list the game uses.** 46 distinct verbs carry a binding on an object and 49 do
counting actors, so **8 of the 57 are declared and never used**. Of the 49, **23 are used exactly
once** — `lick`, `dissolve`, `jab_with`, `scrape`, `tighten` among them. Comment-stripped and not,
the figures are the same.

**There is no generic "use with" in it.** No `VERB(use)` appears anywhere in `Rooms/` or
`Inventory.dinky`; the only `verb_use` in the game is the one `door_verbs = YES` synthesises for
going through a door, which is unary. Every binary interaction is a *named* verb — `unlock_with`,
`jab_with`, `puton`, `scrape` — so this game kept arity and multiplied verbs rather than routing
them through one. That is the opposite of the reduction
`.yidam/decisions/binary-dimension.yml` was testing, and further from it than this corpus's own
six-verb design.

**All eighteen binary verbs are used, and the two the object census misses are on actors.**
`show` and `takefrom` carry no binding on any object; `Defines/DefineActors.dinky` has one and
four. Counting actors as well, the game is 221 bindings of which 32 take a second object — 14.5%.

**The cell count, which is the thing the arity question is actually about.** Both verb lists are
57 entries and they name the same set — `master_verb_list` and `master_verb_names`, checked against
each other at boot — with 18 of the 57 in `verb_usewith_list`, leaving 39 unary. `Inventory.dinky`
declares **15 items** with `DEFINE_INVENTORY`, none commented out.

So the authorable space is not objects × verbs. A unary verb offers one cell per object and a
binary verb offers one per (object, item):

    164 x 39            =  6,396 unary cells
    164 x 18 x 15       = 44,280 binary cells
                          ------
                          50,676, with 203 authored — 0.40%

Counted flat, as `.yidam/decisions/default-binding.yml` counted it, that is 164 × 57 = 9,348 and
2.17%. The correction is five-fold and runs in the direction of that record's own argument: the
sparser the table, the more a default would have to carry.

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

**Preconditions exist in the vocabulary and the game authors sixty-two of them.** `VERB_OK($a)`
declares `okverb_<verb>`, a predicate the engine consults before offering a verb. It has
engine-supplied defaults for declarative doors in `SystemHelpers.dinky` — a door may be opened
only if `state != "open"`, closed only if `state == "open"` — **and 62 live hand-written sites
besides**, 45 in `Rooms/`, 11 in `Inventory.dinky`, 6 on actors, with a 63rd commented out in
`Bathroom.dinky`. `Nickel.dinky`'s file drawer carries four: `VERB_OK(lookin) { return state ==
"open" }` and three like it.

This entry said for three phases that there were none, and the cause is recorded because it is
the second time: the search was for `okverb_`, which is what the **macro expands to**, and the
authors write the macro. `okverb_` appears in four engine files and no game file. It is the same
mistake as searching for `triggerExit` in an engine that spells it `triggerLeave` — a spelling
this source does not use, and a zero-hit search read as an absence.

**Triggers.** `Templates/TRIGGERNAME.dinky` declares `triggerEnter(actor)` and
`triggerLeave(actor)`. The game uses 11 and 13 of them across six rooms — arrival at, and
departure from, a region *within* a room.

**Room entry code.** 20 of 21 rooms have `enter = function(from_door=null)`; 237 lines between
them, mean 11.8, and three rooms hold 36-44 of them.

## The room geometry — 2026-09-10

The 22 `.wimpy` files, read in full for the first time. `HelpDocsHtml/wimpy.html` is the single
word "TDB", so the format is undocumented and everything below is read off the files themselves —
the same standing as Yack's grammar.

**It is not JSON, and this entry said it was.** `room-manifest` records Thimbleweed Park's rooms
as "written as `.JSON` by a standalone editor called Wimpy", which is what the development diary
quotes for the 2017 game. Three years later the same editor writes something else: bare
`key: value` lines with no enclosing braces and no commas, arrays whose elements are separated by
newlines, tuples written `(320,180)` and `((0,0),(320,180))`, and a **legacy vector-as-string**
encoding `"{320,180}"` that survives in some fields and not others. 40 distinct keys across the
corpus of files. A JSON parser reads none of it.

**948 objects, and the shape vocabulary is three exclusive flags over two orthogonal fields.**
`prop`, `spot` and `trigger` are each `1` or absent, and **no object carries more than one** — 0
of 948. Crossed with whether the object has a `hotspot` rectangle and whether it has `animations`,
every object in the game falls into six cells:

| shape | count | what it is |
|---|---|---|
| `prop` + animations | 455 | drawn, not targetable — scenery |
| `hotspot` rect only | 143 | targetable, not drawn |
| `hotspot` + animations | 133 | targetable and drawn |
| `spot` | 163 | neither — a position and a facing |
| `trigger` + `hotspot` | 48 | a region that fires on entry |
| `prop`, no animations | 6 | drawn from the sheet with no state |

**The rectangle is a separate field from the drawing, and the two barely overlap.** 598 objects
are drawn, 324 carry a hotspot rectangle, and only 133 do both. Targetability is not a property of
the art and is not derived from it: it is its own rectangle, authored separately, and 143 regions
have one with nothing drawn under them at all.

**A trigger is a hotspot with a boolean.** All 48 carry exactly the same seven keys — `name`,
`trigger`, `hotspot`, `pos`, `usepos`, `usedir`, `zsort` — which is a hotspot's key set plus the
flag. None has an animation. The engine types them `OBJECT_TRIGGER` and `_enterObject` calls
`addTrigger(object)` on them, then `enableTrigger(object, TRIGGER_ENTER|TRIGGER_LEAVE)`.

**A spot is `usepos`/`usedir` promoted to an object.** All 163 carry exactly four keys — `name`,
`spot`, `usedir`, `pos` — and nothing else. They are the only objects with a `usedir` and no
`usepos`, and they are exactly that set.

**Every targetable region has an approach point and only targetable regions have one.** `usepos`
appears on exactly the 324 objects with a hotspot rectangle. 64 of them carry a **second** one,
and the nesting is perfect: no `usepos2` without a `usepos`, no `usedir2` without a `usepos2`. The
manual confirms it is an engine concept rather than a convention — `objectUseDir2(object)`
"Returns the secondary use direction, as defined in Wimpy".

### Walkboxes

**84 polygons across 15 rooms, and a walkbox carries nothing but a polygon and sometimes a name.**
Those are the only two keys that ever appear on one. Vertex counts run 4 to 118, median 5, mean
9.5; **42 of the 84 are quadrilaterals**.

**37 of the 84 are not convex.** Tested by cross-product sign change around the ring, ignoring
collinear vertices.

**No walkbox carries a scale.** The scale is a room-level field: `scaling` is a list of
`"<factor>@<y>"` breakpoints — `["0.90@54", "0.80@93"]` — present in 12 of 22 rooms, and it is a
piecewise function of the actor's **y coordinate in the room**, not of which region the actor
stands in. MainStreet's is a list of five alternative curves each keyed by a `trigger` name, so
the scale function is state-dependent and the state is a trigger region.

**14 of the 84 are named, and the names come in pairs.** `hydrant` / `hydrant_front`, `jail` /
`jail_door`, `natalie_desk` / `desk_path`, plus `upperLadder0..2`, `lowerLadder0..2`, `backroom`
and `stairs`. `walkboxHidden(name, state)` is called at 11 sites, every one naming a named box;
the 70 unnamed ones are never hidden. In each pair the larger region is non-convex and its partner
is a 4-vertex quad.

### Extent and camera

**A room declares its size and its camera range as two separate fields, and they differ in ten of
twenty-one.** `room_size` is the image — the manual says `roomSize(room)` "Returns the size of a
room based on it's image size" — and `room_bounds` is a rectangle authored beside it. Six of the
differences are an origin of `(1,1)` against `(0,0)`. Four are real: Diner is inset ten pixels on
every side (`((10,10),(470,170))` against `(480,180)`), Vista and Nickel are inset at the right
edge, and **Bathroom's bounds are taller than its image** — `(320,240)` against a `(320,180)`
background. `Inventory.wimpy` has bounds and no size at all.

### What round-tripping through the tool costs

**`roomsize` is a dead key that survives in seven of the 22 files.** Six carry it near the top in
the legacy string encoding (`roomsize: "{320,180}"`); PostOffice carries it at the bottom as a
tuple. Every file also carries `room_size`, and in all seven the two agree — the dead key has
never drifted, because nothing writes it. The tool rewrites these files and does not remove it.

**The hand-written template is stale against the tool.** `Templates/ROOMNAME.wimpy` writes
`room_bounds: "{{0,0},{428,240}}"` — the string encoding — and no file the editor has touched uses
that form for that key. The template also has no `room_size` and no `walkboxes`.

**One key is written two ways within one file kind.** `parallax` on a layer is a scalar in 34 of
39 layers and a tuple `(1.2,1.2)` in 5. `Diner.wimpy` carries a top-level `height: 128` that no
other file has.

### The geometry against the scripts

Cross-referencing every `.wimpy` object against the room scripts that declare and bind it. The
reader is held to this entry's own figures: it reproduces 203 live `VERB` bindings and 107 of them
`lookat`, and refuses to report anything if it does not.

**The 164 object declarations recorded above is a raw count and four of them are commented out.**
`AStreet.dime`, `Nickel.newspaper17`, `QuickiePal.tShirt` and `QuickiePal.rat` are commented-out
`DEFINE_OBJECT` lines. The live figure is **160**, and it is the same error as the `lookat` recount
in a different key. Every figure below uses 160.

**202 of the 203 bindings are written inside an object declaration. The other one is installed on
an object passed as an argument** — `obj.VERB(read) { … }` inside `assignHeadline(obj, headline)`
in `AStreet.dinky`.

**A room script names an object in three ways, and the geometry declares more than it names.**
A `DEFINE_OBJECT({…})` attaches behaviour, a `DEFINE_INVENTORY` does the same for a carried item,
and a bare `name = {}` is a stub that names a geometry object and gives it nothing — 290 of those,
the "bare props" counted above. Against the 948:

| geometry shape | total | named by the script | carries a binding | never named |
|---|---|---|---|---|
| `hotspot` (not a trigger) | 276 | 170 | **115** | 106 |
| `prop` | 461 | 289 | 0 | 172 |
| `spot` | 163 | **0** | 0 | **163** |
| `trigger` | 48 | 26 | 0 | 22 |
| **total** | **948** | **485** | **115** | **463** |

**Of the 276 clickable regions, 115 carry a binding — 41.7%.** 55 are named by the script and bind
nothing; **106 are never mentioned by it at all.** That is the same question as "46 of 164 carry no
binding" above asked from the other side of the split, and the answer is much larger, because the
164 was every object the *script* names and the geometry declares 948.

**The 163 spots are named by the script exactly zero times.** They are read only by the engine —
`objectUseDir(object)`, "as defined in Wimpy" — and no author's code refers to one. A declarative
field doing real work with no script surface at all.

**Every name in a room script resolves to an object in that room's geometry.** The only script
names without one are the fifteen inventory items, which are in no room. There are no orphans in
that direction; all 463 are on the geometry side.

**`Nickel.newspaper17` is a fully-formed targetable region with no live declaration anywhere.** It
has a hotspot rectangle, a `usepos`, a `usedir` and a `zsort`, sitting between `newspaper16` and
the end of the object list; the only line in the scripts that mentions it is a commented-out
declaration. Finding it means reading the two files together, and they are the two files the
engine deliberately keeps apart.

**Departure is offered at both levels and used at only one of them.** `enableTrigger` takes
`TRIGGER_ENTER|TRIGGER_LEAVE` and `_triggerActor` dispatches to `triggerEnter` or `triggerLeave`.
Across the game there are **11 `triggerEnter` bodies and 13 `triggerLeave` bodies**, in six rooms —
so at the region level departure is written *more often* than arrival.

**This entry recorded that figure as zero, and the same entry already held the right one.** The
count above under "Yack" says "11 and 13 of them"; a later reading grepped for `triggerExit`, a name
this engine does not use, and reported no departures at all. It is the third time a figure in this
file has disagreed with itself — after the `lookat` recount and the object-declaration recount — and
the first where the correct value was sitting in the same document.

**`assignHeadline` is one binding shape written 28 times.** The function is defined twice —
`Nickel.dinky:100` and `AStreet.dinky:62`, near-identical, driving two headline tables — and each
copy is applied to a run of one-line declarations: sixteen `newspaper1..16 = DEFINE_OBJECT({
function enter() { assignHeadline(this, N) } })` in Nickel, twelve `assignHeadline(nickelNoteN, N)`
calls in AStreet. 28 geometry objects, 28 declarations differing only in an integer, two copies of
one function. `addTriggerCode` in BookStore is the same shape at six.

### What the manual says about touchable and hidden

`object_functions.html` documents `objectHidden` and `objectTouchable` with **the same sentence**:
"Sets or returns an objects hidden state. Objects that are hidden are also untouchable." The
second is a copy of the first, wrong for the function it describes.

The same page gives the coupling between drawing and targetability a magic value:
`objectState(object, state)` — "If the state is set to `"gone"` then the object becomes
untouchable."

## Availability against refusal — 2026-09-10

This engine spells two things where this corpus's design spells one, and it spells them **on the
same verb slot of the same object**.

`Bathroom.dinky`'s `sink` carries all three of:

    VERB_PRE(turn_on) { if (sinkFaucet.broken) { sayLine(SAY(11501,"This has been broken for
                        as long as I can remember.")); return NO } return YES }
    VERB_OK(turn_on)  { return (sinkFaucet.state == "off" && lights_on) }
    VERB(turn_on)     { sinkFaucet.verb_turn_on() }

Three conditions on one verb, given three treatments the author chose between. **Already on — not
offered, silently. Room dark — not offered, silently. Broken — offered, and refused with a line.**
`sinkFaucet` repeats the whole arrangement for itself, which is a fifth instance of the
sink-and-its-tap delegation this entry already records.

**`VERB_OK` cannot carry a line and does not.** It returns a boolean the engine consults before
offering the verb. Across all **62** live sites, not one contains a `sayLine` or a `SAY`. What the
62 test, by inspection of each body:

| what the predicate reads | sites |
|---|---|
| which second object (`noun2 == …`) | 30 |
| the target's own state (`state == "open"`, `broken`, `position`, `eaten`) | 36 |
| the environment — `lights_on`, `selectedActor.on_ladder` | 19 |
| neither, or something else | 4 |

(The categories overlap; 11 read state and object, 5 read all three.) **Thirty of the sixty-two say
which item the verb applies to** — `noun2 == Inventory.glass`, `noun2 == leonard`,
`noun2?.accepts_mail` — which is what a second key says in a format that has one. Only **32 of the
62** test something no key could express.

**`VERB_PRE` is the other half and there are four.** Three are the refuse-with-a-line shape:
`Nickel.door`'s *"I should find the camera before setting out"* and the two broken-faucet lines. The
fourth, `Inventory`'s `takephoto`, is not a refusal at all — it starts a thread and returns `NO`,
replacing the verb rather than declining it.

So the engine's division is: `VERB_OK` withholds the verb and says nothing, 62 times; `VERB_PRE`
offers it and declines out loud, 3 times. **Twenty to one in favour of silence** — which is the
reverse of the ratio in this corpus's own specimens, where every one of seventeen gated bindings on
a hotspot verb carries a line.

## Frame width — 2026-09-10

A different cut through the same 287 authored verb slots, taken to answer a question about *this*
design: how much of what an author types is theirs? **Frame** is every non-space character
outside a string literal; **prose** is what is inside one. The ratio is what the author gets back
for what they had to write.

Comments stripped, braces matched:

| slot | sites | frame/site | prose/site | prose : frame |
|---|---|---|---|---|
| `VERB(x)` on an object | 203 | 85.1 | 52.1 | 0.61 |
| `VERB(x)` on an actor | 18 | 74.0 | 54.9 | 0.74 |
| `VERB_OK(x)` | 62 | 34.2 | 0.8 | **0.02** |
| `VERB_PRE(x)` | 4 | 59.2 | 46.5 | 0.78 |
| all | 287 | 73.1 | 41.1 | 0.56 |

**The reader reconciles exactly, which is why the figures above can be trusted.** Excluding
`Defines/DefineActors.dinky` it reproduces all three of this entry's pinned counts — **203
bindings, 46 distinct verbs, 107 live `lookat`** — and that reconciliation is itself a small
correction: the 203 is *bindings on objects*, and the game has **221**, the other 18 being on
actors, 11 of them `talkto`.

By verb, and the shape of the distribution is the finding: `lookat` 108 sites at 1.35, `talkto`
11 at 0.77, `walkto` 11 at 0.36, `read` 10 at 1.12, `pickup` 6 at 0.37, `open` 5 at 0.43. The
commonest slot is the most prose-dense and everything mechanical is thin.

**`VERB_OK` at 0.02 is the largest prose-free repetition in the game** — 62 sites, about 2,100
characters of frame carrying fifty of writing. It is a predicate and returns a boolean, so it
*cannot* carry prose: a failing `okverb_` removes the verb from the menu and says nothing, as
"The verb menu, and why there is no default binding" above records.

**`assignHeadline` is what the authors did about it, and the split is clean.** `AStreet.dinky`
declares twelve newspapers by calling one function twelve times. Measured the same way: the
`_noteHeadlines` table is 47 lines at **2.13**, the function is 17 lines at **0.01**, and the
twelve call sites are 12 lines at **0.00**. The factoring took the frame and left every word of
the writing where it was.

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

## The conversation mode, read from the API — 2026-09-10

`yacks.html` calls its own description of the *format* "TBD" but documents the *engine surface* in
full, and what it documents is a mode rather than a place.

**Entering takes no room and moves nobody.** `startYack(actor1, actor2, filename)` — "Starts a
.yack file conversation between 2 actors"; the filename may carry a label, and without one the
engine jumps to `start`, which is required. Either actor may be null. **Leaving takes nothing at
all**: `endYack()` — "Conversations naturally end, but there might be a need to end it outside the
conversation .yack file." Neither call names a location, because the player never left one.

**The mode is one global boolean.** `isYacking()` — "Returns true if a yack conversation is
happening." Beside it `yackActor1()` and `yackActor2()` name the two parties, and
`areChoicesShowing()` says whether the list is up and waiting. So the exchange is a bit and two
actor references laid over the room the player is standing in, not somewhere the player has gone.

**Availability is counted while the list is being drawn.** `yackChoices()` — "Returns the number of
yack choices showing… the number of choices *at the moment it is called*… It is not the total
choices to be shown, just the number up to that point. It is most commonly called from .yack
scripts to find out if there are **no active choices**." A `.yack` script asks, mid-render, how many
of its own options survived their guards. That is availability derived at display time, in an
engine whose guard layer is the 141 bracket expressions above.

The rendering callbacks say the same from the host's side: `_beginYackChoices`, `_addYackChoice`,
`_showYackChoices`, `_endYackChoices`, rebuilt per display. `_addYackChoice(index, text, label)`
notes that `label` is "no longer used" — the engine once routed a choice by its jump target and now
passes the index back instead.

**And a mode cannot nest inside another mode.** `cutscenes.html`: "If a `cutscene()` is started from
within a `startYack()`, the yack will immediately end (with no error)", and "If an override is
triggered during a Yack conversation in a `cutscene` the Yack will be killed."

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

The `.wimpy` files have since been read in full — see "The room geometry" above — and what they
turned up is that the two halves of the split had never been counted against each other. What is
still unread there is the **lighting**: 61 light declarations carrying `color1`, `color2`,
`iradius`, `oradius`, `rate` and a `clipping` rectangle, which is presentation and was skipped for
that reason, and the 39 `layers` with their parallax rates.
`Scripts/Helpers/SystemHelpers.dinky` (1,258 lines) and
`Options.dinky` (659), which are where the authors' own abstractions live. The 34-page manual,
of which four pages were read. And `Scripts/Inventory.dinky`, read for binding
counts and for its one delegation site, and not otherwise.
