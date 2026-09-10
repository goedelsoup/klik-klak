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
used-by:
  - ../corpus/prior-art/ags.yml
  - ../corpus/verb/pick-up.yml
  - ../corpus/interaction/acquisition-binding.yml
  - ../corpus/verb/talk-to.yml
  - ../corpus/conversation/branching-tree.yml
  - ../corpus/conversation/topic-list.yml
  - ../corpus/data-format/interaction-table.yml
  - ../corpus/verb/use-with.yml
  - ../corpus/verb/give-to.yml
  - ../corpus/verb/look-at.yml
  - ../corpus/interaction/default-binding.yml
  - ../corpus/prior-art/ags.yml
  - ../corpus/failure-mode/orphaned-dialogue-branch.yml
  - ../corpus/world-flag/latched-flag.yml
  - ../corpus/inventory-item/stackable-item.yml
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

Eight pages. The count is kept current because this entry is read to decide what is
citable, and a stale one understates what is available — it said "two pages, and only
two" while listing eight, for six of which it was the only record.

- **`EditorCursor.md`** — cursor modes. Names Mode 0 (Walk), Mode 4 (Use Inventory), Mode 6
  (Pointer) and Mode 7 (Wait); refers to modes 1, 2, 3, 5, 8 and 9 without naming them on this
  page. States that an author "may create more if necessary", and that the `StandardMode`
  property marks a mode that fires an event on whatever is clicked — applying to Walk, Look,
  Interact and Talk, and explicitly *not* to Use Inventory.
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

## What this document holds that nobody here has looked at

The rest of the manual, which is most of it. Specifically unread and relevant to this corpus:
the scripting language reference (where the boundary between declarative authoring and
general-purpose code actually sits, which is this corpus's central question), room and hotspot
editing, the character and object models, inventory handling, and the GUI system that would
say how a cursor mode is *presented* to a player as distinct from how it is declared.

The names of modes 1, 2, 3, 5, 8 and 9 are on some other page. This corpus does not know them
and should not guess them: `EditorCursor.md` names four of ten and the reader who needs the
rest has somewhere to go.
