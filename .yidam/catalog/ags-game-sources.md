---
name: AGS game sources on GitHub
description: The script source of twelve Adventure Game Studio games their authors published, most of them released to players. Read for what authors wrote when they made an object pickable, rather than what they asked about it.
type: dataset
obtained: true
retrieved: 2026-11-27
ttl_days: 365
location:
  - kind: url
    value: https://github.com/hewills/10WaysFromSunday
    description: "at c8d5d51a5f73 (30 March 2018); described as the August 2017 MAGS winner; no licence"
  - kind: url
    value: https://github.com/hewills/Wilfred-2088
    description: "at d69ea2693ba9 (30 March 2018); described as the February 2018 MAGS winner; no licence"
  - kind: url
    value: https://github.com/hewills/the4thwall
    description: "at dc22a3506a85 (4 March 2017); described as a MAGS winner; MIT"
  - kind: url
    value: https://github.com/hewills/pinksky
    description: "at 7cbb378ddfb6 (4 March 2017); described as a completed game for the January 2014 MAGS; MIT"
  - kind: url
    value: https://github.com/ericoporto/i_rented_a_boat
    description: "at 72740f53a9bc (20 March 2022); a MAGS game, released on itch.io; no licence"
  - kind: url
    value: https://github.com/adventuregamestudio/ags-demo-game
    description: "at 064a14dfe171 (11 July 2026); the community demo game the AGS project distributes; its own licence"
  - kind: url
    value: https://github.com/adventuregamestudio/ags-demo-quest-old
    description: "at 0793cd4ea0fd (14 April 2024); the older Demo Quest; no licence"
  - kind: url
    value: https://github.com/gondur/cartlife_src
    description: "at b82878b531a7 (6 December 2020); described only as Cart Life's source code; its own licence"
  - kind: url
    value: https://github.com/Wayfinder7/Neofeud
    description: "at 922e9509fce0 (13 September 2014); described only as an AGS game, three script files; no licence"
  - kind: url
    value: https://github.com/SpookyFM/Cargo
    description: "at 05368c07f309 (4 May 2019); described as a point-and-click adventure; no licence"
  - kind: url
    value: https://github.com/NeoDement/game_SteamedHams
    description: "at ebe2267369df (5 December 2022); a fan game; no licence"
  - kind: url
    value: https://github.com/ManiacMansionMania/Maniacweed-Bernard
    description: "at 4da9df7215c6 (4 June 2025); a Maniac Mansion Mania starter pack, which is a template and is not counted"
---

**This is the first AGS entry made of games rather than of what people said about making them.**
[ags-forums](ags-forums.md) holds beginners' threads, and they show authors forgetting to hide the
room object after `AddInventory`, every time caught before release. Whether the gap ever reached
players could not be read from a beginners' board. It can be read from released games' scripts,
because the gap is in the script: a handler that calls `AddInventory` and leaves the object
visible and clickable.

Most of these repositories carry no licence. Only figures, file-and-line citations and handler
names are recorded here; the clones are kept outside the repository. The two MIT repositories are
treated the same way, for uniformity.

## How it was read

Each repository was fetched with only its `.asc` and `.ash` files. A scanner kept outside the
repository found every script function containing `AddInventory` or `AddInventoryToCharacter`,
comments removed and the function found by brace matching. It classified each by the prefix AGS's
editor gives event handlers (`o` an object, `h` a hotspot, `i` an inventory item, `c` a character),
and recorded whether the same function takes a room object off screen (`Visible = false`,
`ObjectOff`, or moving it) and whether it tests anything first. Every object handler that does not
hide was then read by hand, and so was the guard of every hotspot handler.

What was not counted. An object that hides in a function the handler calls was found only by
reading the four non-hiding handlers. An object hidden by a separate room event was not looked for,
because none of the non-hiding handlers needed it.

## What it shows

**Inclusion.** Seven repositories say the game was released: five MAGS entries and the two demo
games the AGS project distributes. Cart Life is a commercial release. That this repository is its
released source is the description's claim, not checked. Neofeud, Cargo and Steamed Hams do not say
whether they were released, and are counted separately. The starter pack is a template and is not
counted.

**Object pickups.** The released eight hold 16 object handlers that add an item. 12 hide the object
in the same function, each by setting that object's own `Visible` to false. [verified] One
hides it through a room function the handler calls: Demo Quest `Room17.asc` 893, whose helper
sets the card's `Visible` to false at line 177. [verified] Three leave the object in the room, and each means to:

- Demo Quest `Room30.asc` 280: buying a cup from a stack the room keeps, with the object's
  graphic changed and a flag set so it cannot be bought twice. [verified]
- *Pink Sky* `room9.asc` 59 and 84: taking the laces from a pair of boots, which stay. Each is
  guarded by its own flag. [verified]

So none of the 16 leaves a pickable copy of what was taken. [verified] Neofeud and Cargo add 8
more, and all 8 hide the object in the same function. [verified] Of the 24, none ships the window. [verified]
With none in 24, a rate above about one pickup in eight would be unlikely to show none (the rule
of three, 3/24). [inference]

**The hide is the guard.** Of the 20 object handlers that hide in the same function, 11 test
nothing first. [verified] Nothing else needs to: once the object is hidden it cannot be clicked
again, so "not already carried" is enforced by the removal. [inference]

**Hotspot pickups.** A hotspot has no picture to hide, so a hotspot that gives an item can give it
again unless something stops it. 12 hotspot handlers add an item, 11 in the released games and one
in Cargo. One disables its hotspot after the add (*I Rented a Boat* `room2.asc` 250). Every one of
the 12 tests a flag or `HasInventory` before adding. [verified] So in the source read, the
precondition the vocabulary states — *not already carried* — is written by hand at every site
where no hide enforces it. [inference]

**No game writes the bundle.** No game has a helper that both hides an object and adds an item.
The one helper found only hides. [verified] Cart Life adds an item in 33 functions, and none is a
room object's handler: what is carried is bought through its interface, not picked up.
[verified]

## What was looked for and not found

A shipped pickup that leaves the object pickable. None in 24. Twelve small games are not the AGS
catalogue, and released source is not players' bug reports. The *Completed Game Announcements*
board's threads were not read for player reports, because the forum rate-limits automated reading.
[inference]

## What these hold that nobody here has looked at

- Everything in the repositories but the `.asc` and `.ash` files: rooms, sprites, the dialog
  scripts, and the editor's project files. A pickup written in a dialog script would not have been
  seen.
- The repositories' histories. Only the tip of each was fetched, so a window fixed before release
  would not show.
- Every AGS game on GitHub outside these twelve, and every one whose source was never published.
