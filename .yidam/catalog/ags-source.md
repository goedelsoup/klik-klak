---
name: Adventure Game Studio source code
description: AGS's own editor and engine source, which still contains the complete action table of the Interaction Editor that version 3.0 removed.
type: other
obtained: true
retrieved: 2026-09-10
ttl_days: 365
location:
  - kind: url
    value: https://github.com/adventuregamestudio/ags
    description: the project's repository; read at master, commit 72e4c72a722a (2026-09-09), latest release v3.6.2.21
  - kind: url_template
    value: https://raw.githubusercontent.com/adventuregamestudio/ags/master/{path}
    description: raw file by repository path
---

**This is a source of record and it is the strongest one this corpus holds.** It is the AGS
project's own code, in the AGS project's own organisation, under the Artistic License 2.0, with a
copyright line reading "Copyright (C) 1999-2011 Chris Jones and 2011-2026 various contributors".
Where the manual describes the authoring surface, this defines it. Claims resting on it are
`[verified]`.

It is also a **living** source — master moves — which is why the commit is pinned in `location`
above and the TTL is one year rather than ten. A claim about what the code contains is a claim
about that commit.

The reason to read it is specific and was not obvious: **the code that converted 2.x games to
3.0 is still in the editor**, which means the complete definition of the layer 3.0 deleted is
still shipping. `.yidam/catalog/ags-272-manual.md` describes the Interaction Editor from the
outside and could not enumerate its commands; this entry enumerates them.

## What was read

Four files.

