---
name: Adventure Game Studio manual (source)
description: The AGS manual's own markdown source, maintained by the AGS project — the authoring surface documented by the people who built it.
type: other
obtained: true
retrieved: 2026-09-09
ttl_days: 365
location:
  - kind: url
    value: https://github.com/adventuregamestudio/ags-manual-source
    description: the manual's source repository, which is the copy this corpus read
  - kind: url_template
    value: https://raw.githubusercontent.com/adventuregamestudio/ags-manual-source/master/{page}.md
    description: raw markdown for a named page — EditorCursor, Dialog, and the rest
  - kind: url
    value: https://adventuregamestudio.github.io/ags-manual/
    description: the rendered manual; JavaScript-driven, so a plain fetch returns an empty page
---

**This is a source of record and it is the only one this corpus holds.** The AGS manual is
written and maintained by the project that builds AGS, documenting its own authoring surface.
Claims resting on it may be tagged `[verified]`.

Read the *source repository* rather than the rendered site. The rendered manual at
`adventuregamestudio.github.io` is assembled client-side, so fetching a page returns an empty
document — two attempts against `EditorCursor.html` and `Dialog.html` came back with no
content at all, which is a failure that looks exactly like a page with nothing on it. The raw
markdown under `ags-manual-source` is the same text and is retrievable.

## What was actually read

Nineteen pages. The count is kept current because this entry is read to decide what is
citable, and a stale one understates what is available — it said "two pages, and only
two" while listing eight, for six of which it was the only record.

- **`EditorCursor.md`** — cursor modes. Names Mode 0 (Walk), Mode 4 (Use Inventory), Mode 6
  (Pointer) and Mode 7 (Wait); refers to modes 1, 2, 3, 5, 8 and 9 without naming them on this
  page. States that an author "may create more if necessary", and that the `StandardMode`
  property marks a mode that fires an event on whatever is clicked — applying to Walk, Look,
  Interact and Talk, and explicitly *not* to Use Inventory.

  **Re-read for movement, and it carries an exception this entry had twice failed to record.**
  Two bullets, at the foot of the page: "For the standard modes, Mode 0 will cause the player to
  walk to the mouse pointer location when clicked. Modes 1, 2, 3, 5, 8 and 9 will run the event
  with the same name as the cursor mode."

  So Walk is named among the standard modes in one paragraph and exempted, in the next, from the
  thing that makes a mode standard: every other one dispatches to an event of its own name, and
  Mode 0 walks. `verb/walk-to.yml` rests on this.
