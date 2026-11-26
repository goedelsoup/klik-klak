---
name: AGI Specifications (Lance Ewing et al.)
description: Reverse-engineered specification of Sierra's Adventure Game Interpreter — the bytecode, the resource formats, and a LOGIC language the document says is not the one Sierra wrote in.
type: other
obtained: true
retrieved: 2026-09-12
ttl_days: 3650
location:
  - kind: url
    value: https://www.agidev.com/articles/agispec/agispecs.html
    description: the eleven-section specification, and the copy this corpus read
  - kind: url
    value: https://www.agidev.com/articles/agispec/agispecs-4.html
    description: section 4, the LOGIC language and the reference of its commands
  - kind: url
    value: https://www.agidev.com/articles/agispec/agispecs-6.html
    description: section 6, LOGIC resources — the bytecode, and 6.2 the command list
  - kind: url
    value: https://www.agidev.com/articles/agispec/agispecs-10.html
    description: section 10, other game data — the OBJECT file and WORDS.TOK
  - kind: url
    value: https://www.agiwiki.sierrahelp.com/index.php/AGI_Specifications:_Chapter_6_-_Logic_Resources
    description: the same section on the AGI Wiki, read to confirm the per-game command range
---

**Reverse-engineered, and it disclaims the one thing this corpus most wanted from it.** The
specification documents the interpreter Sierra used from 1984 to 1989 — Space Quest, Leisure Suit
Larry, the early King's Quests — and it was written by people building editors and interpreters
rather than by anyone at Sierra. It says so about its audience: *"AGI specs is intended for people
writing AGI programs such as editors, viewers and interpreters. It is not supposed to be a
beginners' introduction to AGI, or a LOGIC programming language for those who just want to create
games."*

And then it says the harder thing, in section 4: **"This is the 'official' LOGIC syntax that has
been decided on. It is not the same syntax as Sierra used."** The bytecode is the artifact; the
language above it is a reconstruction agreed on by the people writing the tools. So this entry is
a source about **the machine** and not about the authoring surface, and it is the second entry here
in that position — `scumm-v5-opcode-reference.md` holds the same standing for the same reason and
states the same rule: proximity to the artifact does not promote a derived document to a primary
one. Claims about what an AGI *author* wrote rest on this as `[inference]` and stay there. Claims
about what the interpreter executed are `[verified]`.

The TTL is ten years rather than the one and two this corpus gives its living sources, on the same
ground as the SCUMM entry: the subject stopped changing in 1989.

## What was read

Read for one question — **where did Sierra put the finiteness?** — and it answers it in four places.

- **There is no declarative layer.** *"All rooms have a single LOGIC script that governs what
  happens within that room."* Every interaction in an AGI game is a branch in a procedure that the
  interpreter re-runs each cycle: `logic.0` runs *"over and over again with each interpretation
  cycle"* and every other logic executes only when called from it. [verified] There is no table,
  no binding list, and nothing an author fills in — which makes AGI the second engine here with
  no declarative layer to escape *from*, after SCUMM.

- **The command set grew per game, and the document says so in as many words.** *"The first set of
  codes is the AGI commands themselves and they have the range: 0x00 - 0xB5 AGI commands. (eg.
  animate.obj). The value 181 (0xB5) may well be different for each game. Sierra will have added
  more commands to their set as they went along."* [verified] The 181 quoted is Manhunter 2's.
  Test conditions occupy a separate, much smaller range — `0x00 - 0x12`, so at most nineteen —
  and that range does **not** grow the way the action range does.

  This is the only measurement in this catalog of a vocabulary that expands per title, and it is
  the shape [`verb/give-to.yml`](../corpus/verb/give-to.yml) exists to guard against — *"a
  vocabulary growing by accident needs a record of what it declined"*. Sierra kept no such record.
  Some of what accreted stopped doing anything: *"Be aware that commands 175, 176, 178, 179 and 180
  of the last version of AGI (ver 3.002.149) do absolutely nothing."* [verified]

- **The tests, which are the part comparable to this design's predicates.** Section 4.2 names
  `equaln(n,m)`, `equalv(n,m)`, `lessn(n,m)`, `lessv(n,m)`, `greatern(n,m)`, `greaterv(n,m)`,
  `isset(n)`, `isset.v(n)`, `has(n)`, `obj.in.room(n,m)`, `posn(n,x1,y1,x2,y2)`,
  `obj.in.box(n,x1,y1,x2,y2)`, plus `compare.strings`, `said`, `controller` and `have.key`.
  [verified] The comparison family is **three relations in two operand modes** — a variable against
  a number and a variable against a variable — rather than six relations; `!=`, `<=` and `>=` are
  spelled with the `not` code (`0xFD`), which brackets a test the way `if` (`0xFF`) brackets a
  block.

  Against this design's nine predicates and six comparisons, the differences run both ways. AGI has
  two **spatial** tests and this vocabulary has none. It has strings and this one has none by the
  admission rule. It has `said` and this one has no form of it, because the site *is* the verb and
  the target. What it has no form of is `chosen <option>` and `count <item>` — a conversation's
  history and a quantity — and both of those are things this corpus admitted on evidence.

- **256 flags and 256 variables**, fixed and numbered. [verified] Nothing declares them, nothing
  names them, and nothing scopes them: an author writes `f5` and remembers. That is the exact
  complement of this design's admission rule — *every operand names something the corpus already
  declares* — and it is the first concrete picture this corpus has of what the other choice costs.

- **The vocabulary file, which is the input-side bound.** `WORDS.TOK` is *"used to store the games
  vocabulary, i.e. the dictionary of words that the interpreter understands"*, stored with each
  word compressed against the previous one. [verified] Word group 0 is the ignored words — *"the"*,
  *"at"* — group 1 is `anyword`, and group 9999 is `rol`, rest-of-line, so that
  `said("fight","rol")` matches *"fight"*, *"fight beast"* and *"fight beast with sword"* alike.
  [verified] `said` is condition `0x0E` and is the one test with a variable argument count: *"the
  byte following this byte gives the number of two byte words that follow as parameters"*, and
  those parameters are word-group numbers from `words.tok`. [verified]

  So the parser's vocabulary is a finite declared list, and the wildcards are in it as reserved
  numbers. [`prior-art/sierra-sci.yml`](../corpus/prior-art/sierra-sci.yml) called this shape *"open
  at the surface and closed in fact"* and said the corpus had no word for it. The word is in the
  file format: **the closure is declared, and it is declared on the input.**

## What this document holds that nobody here has looked at

The full command list in 6.2 and the reference in 4.2 — roughly 180 action commands, read here only
for the test half and for the fact that the range grows. The partition
[`.yidam/decisions/forty-eight.yml`](../decisions/forty-eight.yml) applied to AGS's 48 actions and
`scumm-v5-opcode-reference.md` applied to SCUMM's 84 has **not** been applied to these, and it is
the obvious third run of the same instrument: it would say what share of the largest of the three
command sets writes world state a predicate can read, against what share only draws.

Also unread: the PICTURE, VIEW and SOUND resource formats (sections 7 to 9), the resource file
layout (section 5), and the interpreter internals (section 3). None of those is an authoring
surface, which is why none was opened.
