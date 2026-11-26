---
name: Adventure Game Studio forums
description: The AGS community forums, where Chris Jones announced and argued the 3.0 changes in the beta threads and where beginners' questions record what authoring in AGS was like to do.
type: other
obtained: true
retrieved: 2026-11-26
ttl_days: 1825
location:
  - kind: url
    value: https://www.adventuregamestudio.co.uk/forums/advanced-technical-forum/ags-3-0-final-it-s-been-a-long-road/msg414609/
    description: "Chris Jones (Pumaman), reply #189, Wed 15/08/2007 22:01, in the thread then titled \"AGS 2.8 beta 7 - Climate Change Edition\" and since renamed. The old address, yabb/index.php?topic=31519.msg414609, redirects here through index.php?msg=414609"
  - kind: url
    value: https://www.adventuregamestudio.co.uk/forums/advanced-technical-forum/suggestion-for-a-more-user-friendly-adventure-game-studio/20/
    description: "\"Suggestion for a more User-Friendly Adventure Game Studio\", page 2, replies #20-#39, 1-6 October 2007"
  - kind: url
    value: https://www.adventuregamestudio.co.uk/forums/beginners-technical-questions/object-appearing-after-dialog-has-ended-solved/
    description: "\"OBject appearing after dialog has ended [SOLVED]\", Beginners' Technical Questions, 14 July 2013, page 1"
---

**This is the testimony row the catalog was missing for AGS.** The three AGS entries
([ags-manual](ags-manual.md), [ags-272-manual](ags-272-manual.md), [ags-source](ags-source.md))
are artifacts. They record what was built. This entry records what the maker and the authors said
about it. `.yidam/decisions/testimony.yml` named that gap as the reason three questions stayed
open, and `.yidam/decisions/ags-testimony.yml` is the record of reading this entry against them.

It is a **forum**, and two kinds of standing need keeping apart:

- **Chris Jones's own posts** are a maker's testimony, the same kind of source as
  [twp-dev-diary](twp-dev-diary.md) is for the other house. He posts as *Pumaman*, marked
  "Creator of AGS" and "Administrator".
- **Everyone else's posts** are other authors' testimony. They say what using AGS was like for
  them and what the community taught. One beginner thread is not a base rate.

**A name to keep straight.** The moderator of the Advanced Technical and Beginners' Technical boards
posts as *Gilbert*, marked "Local Moderator". That is not Ron Gilbert. Everywhere else in this
corpus "Gilbert" means Ron Gilbert and Terrible Toybox, so this entry and every node citing it says
"the forum moderator Gilbert" whenever it means him.

## What was read

Read 2026-11-26, three pages: reply #189 in the beta thread, "Suggestion for a more User-Friendly
Adventure Game Studio" page 2, and "OBject appearing after dialog has ended" page 1.

Each page was read whole in a browser. The quotations are copied from the page as rendered.

**Chris Jones, 15 August 2007, reply #189.** He was answering a user who could not find the
Interaction Editor in the 2.8 beta, the beta that became 3.0:

> Yes, the inteaction editor has been scrapped. I had a long, hard think about this but basically I
> felt that the Interaction Editor in 2.72 was too simple to really be able to make a game with, and
> that it gave a false sense of "easiness" of game creation to people who would very quickly come
> across something that it didn't support, and get frustrated with it.
>
> I think the way forward is to have some sort of auto script-generating tool (maybe as a plugin)
> that works in a similar way to the interaction editor but it basically creates the script code
> for you.

The spelling "inteaction" is his.

**"Suggestion for a more User-Friendly Adventure Game Studio", page 2.**

- The forum moderator Gilbert, reply #29, Tue 02/10/2007 03:18:57, gives the same reason
  secondhand. The editor is "never complete to a point that it's really useful (you can't even make
  a simple game without any scripting)", and it "requires a lot of work to maintain and update". He
  then links to Jones's post above ("some comments on it by CJ here").
- RickJ, reply #30, quotes Jones's second paragraph.
- On the users' side: bolfar (#24) says the editor "got me started with AGS", and that "any new user
  with no scripting experience would" miss it. subspark (#25) and Sylvr (#27) say it helped them
  learn. Khris (#21) says "there's no need for the IE at all".
- SupSuper (#37) describes what 3.0 kept: "Think of it as an I.E. with only a Run Script option."
- Jones's own reply on this page (#23) is advice on learning to script. It says nothing about the
  removal.

**"OBject appearing after dialog has ended [SOLVED]", page 1 (2013).** A beginner asks how to make an
object appear after a conversation and then be picked up. Slasher (#2) answers with a recipe that
writes both halves by hand: make the object visible, walk to it, "make object visible = false and
then add inventory object to player". Slasher's second example (#10) does the same thing:
`object[7].Visible=false;` and then `cEgo.AddInventory(isatellite);`. The rest of the thread is
about an object index error, which is unrelated.

## What was looked for and not found — read this before citing

**Whether AGS authors left a room object on screen after `AddInventory`, and paid for it, is not
shown.** The one thread read here teaches the two writes as a pair. That shows authors had to know
the pair. It does not show anyone forgetting it. A web search (2026-11-26) returned more beginner
threads on the same topic, including "New inventory item flashing" and "How to remove an item from
the inventory when used?". Its summary attributes to Crimson Wizard the remark that giving an item
never removes the room object. **Those threads were not read**, and the summary is an aggregator's,
so nothing here rests on it.

**Whether AGS authors experienced an orphaned dialogue option as a loss is not shown.** Four dialog
threads were read by a fetching agent during this retrieval, and none of them was re-read here:

- guga2112, 2020
- TheRoger, 2011
- Dannymac247, 2015
- Fluxpuppy, 2014

As that agent reported them, all four are how-to questions. The nearest case is the Dannymac247
thread: an option meant to be switched off permanently was defeated by fall-through after a missing
`return`, so the player got extra options rather than lost content. That is agent-reported, so it
carries no tag weight. "Dialog options problem", "Dialog puzzle", topic 56985 and topic 24248 were
named as leads and not read.

**The forum rate-limits.** A burst of automated reads earlier in this retrieval returned
`503 Service Unavailable` with `Retry-After: 3600`. The pages above were read one at a time after
that hour had passed.

## What the forums hold that nobody here has looked at

Nearly all of it. The beta thread cited above runs to more pages than the one page opened here, and
Jones answered questions about 3.0 across it. The other 2007-2008 beta threads are unopened. So is
everything Jones wrote after that, and so are the Beginners' Technical Questions archive and the
leads named in the section above. The forum's own search was not used, because the rate limit made
automated reads the wrong tool for it.
