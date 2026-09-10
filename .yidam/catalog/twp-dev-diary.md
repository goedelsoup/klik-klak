---
name: Thimbleweed Park Development Diary
description: A near-daily record, 2015-2019, of a classic point-and-click adventure being designed and its engine being built, written by the people who invented the form.
type: other
obtained: true
retrieved: 2026-09-10
ttl_days: 1825
location:
  - kind: url
    value: https://blog.thimbleweedpark.com/archives.html
    description: "The complete archive index — 258 posts, Jan 2015 to Mar 2019, reverse chronological"
  - kind: url
    value: https://blog.thimbleweedpark.com/
    description: "The blog itself; every post cited below is `<slug>.html` under this host"
used-by: []
---

**A source of record, and the closest one this corpus will ever get to its own question.**
Thimbleweed Park was written by Ron Gilbert, Gary Winnick, David Fox and Jenn Sandercock —
the designers of *Maniac Mansion* and *Zak McKracken* — on an engine Gilbert wrote from
scratch between 2015 and 2017, and the diary is their own account of building it, posted as
they built it, with their own source code in it.

That matters because every other prior-art source here is at one remove. AGS is documented by
its maintainers but was not built by anyone who defined the form; SCUMM was, and its authoring
documentation was never released, so the corpus reads a reverse-engineered opcode list and an
encyclopedia article. This is the first source that is **both** primary and by the people who
invented the thing. For *what they built and why*, claims may be `[verified]`.

Two limits. It is a **development** diary: what is described is often in flight, and syntax
changes between posts (see the prefix change below). And the posts are written for backers, not
for engineers — code appears as illustration, never as specification, so a claim about what the
engine could *not* do is `[inference]` unless the text says so.

## What was read — 2026-09-10, twenty-two posts of 258

The retrieval was aimed at one question: **what authoring surface did they build?**

**The decision, and it took fifteen days.** `engine.html` (Ron Gilbert, 5 Jan 2015) is the
second week of the project, and it ends: "I think my engine has everything I need... Except a
scripting language." Lua is rejected on syntax. Writing one is costed and declined — "I figured
it would take me about a month, but I'm not sure I want to eat up that much time." Squirrel is
chosen, with the intent "to modify the syntax a little to mimic how SCUMM laid out code for
rooms and objects."

`scripting_test.html` (20 Jan 2015) says what the month was for. Squirrel was not enough out of
the box and had to be extended before anything else happened: "one of the things I always liked
about SCUMM was how it handled multi-tasking. It was effortless to run scripts in different
threads and let objects handle themselves without cumbersome state engines or weird
co-routines. I just want to start a function and have it run until told otherwise... Squirrel
didn't have the ability 'out of the box'." Vincent Hamm wrote the proof of concept. And the
criterion, stated flatly: "The true test of a game scripting language is being able to do
powerful things with very little code."

**The shape of a room.** `ui_in_action.html` (17 Feb 2015) gives the first room script and the
reason for its shape: "I was having some issues with how Squirrel handled classes, so I ditched
the notion of rooms being classes and just made them simple tables/dictionaries... It also
allowed me to get rid of the `object { }` table that enclosed all the room's objects." So a
room is a table, an object is a table inside it, and a binding is a **field on the object named
after the verb**:

```
bankClock =
{
    name = "Clock"
    lookAt = function()
    {
        currentActor.say("It's who cares'o clock.")
    }
}
```

`pseudorooms.html` (5 Jan 2016) shows the same shape eleven months later with the verb slots
**prefixed** — `verbLookAt`, `verbUse`, `verbPickUp`, `verbWalkTo` — alongside ordinary author
data on the same table (`name`, `initState`, `flags`, `hotel_room`, `look_count`,
`flyers_left`). It also gives the room's own hooks, `enter`, `exit` and `initRoom`, and the
whole 110-room hotel built by a `for` loop over `definePseudoRoom`.

`switching_and_inventory.html` (9 Mar 2015) has the fullest single object, a stack of flyers
with a count, a state change at zero, and one verb slot calling another. `codemonkey2.html`
(Jenn Sandercock, 11 Jan 2016) has the same shape reached from the other end: a shared
`sewerValveSetup(this)` helper in `SewerHelpers.nut`, written because the alternative was
copy-paste across dozens of rooms.