- **`Dialog.md`** — dialog option state. `Dialog.SetOptionState()` with `eOptionOff` ("the
  option is disabled - the player will not see it") and `eOptionOffForever` ("the option is
  permanently disabled - no other command can ever turn it back on again"), equivalent to the
  `option-off`, `option-on` and `option-off-forever` dialog script commands. Also
  `HasOptionBeenChosen()` and `SetHasOptionBeenChosen()`.
- **`Character.md`** — inventory. `AddInventory` "adds one quantity of the specified item to the
  character's inventory… ensures that the item gets added to the character's inventory list, and
  that any on-screen inventory display gets updated if appropriate." `LoseInventory` and
  `ActiveInventory` likewise. **The manual does not describe any built-in pick-up verb, mode or
  event, and does not say what happens to a room object when an item is added.** Both absences
  were looked for specifically; see `verb/pick-up.yml`.
- **`EditorDialog.md`** — dialogs are first-class assets: "When creating a new dialog, or
  double-clicking on existing dialog within the tree, a new editor panel will open." `@S` labels a
  dialog's entry point. Conversation lines "are played in game using standard `Character.Say`
  command by default". **Does not** say how a dialog is started during play, or how a Talk
  interaction reaches one; both were looked for.
- **`DialogScript.md`** — the dialog sub-language, read once for conversation shape and **re-read
  in full** when a design premise came to rest on it. "Dialog scripts are streamlined for
  conversations and use an extended variant of script syntax that simplifies several common
  commands."

  The page introduces its command set as "The dialog commands available are:" and lists eight.
  All eight are recorded here with their descriptions verbatim, because the useful property of
  this set is not any single command but what the set as a whole does not contain:

  | Command | Documented as |
  |---|---|
  | `goto-dialog X` | "Switches the current topic to Topic X, and displays the current list of choices for that topic." |
  | `goto-previous` | "Returns to the previous topic that this one was called from. If the dialog started on this topic, then the dialog will be stopped." |
  | `option-off X` | "Turns option X for the current topic off, meaning it won't be displayed in the list of choices next time." |
  | `option-off-forever X` | "Turns option X off permanently. It will never again be displayed, not even if an 'option-on' command is used." |
  | `option-on X` | "Turns option X for the current topic on, including it in the list of choices to the player next time they are displayed." |
  | `run-script X` | "Calls dialog_request event handler if one is present in the game script, passes X as an integer parameter." |
  | `return` | "Stops the script and returns to the list of choices." |
  | `stop` | "Stops the conversation and returns the player to the game." |

  **Not one of the eight reads anything.** Four are control flow, three write option state, one
  is the escape, and none is a predicate — there is no comparison, no flag reference, no
  conditional and no count in the set. The page was searched on the re-read specifically for a
  conditional, for any variable reference, and for any way to read game state without leaving the
  sub-language; it documents none of the three.

  What the page does **not** do is claim the list is complete. It says *available*, and a reader
  wanting exhaustiveness has to get it somewhere else. The distinction is left standing here
  rather than smoothed over, because it is exactly the distinction this entry got wrong about
  cursor modes: the eight commands and their descriptions are quotable, and *"the sub-language
  has no conditional"* is a claim about the whole language that this page supports without
  settling.

  **The escape hatch is stated twice and the second statement is the informative one.** First as
  a concession: "Often the provided dialog scripting commands won't be enough... AGS now lets you
  put normal scripting commands in your dialog script, by indenting the line with spaces or
  tabs." Then as a warning attached to the dialog commands themselves: "Do NOT indent these lines
  with spaces or tabs. Indented lines signify that AGS should interpret the line as a normal
  scripting command."

  **Indentation switches languages**, and the warning is what shows the boundary is load-bearing
  rather than incidental — an author who indents a line by accident has silently changed which
  language it is written in, with no delimiter, no fence and no declaration anywhere to say so.

  `run-script X` is the other half of the escape, and it is weaker than a call. It does not run
  script *in place*: it "calls dialog_request event handler if one is present in the game script,
  passes X as an integer parameter", so the logic leaves the conversation entirely and reappears
  in a global handler switching on an integer tag.

- **`EventTypes.md`** — the event lists, and the page that should have been read before this
  corpus said anything about AGS's mode set. Characters have eight events: Look at, Interact,
  Talk to, **Pick up**, **Use inventory on**, Usermode1, Usermode2, Any click. Hotspots have the
  same eight plus Stands on and Mouse moves over.

  Two things follow and both matter.

  **AGS has a pick-up mode.** "Pick up character" and "Pick up hotspot" are documented events.
  `verb/pick-up.yml` asserted the opposite at `[verified]` on 2026-09-09 and that claim is
  withdrawn — see the `withdraw:` commit. The failure was this corpus's own and it was
  *predictable from this very entry*: the `EditorCursor.md` note above already recorded that six
  of ten modes were unnamed on the page that had been read, and the assessment asserted past the
  gap anyway.

  **There is no separate "give" event.** The manual describes "Use inventory on character" as
  "Occurs when the player uses an inventory object on a character. This event could be used to
  allow the player to give items to characters. You can use the `player.ActiveInventory` property
  to distinguish which item they used." Giving is using-an-item-on-someone, said by the system's
  own documentation.

  **Read a second time for movement, and it holds a whole family of events organized by position
  rather than by verb.** Rooms carry "Walk off left edge" and its three siblings — top, bottom,
  right. Hotspots carry "Stands on hotspot", whose handler the page names `hHotspot_WalkOn`.
  Regions carry "Walks onto region", "Walks off region", and an event that "occurs repeatedly
  while the player character stands on this region."

  None of them is a verb applied to a target. Every one is the engine reporting where the actor
  is, and together they are how a stock AGS game changes rooms. `verb/walk-to.yml` rests on this
  and on the `unhandled_event` note below.
- **`Globalfunctions_Event.md`** — `unhandled_event(int what, int type)`, "Called when an event
  occurs, but no corresponding event handler has been configured. This is typically used to
  display a default 'I can't do that' type response in-order to avoid having to add unique
  messages for all in-game interactions."

  The parameter table is a **verb × entity-type applicability matrix** and is the most
  information-dense thing this corpus has retrieved. `what` is the entity kind — 1 hotspot,
  2 object, 3 character, 4 **nothing**, 5 inventory — and `type` is the verb.

  Two readings fall out of it. **Look, Interact, Talk and Use-inventory each appear against all
  five entity kinds. Pick up appears against three** — hotspot, object, character — and has no
  entry for *nothing* or for *inventory*. And **"Look at nothing" (4|1) exists**: a verb applied
  to no target at all is a first-class event here, which this corpus's ontology has no way to
  express.

  **A third reading, for movement, and the finding is an absence that is safe to use.** The table
  has twenty-six rows and Walk is in none of them. What makes that usable rather than merely
  unrecorded is what the table *does* include: `1|8` "Cursor Mode 8 on hotspot" and `1|9`
  "Cursor Mode 9 on hotspot", with the same pair against objects and characters. Those are modes
  this manual declines to name anywhere, that an author invents. A table that gives a fallback row
  to an unnamed custom mode is not a table omitting modes for want of room, so Walk's absence is
  a statement: **there is no unhandled walk.** See `verb/walk-to.yml`.

  The page's closing note points the same way and is quoted here because it draws the boundary
  explicitly: "the 'Character stands on hotspot' event does not trigger this function, nor will it
  be triggered if there is an 'Any click' event handler defined, nor if the player clicks on
  nothing (hotspot 0)". Position-triggered events sit outside verb dispatch, by the manual's own
  statement rather than by inference from silence.

  The page **does not address** whether `unhandled_event` can be defined per room, or whether a
  default can be scoped to a hotspot or a verb rather than globally. Looked for; absent.

- **`Tumbleweed.md`** — a template shipping with AGS that "enables creation of classic adventure
  games in the AGS engine, specifically emulating SCUMM-style interfaces", to "recreate those
  classic SCUMM games, as well as the modern iteration Thimbleweed Park". It provides twelve
  actions: `eGA_LookAt`, `eGA_TalkTo`, `eGA_GiveTo`, `eGA_PickUp`, `eGA_Use`, `eGA_Open`,
  `eGA_Close`, `eGA_Push`, `eGA_Pull`, `eGA_UseInv`, `eGA_Default`, `eMG_WalkTo`. Rather than one
  function per interaction, an author writes a single "any click" handler and asks
  `Verbs.UsedAction()` which action fired.

  This is counter-evidence to a verdict this corpus reached one phase earlier and it is recorded
  here plainly: **`eGA_GiveTo` is a distinct action in the template**, alongside `eGA_UseInv`. See
  `verb/give-to.yml`.

  **Two further things, found on a re-read for movement.**

  *The twelfth constant is typed differently from the other eleven.* The list as the page gives
  it: "eGA_LookAt, eGA_TalkTo, eGA_GiveTo, eGA_PickUp, eGA_Use, eGA_Open, eGA_Close, eGA_Push,
  eGA_Pull, eGA_UseInv, eGA_Default and eMG_WalkTo". Eleven `eGA_`, one `eMG_`. **The page never
  says what `eMG_` denotes and this corpus does not know** — searched for, absent. What is on the
  page is the prefix. Do not expand the abbreviation; a plausible expansion is exactly the kind of
  guess this entry exists to prevent.

  *And the template's room transition is not an action at all.* Under "Exit Extensions":
  "Clicking on such a hotspot will make the player walk to it and change the room afterwards."
  It is declared by writing an extension into the hotspot's **description** — `Exit>e`, or `>el`,
  `>er`, `>eu`, `>ed` to continue off a screen edge — and dispatched by adding the `Usermode_1`
  hotspot event, which lands in a handler the page names `hExit_Mode8`. The page lists what it
  improves on: "the usual methods like 'screen edges' or stand-on hotspot functions".

  A template that was willing to add `eGA_GiveTo` to the action list put its exits in a
  description string and a user mode instead. See `verb/walk-to.yml`.

- **`UpgradeTo30.md`** — read for where the escape hatch sits in room and hotspot interactions,
  and it answers a question this corpus had been asking of the wrong document. **"The Interaction
  Editor has been removed from AGS 3.0. When you import your game, AGS will attempt to convert all
  your interactions into scripts."**

  So AGS had a declarative authoring layer for room and hotspot interactions and deleted it. The
  page states two things that changed, and neither is about expressiveness:

  > "Concurrency issues -- that is, while a blocking script was running in 2.72 it was still
  > possible for interactions to run at the same time if they didn't include any scripting. Now,
  > that is no longer the case."

  > "Blocking interactions -- in 2.72, a 'Run Dialog' interaction, for example, waits until the
  > dialog has finished before moving onto the next action. If you had a Run Dialog followed by a
  > Change Room, then the dialog would finish before the room change happened. With scripting, you
  > need to be careful because the dialog[x].Start() command does not block".

  And a third thing died with it: "Global Messages are no longer supported and should be
  considered obsolete -- **there's really no need for them now that the interaction editor has
  gone.**"

  **What the page does not say is why it was removed.** It reasons about migration hazards and
  never about the decision. It also opens by calling 3.0 "this new editor" and "a massive
  improvement over the old one" — 3.0 was a full editor rewrite, so the removal is consistent with
  a rewrite's scope and does not by itself demonstrate a verdict on declarative authoring. That
  distinction is load-bearing and is kept in `prior-art/ags.yml`.
- **`acintro3.md`** — the tutorial's hotspot chapter, and the page that shows where the boundary
  sits *now*. Clicking the "..." button on an event "will create an entry in the script for this
  event, and you'll find yourself taken to the script editor". The tutorial's first interaction is
  a function body:

      function hSignHome_Look()
      {
          Display("The sign says 'EWOH'. I have no idea what that could mean.");
      }

  Also here: the walk-to point is a hotspot *property*, and "whenever the player clicks interact
  or talk on the hotspot, the main character will first walk to the walk-to point before the
  relevant event is triggered." Movement is the automatic prelude to an interaction rather than
  something an author binds — see `verb/walk-to.yml`.
- **`Hotspot.md`** — the whole hotspot API, read to check whether any declarative facility
  survives. It does not. Sixteen members, all script: `GetProperty`, `SetProperty`,
  `RunInteraction`, `IsInteractionAvailable`, `Enabled`, `WalkToX`, `WalkToY` and the rest.
- **`CustomProperties.md`** — the one declarative surface that did survive, and its own motivating
  example says what it is worth. Characters, objects, hotspots, rooms and inventory items carry
  author-defined properties of type "Boolean", "Number" and "Text". The page opens by wanting a
  per-hotspot default verb — "if the player right-clicks a window, for example, 'Open' will be the
  default, but if they click on a pen, 'Pick up' will be the default" — and resolves it as: create
  the property, "and then have some simple code in the global script to use the setting
  accordingly." **Declarative data, script behaviour**, stated by the manual as the pattern.
- **`GlobalVariables.md`, `Globalfunctions_General.md`, `Game.md`** — read for the vestiges of the
  removed layer, which are still documented. `GetGraphicalVariable`/`SetGraphicalVariable` "allow
  your script to access the values of variables set in the interaction editor", example
  `GetGraphicalVariable("climbed rock")==1`; both marked obsolete, "only provided for backwards
  compatibility with AGS 2.x". `GlobalVariables.md` names them: "GlobalInts and Graphical
  Variables were ways in which previous versions of AGS provided global variable capabilities."
  `Game.GlobalMessages` and `DisplayMessage` are the message tables the editor displayed from;
  "Global Messages were a feature of AGS 2.x and are now obsolete."
- **`BlockingScripts.md`** — the page that exists because interactions became script. Three script
  threads, a queue of five, events lost when a thread blocks too long, and a worked example of
  event ordering that surprises authors. None of this was an authoring concern when a
  non-scripted interaction could run alongside a blocking script, which `UpgradeTo30.md` says it
  could.
- **`Scripting.md` and `ScriptingLanguage.md`** — the indexes, read to size the thing being
  escaped to rather than to cite it. Roughly sixty API pages and twenty language pages. For room
  and hotspot interactions this is not a hatch at the end of a declarative surface; it is the
  surface.
- **`RepExec.md` and `EventTypes.md`** — read for the **threshold trigger**, which this corpus had
  looked for and not found. It is here, and the page opens by naming the problem in exactly the
  corpus's words: *"One of the most common things you'll need to do when scripting is to check if
  something has happened in the game — and if so, then make the game do something in response."*

  The answer is `repeatedly_execute`, "run on every game loop (by default this is 40 times per
  second), but only when the game is not blocked", with `repeatedly_execute_always` and
  `late_repeatedly_execute_always` running regardless of blocking and forbidden from blocking in
  turn. The worked example on the page *is* a threshold: `hungerTimer++;` then
  `if (hungerTimer == 800) { Display(...); player.Animate(...); }`.

  **The occasion is declarative and the condition is not.** `EventTypes.md` lists "Repeatedly
  execute" in the room's event table beside "Enters Room After Fade In" — "Occurs repeatedly on
  every game cycle, at a frequency defined by the game speed… a equivalent of a global
  `repeatedly_execute` callback, as `repeatedly_execute` is not run in room scripts." So the slot
  is a declared event and what goes in it is arbitrary script. There is no declarative form of
  *when this value reaches that*, in either era of this system.

  `RepExec.md` also states the rule for which to use: `repeatedly_execute` "for doing things that
  affect the player character", `repeatedly_execute_always` "for background tasks that don't
  directly affect the player" — a division that exists only because responses have duration.
