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
  - ../corpus/failure-mode/orphaned-dialogue-branch.yml
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

Two pages, and only two:

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

## What this document holds that nobody here has looked at

The rest of the manual, which is most of it. Specifically unread and relevant to this corpus:
the scripting language reference (where the boundary between declarative authoring and
general-purpose code actually sits, which is this corpus's central question), room and hotspot
editing, the character and object models, inventory handling, and the GUI system that would
say how a cursor mode is *presented* to a player as distinct from how it is declared.

The names of modes 1, 2, 3, 5, 8 and 9 are on some other page. This corpus does not know them
and should not guess them: `EditorCursor.md` names four of ten and the reader who needs the
rest has somewhere to go.