- **`Editor/Native/ac/actiontype.h`** — the schema. `#define NUM_ACTION_TYPES 48`. Each action
  carries a name, flags, an argument count, argument types and names, a description, and a
  `textscript` template. Argument types are declared: `ARG_TYPE_INT`, `ARG_TYPE_INV`,
  `ARG_TYPE_MSG`, `ARG_TYPE_CHAR`, `ARG_TYPE_BOOL`, `ARG_TYPE_VARONLY` ("must be variable, no
  literal values"). Flags are `AFLG_COND`, `AFLG_RUNSCRIPT`, `AFLG_INVCHECK`, `AFLG_MESSAGE` —
  **the schema marks which actions are predicates.**
- **`Editor/Native/ac/actiontype.cpp`** — the table itself, all forty-eight entries, **recorded
  in full below**. The shape is `Category - Verb`, and the categories are Game, Player, Object,
  Character, Room and Conditional, plus three uncategorised entries: `Do nothing` (0),
  `Run script` (1) and `Stop running more commands` (41). This entry said for six phases that
  the table was recorded in `data-format/interaction-table.yml`; that node carried the seven
  predicates and no table. `.yidam/decisions/forty-eight.yml`.
- **`Common/game/interactions.h`** — the structure and its bounds.
  `MAX_ACTION_ARGS 5`, `MAX_NEWINTERACTION_EVENTS 30`, `MAX_COMMANDS_PER_LIST 40`. An
  `InteractionCommand` carries a type, its arguments, and **`UInterCmdList Children` — a list of
  sub-actions**, so the structure is a tree rather than a list. An argument
  (`InteractionValue`) is typed at runtime as one of `kInterValLiteralInt`, `kInterValVariable`,
  `kInterValBoolean`, `kInterValCharnum`, so an argument may be **a literal or a variable**.
- **`Editor/AGS.Native/agsnative.cpp`**, `ConvertInteractionToScript` and
  `ConvertInteractionCommandList` — the translation, which is where the semantics are legible
  rather than merely the vocabulary. A command with children emits its `textscript`, then the
  child list, then a closing brace: **a conditional guards the block nested under it.** Consecutive
  `AFLG_INVCHECK` commands are chained with `else`. And where an event's list is entirely
  inventory checks, the converter appends `else { unhandled_event(<type>, 3); }` — **the editor
  had an implicit default refusal, and it is visible only in the code that removed it.**

  Two commands translate to more than a call. `Add score on first execution` becomes
  `if (Game.DoOnceOnly("<guid>")) { GiveScore(n); }` — once-only was a property of an action
  rather than something an author arranged. `Character - Quick animation` expands to a
  lock/animate/unlock triple.

  Blocking is visible here too, as an argument rather than a convention: `Object - Move object`
  and `Character - Move character` each take a `BOOL` the editor labelled "Wait for move to
  finish", translated to `eBlock`/`eNoBlock`; `Character - Run animation loop` is `eBlock`
  unconditionally; `Game - Pause command processor for a set time` is an explicit wait.

Also read and not used: `Common/game/interactions_deprecated.h`, which holds a *different* and
older vocabulary — fifteen commands from AGS v1.00, including `MAN_DIES`, `GRAPHSCRIPT` ("run
graphical script") and `RUNSCRIPT` at index 9. It is guarded by `#if defined (OBSOLETE)` and its
header says "Perhaps move to some legacy knowledge base". Recorded here as evidence that the
escape hatch predates everything else in this history — it was present in the first version —
and not cited, because nothing here has established how v1.00's list relates to the 48.

## What was looked for and not found — read this before citing

**Why the Interaction Editor was removed is not in the code and was not found anywhere.** The
code says what the thing was and says nothing about the decision. Three retrievals were made for
the reason and none produced it: the Adventure Classic Gaming interview with Chris Jones, which
covers AGS 3.0 as "a complete re-write of the editor" and does not mention the Interaction Editor
at all; and two searches of the AGS forums, which surface the manual's removal note and no
discussion behind it. The AGS forum's own search, and the 2007-2008 beta threads, have not been
read.

That question stays `[open]` in `data-format/interaction-table.yml`, and the reason it is worth
keeping open rather than settling by inference is that the vocabulary read here is *large* — 48
commands, seven of them predicates, with a type system for arguments. A removal is not obviously
explained by the thing being thin.

## What this repository holds that nobody here has looked at

Almost all of it: the engine, the script compiler, the room and game file formats, and the
editor beyond the four files above. The script compiler in particular would answer what the
language actually is, where this corpus currently relies on the manual's description.

## The action table

All forty-eight entries of `NUM_ACTION_TYPES`, in table order, at the commit pinned in
`location` above. **#** is the action's index, which is its on-disk encoding; **args** is its
declared argument count.

The last column is this corpus's reading rather than AGS's, and it partitions the table four
ways. *predicate* is the seven carrying `AFLG_COND`. *control* is the three that act on the
command list itself. *presentation* is an action that plays, shows, animates, faces, follows or
waits — it writes nothing a predicate in this design could read. Everything else names the
effect of [the `.game` surface](../../crates/game-format/src/lib.rs) it maps onto.

**Twenty-three of the thirty-eight real actions are presentation or timing**, and all fifteen
that write world state map onto the eight effects that surface already has.
`.yidam/decisions/forty-eight.yml` carries what follows from that.

| # | action | args | this design |
|---|---|---|---|
| 0 | Do nothing | 0 | *control* |
| 1 | Run script | 0 | *control* |
| 2 | Game - Add score on first execution | 1 | `set-counter` |
| 3 | Game - Add score | 1 | `set-counter` |
| 4 | Game - Display a message | 1 | *presentation* |
| 5 | Game - Play music | 1 | *presentation* |
| 6 | Game - Stop music | 0 | *presentation* |
| 7 | Game - Play sound effect | 1 | *presentation* |
| 8 | Game - Play Flic animation | 2 | *presentation* |
| 9 | Game - Run dialog | 1 | `enter` |
| 10 | Game - Enable dialog option | 2 | `set-flag` |
| 11 | Game - Disable dialog option | 2 | `clear-flag` |
| 12 | Player - Go to a different room | 2 | `move` |
| 13 | Player - Give the player an inventory item | 1 | `give` |
| 14 | Object - Move object | 5 | *presentation* |
| 15 | Object - Remove an object from the room | 1 | `clear-flag` |
| 16 | Object - Switch an object back on | 1 | `set-flag` |
| 17 | Object - Set object view number | 2 | *presentation* |
| 18 | Object - Start object animating | 4 | *presentation* |
| 19 | Character - Move character | 4 | *presentation* |
| 20 | Conditional - If inventory item was used | 1 | *predicate* |
| 21 | Conditional - If the player has an inventory item | 1 | *predicate* |
| 22 | Conditional - If a character is moving | 1 | *predicate* |
| 23 | Conditional - If a variable is set to a certain value | 2 | *predicate* |
| 24 | Character - Stop character walking | 1 | *presentation* |
| 25 | Player - Go to a different room (at specific co-ordinates) | 3 | `move` |
| 26 | Character - Move NPC to different room | 2 | `move` |
| 27 | Character - Set character view | 2 | *presentation* |
| 28 | Character - Release character view | 1 | *presentation* |
| 29 | Character - Follow another character | 2 | *presentation* |
| 30 | Character - Stop following | 1 | *presentation* |
| 31 | Room - Disable hotspot | 1 | `clear-flag` |
| 32 | Room - Enable hotspot | 1 | `set-flag` |
| 33 | Game - Set variable value | 2 | `set-flag, set-counter` |
| 34 | Character - Run animation loop | 3 | *presentation* |
| 35 | Character - Quick animation | 4 | *presentation* |
| 36 | Character - Set idle animation | 3 | *presentation* |
| 37 | Character - Disable idle animation | 1 | *presentation* |
| 38 | Player - Remove an item from the inventory | 1 | `spend` |
| 39 | Game - Show GUI | 1 | *presentation* |
| 40 | Game - Hide GUI | 1 | *presentation* |
| 41 | Stop running more commands | 0 | *control* |
| 42 | Character - Face location | 3 | *presentation* |
| 43 | Game - Pause command processor for a set time | 1 | *presentation* |
| 44 | Character - Change character view | 2 | *presentation* |
| 45 | Conditional - If the player character is | 1 | *predicate* |
| 46 | Conditional - If mouse cursor mode is | 1 | *predicate* |
| 47 | Conditional - If the player has been in room | 1 | *predicate* |