- Also read and not cited: `EditorRoom.md`, which mentions interactions nowhere, and
  `Globalfunctions_Message.md`.

## What a save file is, and what it costs an author — 2026-09-11

Three pages read on 2026-09-11: `GameSavesCompatibility.md`, `HandlingIncompatibleSaves.md`
and `ValidateRestoredSave.md`. They are on this corpus's subject and nobody here had opened
them; `world-flag.ont.yml` has declared a `save_semantics` property since genesis and all four
flag nodes answer it at `[inference]`.

**What the save contains.** *"AGS games have a built-in save system… these saves are essentially
'save states', meaning that they contain a 'dump' of all the game objects, regardless of whether
they are related to the game progress or not."* That is the sentence, and the engine's own manual
is the one making the distinction between state that is about progress and state that is not.

**How it is keyed, and what that costs.** *"When game data is written to a save file the game
objects and variables are written as lists, where separate items are not identified in any way
rather than an order in that list… Because of that, adding, removing or changing an order of
objects in game may make older saves incompatible, or completely unusable and impossible to
load."* The manual calls this *"one serious technical flaw"* and states the consequence for a
shipped game: *"it becomes a problem after you released your game to the public, as any update or
patch that involves adding new objects or variables will render the player's saves unstable or
unusable."*

**What breaks a save and what does not**, which is the part that reads as a statement about
authoring rather than about serialization:

| breaks older saves | safe |
|---|---|
| adding, removing or reordering Characters, Dialogs, Global Variables, GUIs and their controls, Inventory items, cursor modes, Views/loops/frames, script modules | adding rooms; adding room objects; editing backgrounds and area masks; adding or removing dialog options |
| changing the total size of variables declared at global scope in any script | **functions and attributes, and generally — function code itself**; macros; existing dialog scripts |

So a save is coupled to the **declaration list** and not to the code. An author may rewrite every
response in the game and keep every save; adding one inventory item invalidates all of them.

**The remedy is a script hook, which is the fourth time this manual has answered a declarative
problem that way.** Since 3.6.2 an author who has added anything *must* write
`validate_restored_save(RestoredSaveInfo* info)* — *"or the game will refuse to load older
saves"* — and the feature works only if new objects went on the **end** of their lists. The rules
are stated as prohibitions: *"Can't remove existing objects or variables. If you no longer need
any, then hide or disable them."*

That last sentence is word-for-word the conclusion this corpus reached about a different AGS-era
format from the other direction: `data-format/room-manifest` found that a file survives
round-tripping through a tool *"by never removing anything."* Two formats, two phases, one rule.

**Also settled in passing, and it is the third system to say it.** *"Local variables (inside
functions) may be added and removed freely because they are not saved, because AGS does not allow
the game to save while it is inside a script function."* Thimbleweed Park could not serialize a
running Squirrel thread and paid in three workarounds; AGS forbids the save instead; this design
refused duration and has no inside-a-function to be in.

**What was looked for here and not found.** Any statement of what a save *ought* to contain, or
any acknowledgement that a smaller save keyed by name would not have this problem. The manual
describes the format it has and the ways around it, and does not compare it to an alternative.

## What this document holds that nobody here has looked at

The rest of the manual, which is most of it. Specifically unread and relevant to this corpus:
room and hotspot editing, the character and object models, inventory handling, and the GUI
system that would say how a cursor mode is *presented* to a player as distinct from how it is
declared.

**The scripting language reference was the item at the top of this list and it has been read**
— see *The other side of the escape*, below. It sat here for eight phases described as "where
the boundary between declarative authoring and general-purpose code actually sits, which is
this corpus's central question."

The names of modes 1, 2, 3, 5, 8 and 9 are on some other page. This corpus does not know them
and should not guess them: `EditorCursor.md` names four of ten and the reader who needs the
rest has somewhere to go.


## The other side of the escape

Read 2026-09-11: `ScriptingLanguage.md`, `ScriptKeywords.md`, `Scripting.md`,
`ScriptAPIOverview.md`. This is the language AGS's declarative layer escaped *to* — the target of
action number 1, `Run script` — and it had never been read here, which meant the corpus knew the
size of what the hatch was in and nothing about what was on the other side of it.

### The language

**Twenty-five keyword sections**, and the boundary falls in a place the corpus can state exactly.

| | AGS Script | this design |
|---|---|---|
| comparison | `==` `!=` `>` `<` `>=` `<=` | the same six |
| logical | `!` `&&` `&#124;&#124;` | `not` `and` `or` |
| arithmetic | `*` `/` `%` `+` `-` `++` `--` | none |
| bitwise | `<<` `>>` `&` `&#124;` `^` | none |
| control | `if`/`else`, `switch`/`case`, `while`, `do..while`, `for`, `break`, `continue` | `when`/`else`/`otherwise` |
| abstraction | `function`, `return`, `struct`, `managed`, `new`, `enum`, `this`, arrays | none |
| value types | `char` `short` `int` `String` `float` `bool` | a flag and a bounded counter |

