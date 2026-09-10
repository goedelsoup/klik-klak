---
name: Adventure Game Studio 2.72 manual (archived)
description: The AGS manual as it stood before version 3.0 removed the Interaction Editor — the only description this corpus has of a shipped declarative interaction layer.
type: other
obtained: true
retrieved: 2026-09-10
ttl_days: 3650
location:
  - kind: url
    value: https://ags-archives.github.io/ags272/ags8.htm
    description: the tutorial index; pages are `agsNN.htm` and `acintroN.htm` with no working root index
  - kind: url
    value: https://ags-archives.github.io/ags272/acintro3.htm
    description: Tutorial Part 3 — hotspots and interactions, which describes the Interaction Editor
  - kind: url
    value: https://ags-archives.github.io/ags272/ags67.htm
    description: Interaction events — the 2.72 counterpart of `EventTypes.md`
used-by:
  - ../corpus/prior-art/ags.yml
  - ../corpus/data-format/interaction-table.yml
---

**This is AGS's own documentation and this corpus is not treating it as a source of record.**
The content is the AGS manual — the tutorial pages carry "Copyright (c) 2001-2002 Chris Jones",
the author of AGS — but it reaches this corpus through `ags-archives.github.io`, a GitHub Pages
site whose relationship to the AGS project has not been established here. It is a republication,
and `agent-conduct` is explicit that closer to the source is not the source.

So the split is deliberate and is worth stating before anything cites this entry. **The facts
that the Interaction Editor existed, that AGS 3.0 removed it, that interactions were converted
to script, that its variables are obsolete and that Global Messages existed to serve it are all
in the *current* manual** ([ags-manual](ags-manual.md)) and are `[verified]` from there. This
entry supplies what that one no longer describes: what the thing actually was. Claims resting on
this entry are `[inference]`.

The document is frozen — 2.72 shipped in 2007 and the tutorial page dates itself "created 21
November 2002; updated 11 Feb 2006" — which is why its TTL is ten years.

There is no working root index: `https://ags-archives.github.io/ags272/` returns GitHub's 404
page with HTTP 200-looking content, and the pages are reachable only by name. Numbered pages run
`ags1.htm` to `ags77.htm`; above that the same 404 page is returned, which is how the extent was
established rather than by reading a contents page.

## What was read, and for what

Read for one question: what did AGS's authoring surface for room and hotspot interactions look
like before it was script?

**The structure of the Interaction Editor** (`acintro3.htm`). A hotspot carried an
"Interaction..." button opening a window listing *events* — "from 'Player stands on hotspot' at
the top down to 'Usermode2 hotspot' at the bottom". Double-clicking an event opened an *action
editor* whose "large drop-down list box near the top of the window… currently says 'Do nothing'".
Choosing a command reveals its parameters: "The text 'Message number: 0' is a parameter for the
command. Different commands have various parameters, and they let you set specific information
for this use of the command."

Three properties follow and each matters to this corpus:

- **The command set is closed and the author picks from it.** A combobox, described as "quite an
  extensive list of options" and, tellingly, "in alphabetical order".
- **An event maps to an ordered list, not one command.** "You can add as many commands as you
  like to be run when an event happens", added by right-clicking the event and choosing "New
  action", and displayed as a tree under it.
- **The default is a value, not an absence.** The box "currently says 'Do nothing'" at the same
  moment the page says "nothing is set to happen in response to any of the events".

**There was a conditional.** `ags67.htm`, documenting "Use inventory on inv", instructs the
author to "Add a **'Conditional - if inventory item was used'** check to determine which item
they used." So the command list had a `Conditional` category, and a predicate over world state
was a command in the same list as the effects.

**Nine commands are named across the manual and the set is never enumerated.** In
`Category - Verb` form:

| Command | Where |
|---|---|
| `Conditional - if inventory item was used` | `ags67.htm` |
| `Game - Display a message` | `acintro3.htm` |
| `Game - Run Dialog` | `acintro8.htm` |
| `Game - Enable dialog option` | `acintro8.htm` |
| `Player - Go to a different room` | `acintro3.htm` |
| `Player - Add inventory` | `acintro5.htm` |
| `Object - Remove an object` | `acintro7.htm`, `ags9.htm` |
| `Character - Move Character` | `acintro7.htm` |
| `Character - Quick animation` | `acintro7.htm` |

**The same tutorial task, in both eras, is the sharpest thing here.** 2.72's scripting tutorial
opens by teaching how to display a message when the player looks at a hotspot and immediately
concedes the point: "(Of course, you don't actually need to use a script to do this, a simple
Display Message interaction command would work just as well)." The current manual's `acintro3.md`
sets the identical task — look at a sign, display a line — and answers it with
`function hSignHome_Look() { Display("…"); }`.

Also read: `ags26.htm` and `ags27.htm`, the scripting tutorials, checked specifically for whether
"conditional" anywhere refers to the Interaction Editor. Every occurrence in the pages read is
about the script language's `if`.

## What was looked for and not found — read this before citing

**The full contents of the command combobox are nowhere in this manual.** The tutorial calls the
list "quite an extensive list of options" and never enumerates it; there is a page titled
"Interaction events" and none titled anything like "Interaction commands". The nine above are
what the prose happens to name while explaining something else.

That is the same shape of gap this corpus has twice been caught by — a page that names some
members of a set and does not claim to name all of them. Nine commands are quotable. *"The
Interaction Editor's vocabulary was N commands"* is not, and neither is any claim about what it
could not express. In particular: one conditional is documented, and whether the `Conditional`
category held one member or twenty is unknown here.

Not read: the remaining seventy-odd pages, which are mostly the 2.72 script API and duplicate
what the current manual documents better.
