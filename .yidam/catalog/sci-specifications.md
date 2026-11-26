---
name: SCI Specifications (Skovlund, Reichenbach et al.)
description: The SCI community's reverse engineering of Sierra's Creative Interpreter — resource formats, the virtual machine, and the kernel function table, which is the whole of what the engine offered a game script.
type: other
obtained: true
retrieved: 2026-09-12
ttl_days: 3650
location:
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Specifications
    description: the six-chapter specification and its index, and the copy this corpus read
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Specifications:_Chapter_5_(cont.)_-_Kernel_functions
    description: the SCI0 kernel function table — 114 calls, read in full for what the engine offers a script
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Specifications:_Chapter_3_-_The_Graphics_subsystem
    description: windows, dialogs and controls — the six control types, read for the icon
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Specifications:_Chapter_6_-_SCI_in_action
    description: event handling, the parser and Said specs — read for the event vocabulary
---

**What [`agi-specifications`](agi-specifications.md) is to AGI, this is to SCI**, and the pairing
was named in [`sci-inhouse-docs`](sci-inhouse-docs.md) before either had been read: the community's
reverse engineering of the interpreter, as against Sierra's own documentation of how to write for
it. Credited to Lars Skovlund, Christoph Reichenbach, Ravi Iyengar, Rickard Lind, Vladimir
Gneushev, Petr Vyhnak, Dark Minister and Francois Boyer, copyright 1999–2002, last edited 2 December
2015. Six chapters: Introduction, Resource Files, Graphics, Sound, the Virtual Machine, and SCI in
Action.

It distinguishes the interpreter versions this corpus cares about: *"SCI0 will refer to all games
using the SCI version 0.xxx, except for those games who use the 'in-between' game engine referred to
as SCI01… SCI1 will refer to the interpreter version 1.xxx."* [verified]

**Why it was retrieved.** `prior-art/sierra-sci` asked whether SCI1's icon bar was a game script or
an engine feature, named the wrong document as what would settle it, and was corrected to ask for an
SCI1-era artifact ([retraction-reach](../decisions/retraction-reach.yml)). A kernel function table
is better than an artifact for this question: it is the complete list of what the interpreter
offered a script, so a thing absent from it is a thing the engine did not do.

## What was read — 2026-09-12

**Chapter 5 (cont.), the SCI0 kernel function table, in full.** 114 calls, `0x00` to `0x71`.
[verified] The whole surface between a game's scripts and the interpreter, which is why a census of
it answers a question about where an interface lives.

**Five of the 114 are the menu bar.** `AddMenu` (`0x22`), described as *"adds a menu to the menu
bar"*, `DrawMenuBar` (`0x20`, *"1 to draw, 0 to clear"*), `MenuSelect` (`0x21`, taking an event),
`GetMenu` (`0x5f`) and `SetMenu` (`0x60`). [verified]

**And `AddMenu` takes a declaration rather than a drawing.** Its `content` parameter is a string in
a five-character markup:

| character | what it does |
|---|---|
| `` ` `` | right-justify what follows |
| `:` | separate one menu item from the next |
| `--!` | draw a separation line |
| `#` | a function key, shown as F |
| `^` | a control key, shown as CTRL |

[verified] A menu, its items, their layout and their accelerators, declared as a string and handed
to the engine. **That is a second closed pattern language beside the Said spec**, and this corpus
had counted Sierra's declarative budget as one.

**Three of the 114 are controls, and a control has six types.** `DrawControl` (`0x17`),
`HiliteControl` (`0x18`) and `EditControl` (`0x19`), against type numbers 1 Button, 2 Text, 3 Edit,
**4 Icon**, 5 unused, 6 Selector — the last glossed *"as in the Save and Restore boxes"*. A control
carries a state whose bits are *"selectable"*, unknown, *"disabled"* and *"selected"*. [verified]
The section opens by saying controls and menus are unrelated and use different kernel calls, and
that *"All control types are descendants of a common class (I do not know its name, since it appears
to have an invalid name property)."* [verified]

**So the interpreter draws an icon and knows nothing about a bar.** There is no `AddIcon` beside
`AddMenu`, no icon-bar call, no verb call, and nothing named Icon anywhere in the 114 — the icon is
a *type number* passed to a general control-drawing call. [verified]

**Chapter 6, event handling.** By Lars Skovlund, version 1.0, 12 July 1999. The interpreter
delivers five event types and no more: `0x00` null, `0x01` mouse button, `0x02` mouse button
release, `0x04` keyboard, `0x40` movement (joystick). [verified] Polling differs by device — *"The
keyboard is typically polled at each timer tick (which is 60 hz)"*, the mouse driver *"polls itself"*
through a callback, and *"The joystick is only polled when the script wants to."* [verified]

**Nothing in that vocabulary reports a selection.** A script receives a raw mouse-down and hands it
to `MenuSelect` for the engine to interpret; there is no *a menu item was chosen* event and no *an
icon was clicked* event. That is why `Script Classes for Adventure Games`' `User` class sends
`handleEvent:` to `TheMenuBar` first ([sci-inhouse-docs](sci-inhouse-docs.md)): the menu bar has to
be offered the raw event before anything else can have it.

## What was looked for here and not found — read this before citing

**The SCI1 kernel table is not in this document.** The list above is headed *SCI0 Kernel functions*
and no equivalent enumeration for 1.xxx appears. So every claim here about what SCI1 did **not**
add is `[inference]` from SCI0's table and the shape of the change, not a reading of SCI1's.

**No account of how a game arranged icons.** The document specifies the interpreter and says
nothing about any game's system scripts, so what the icon bar of King's Quest V is made of is
outside it in both directions.

**No statement of intent anywhere.** This is a specification written by people building
interpreters, and it describes what the bytes do. It is an artifact about an artifact, and
[testimony](../decisions/testimony.yml)'s gap is untouched by it.

## What this document holds that nobody here has looked at

- **Chapters 2 and 4** — resource file formats and the sound subsystem, neither of which bears on
  an authoring vocabulary.
- **Chapter 5's instruction set and heap layout.** The PMachine's opcodes are the tier below the
  language `Script Classes for Adventure Games` describes, and this corpus has read the language
  end from Sierra's own documentation.
- **Chapter 6's parser sections** — *The Parse tree*, *Said specs*, *Matching the trees*. Sierra's
  own SCI Parser Programmer's Reference covers the same ground and this corpus holds it in full
  ([sci-inhouse-docs](sci-inhouse-docs.md)), so these would be a second account from the weaker
  source.
- **The SCI1 kernel table, wherever it is.** It is the one retrieval this entry leaves named and
  undone, and it would turn the inference above into a reading. ScummVM's SCI engine holds a
  kernel table per version in source, which is a different kind of artifact and a plausible place
  to find it.