**Twenty-one operators, of which nine are exactly this design's nine.** The bounded predicate
language is the comparison-and-logical fragment of the escape language with the arithmetic and
the bitwise removed — not a different vocabulary chosen for a different purpose, the same one
truncated.

### Termination, which both systems had to answer

**AGS answers it at runtime, by guessing.** *"Normally, if a `while` loop runs for more than
150,000 loops, AGS will assume that the script has hung and abort the game."* And it ships a
keyword to turn the guess off: `noloopcheck`, *"for example, when initializing a very large
array"*, placed between `function` and the function name.

This design answers it structurally — no loops and no recursion, so evaluation terminates by
construction, and `design/specimen/ferry.game` records the one remaining source of infinity
(an unbounded item quantity) closed by a declared ceiling rather than by a heuristic.

### What lives on the far side

The API index lists **fifty-seven pages**. The largest groups are presentation and interface:
`AudioChannel`, `AudioClip`, `Multimedia`, `Screen`, `Palette`, `DrawingSurface`, `DynamicSprite`,
`Overlay`, `Speech`, `ViewFrame`, `Viewport`, `Camera`, and ten `GUI*` pages. Alongside them are
general-purpose facilities no adventure-game vocabulary contains: `File`, `Dictionary`, `Set`,
`Maths`, `DateTime`, `String`, `GlobalArrays`.

That is the same partition the action table shows from the other side. `Globalfunctions_Message`
and the `Character` struct's `Say` are where text lives; everything else in the presentation
group has no counterpart in the forty-eight and none in this design.
