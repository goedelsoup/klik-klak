---
name: Sierra's in-house SCI documentation
description: Eleven documents Sierra wrote for its own game programmers between 1988 and the early nineties, preserved by the SCI community — the first source here written by a working author's employer to tell them how to write.
type: other
obtained: true
retrieved: 2026-09-12
ttl_days: 3650
location:
  - kind: url
    value: http://sciwiki.sierrahelp.com/index.php?title=SCI_Documentation
    description: the index of what Sierra's own documentation set contains, and the copy this corpus read
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Parser_Programmer's_Reference
    description: the one document read in full — Pablo Ghenis, 21 July 1988
  - kind: url_template
    value: https://sciwiki.sierrahelp.com/index.php/SCI_Parser_Programmer's_Reference/{chapter}
    description: its chapters — Introduction, User Parse Trees, Said Specs, Tree Matching, Examples
  - kind: url
    value: https://sciwiki.sierrahelp.com/index.php/Sierra_Creative_Interpreter
    description: the community's account of SCI's versions, read for the SCI0-to-SCI1 interface change
used-by:
  - ../corpus/prior-art/sierra-sci.yml
---

**The first entry here that is documentation an author was handed by the people who employed them.**
Every other source in this catalog is a manual published to the world (`ags-manual`,
`ags-272-manual`), a shipped artifact (`ags-source`, `delores-source`,
`scumm-v5-opcode-reference`), a designer writing in public (`grumpy-gamer-pdc`, `twp-dev-diary`),
or an encyclopedia (`wikipedia-scumm`). This is internal: written at Sierra, for Sierra's own game
programmers, and preserved by the SCI community after the fact.

That is **not** the testimony [`.yidam/decisions/testimony.yml`](../decisions/testimony.yml) says
this corpus lacks, and the distinction is worth holding. A style manual tells an author what to do;
it does not report what doing it was like. These are artifacts about practice rather than accounts
of experience, and the gap that record names stays open.

The set as indexed is eleven documents: *The Script Programming Language*, *SCI Kernel
Documentation*, *Object Oriented Programming in Script*, *Script Classes for Adventure Games*,
*SCI Parser Programmer's Reference*, *Run Time System Changes*, *SCI Changes & Updates*, *The SCI
Programming Language*, *Recommended Procedures for SCI Game Programming Coordinators*, *Gravity
Simulation in SCI*, and *Laura Bow II Programming Style Manual*. **One has been read.**

The transcriptions are hosted on a live wiki, so the TTL is set against the subject rather than the
page: SCI stopped changing in 1997 and these documents are dated 1988 onward. The risk this entry
carries is transcription drift on a wiki nobody here watches, which is a different failure from the
one a TTL catches and is recorded rather than defended against.

## What was read

**The SCI Parser Programmer's Reference**, attributed on the page to Pablo Ghenis and dated
21 July 1988 — within months of SCI0 shipping with King's Quest IV. Six chapters: Introduction,
User Parse Trees, Said Specs, Tree Matching, Examples, Index.

It opens by saying exactly what it is for: **"The parser is the part of SCI that accepts sentences
from the user and allows game coders to specify how to respond to it."** [verified] Three
components, and the split is the design: *"The sentence parser takes the words typed by the user
and generates a 'tree' that describes the sentence's structure using traditional grammar rules. The
spec parser accepts 'specs' and also generates tree structures."* [verified] A matcher compares the
two trees.

**The Said spec is a small closed pattern language, and it is the only declarative construct
Sierra gave an author.** The reference's own worked example is `(Said 'take/gold/dwarf<from')`, and
it takes it apart term by term [verified]:

- `take` — *"the first of three sections separated by slashes, it is the verb in the verb phrase."*
- `/gold` — *"the slash signals the beginning of the direct object, which is a noun phrase with
  root 'gold'"*
- `/dwarf` — *"the slash signals the beginning of the indirect object, which is an Associated
  Phrase. The root is 'dwarf'"*
- `<from` — *"the '<' means 'modified by'. In this case the preposition 'from' modifies the root of
  the AP"*

Two operators, three sections, roots drawn from a dictionary the game declares. Other specs given
are `'get/rock'` and `'hit/tree/ax<with'`. **That is Layer 0 of
[`design/bounded-expression-language.md`](../../design/bounded-expression-language.md) written as a
grammar** — `site := verb target [ object ]` is the same three slots, and a Said spec adds the
preposition and a notion of root-matching that this design has no need for because its targets are
declared names rather than typed words.

**And it is where all of Sierra's declarative budget went.** What follows a matching Said spec is
ordinary code in a general-purpose object-oriented language — SCI is *"an object-oriented language
with message passing semantics (like Objective-C or SmallTalk)"*, and a room's responses live in
its script's `handleEvent` method, with game-wide ones in the main script's. So the bound is on
**what the player may say**, and there is none at all on what happens next. This design spends it
the other way round: the site is fixed by a verb menu, and the bounded language is the guard and
the effect.

**The interface change this corpus most wanted to read about was not an engine change.** SCI0
shipped in 1988 with King's Quest IV and a parser; SCI1 in 1990 with King's Quest V and a mouse.
And the community account of SCI's versions puts the load-bearing fact plainly: *"SCI1 also
introduced a Point&Click interface (a mouse-driven, icon-based), **which was implemented by the
game's scripts, not the SCI1 interpreter**."* [inference] — this sentence is the wiki's, not
Sierra's, and no document read here contradicts or confirms it; the same page's account of generic
classes *"for rooms and actors and icon bars"* living in **system scripts** is consistent with it.

If that holds, the icon bar was a library a game shipped rather than a primitive an engine offered,
and Sierra's authoring surface was a general-purpose language at both ends of the transition —
which is what [`prior-art/sierra-sci.yml`](../corpus/prior-art/sierra-sci.yml) rests on it for.

## What this document holds that nobody here has looked at

Ten of the eleven, and two of them are aimed straight at open questions here.

- ***Script Classes for Adventure Games*** is the authoring surface this entry has read around
  rather than read: what an author inherits from, and what a room, an actor and a feature are as
  classes. The corpus's `subsystem` class and its `verb` instances both have a stake in it.
- ***Recommended Procedures for SCI Game Programming Coordinators*** and the ***Laura Bow II
  Programming Style Manual*** are procedural rather than referential — what Sierra told a team to
  *do*. Nothing else in this catalog is that, and they are the nearest thing in reach to the
  question `testimony.yml` names, without being it.
- *The Script Programming Language*, *The SCI Programming Language*, *Object Oriented Programming
  in Script* and the *SCI Kernel Documentation* describe the general-purpose half — the thing an
  AGS author escapes *to*, documented from the inside for once.
- *Run Time System Changes*, *SCI Changes & Updates* and *Gravity Simulation in SCI* have not been
  assessed for relevance.

Also unread on this wiki, and outside the in-house set: the SCI Specifications, which are the
community's reverse engineering of the interpreter and stand to SCI as `agi-specifications.md`
stands to AGI.