**The room file, which is a different thing.** `tool_names.html` (14 Mar 2015) and `wimpy.html`
(16 Mar 2015) describe **Wimpy**, the successor to SCUMM's FLEM: a standalone editor that reads
and writes a room's `.JSON` file. The file is quoted in full form and **contains no behaviour
at all** — `background`, `name`, `sheet`, and per object `hotspot`, `pos`, `zsort`,
`animations`, `prop`, and the two that matter most here, `usepos` and `usedir`. Walk boxes are
in the same tool: FLEM "allowed the game programmers to define objects, object states and the
walk boxes", and `roadmap1.html` (7 Apr 2015) lists "Walk boxes - The editor part of the task is
done".

**What a save can hold.** `savegame.html` (19 Oct 2015) is the single most useful post here and
it is about state, not saving. Five things are saved: global variables, per-room locals,
per-actor locals, and internal actor and object state. Everything else is rebuilt: "When a game
loads, I just re-enter the room." Three consequences are stated outright — game globals had to
be moved into one table (`g.talked_to_sheriff`) because there was no other way to tell them from
the interpreter's; **a running thread cannot be saved** ("I looked at the Squirrel source code
and I don't see an easy way to serialize and recreate a running thread"), so global threads are
simply not saved and local ones are killed on exit and restarted by `enter`; and **you cannot
save inside a cut-scene**, so the engine silently saves just before one and a save during it
restores to that point. A real savegame file is quoted, with real variable names.

**The verbs.** No post enumerates the set. `freewriting.html` (1 Dec 2015) names six while
arguing about keyboard mapping — "Open being 'O', Close being 'C', use being 'U'" and "the
first letter matching the verb has some issues, mainly Push, Pull and Pickup" — and settles that
they are laid out in a **grid**. `ui_changes.html` (24 Aug 2016) removes the sentence line and
keeps the verbs. `switching_and_inventory.html` records `USE obj WITH obj` as the last inventory
task; actor tables carry `flags = GIVEABLE|TALKABLE`.

**Dead ends, on the record.** `modernizing.html` (30 Mar 2015): "Thimbleweed Park will not have
the death and dead ends of Maniac Mansion. **Those were born from ignorance more than a focused
design decision.** It will follow my rules of adventure game design." The same post asks for two
things a conversation should be able to gate on that are not flags — elapsed time since the last
conversation, and whether the player has visited a location.

**The charts.** `act_2_puzzles.html` (11 Mar 2015) and `final_puzzles.html` (18 May 2015) post
the Thimbleweed Park dependency charts during design. The second calls it "the most complex
dependency chart I've ever done" and describes the unfinished nodes the same way the 2015 Act
1-3 post does: "The nodes with red outlines are nodes we still need to figure out the puzzle.
We know the object/outcome, but not how you get here. **These nodes will expand into 2 or 3
nodes.**"

`cutting.html` (20 Jul 2015) records the scale the design was cut *to*: "We had over 100 rooms,
far more then either of the Monkey Island games", and twenty-five were cut in one week.

## What was looked for and not found — read this before citing

**Still no measured chart width, and now there is a reason rather than an absence.** The charts
are posted as images and they are **deliberately blurred**: "Everything is blurred out, so there
are no spoilers." So the obstacle is not that nobody has counted the boxes — it is that the
published artifact was produced so that they cannot be counted. That closes off the retrieval
[grumpy-gamer-pdc](grumpy-gamer-pdc.md) was hoping for, from this source, permanently.

**No enumeration of the verb set.** Six are named in passing and the corpus should not infer the
rest from the screenshots, which is what the "verb counts" trap in
[wikipedia-scumm](wikipedia-scumm.md) already cost once.

**No statement that anything was impossible.** The posts describe what was built. Nowhere does
anyone say a declarative layer was considered and rejected — Gilbert costs writing a *scripting
language* at a month and never mentions the alternative. That a bounded layer was not considered
is `[inference]` from silence, and weak.

## What these posts hold that nobody here has looked at

236 of 258 posts, including sixty-eight podcast episodes with transcripts nobody has read, the
whole of David Fox's and Gary Winnick's contributions beyond one post, and the entire art,
audio, translation and testing story. Also unread and directly on-topic: `debugger.html`,
`text_tron_3000.html`, `hints_and_dialogs.html`, `text_lock.html` and `translations.html`, which
between them cover how the dialogue and text were authored, and that is the half of the
authoring surface this retrieval did not ask about.

**And the artifact this source points at.** The complete game scripts of *Delores: A Thimbleweed
Park Mini-Adventure* were released by Gilbert in 2020 at `github.com/grumpygamer/DeloresDev`,
along with Wimpy, for the same engine described here. It is the thing the diary is evidence
about, in full, and it would turn every claim in this entry from *illustrated* into *counted* —
how many bindings, how many are one line, how many need a conditional, how many need a thread.
Nobody here has read it. It is the highest-value retrieval this corpus has ever identified.
