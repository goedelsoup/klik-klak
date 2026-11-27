---
title: what the specimens found
status: proposal
reconciled: d580d870af7a15f128ecc62f51777dea682622b1
rests_on:
  - design/bounded-expression-language.md
  - .yidam/corpus/actor/player-character.yml
  - design/specimen/lighthouse.game
  - design/specimen/ferry.game
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/corpus/interaction.ont.yml
  - .yidam/corpus/interaction/binary-binding.yml
  - .yidam/corpus/interaction/consuming-binding.yml
  - .yidam/corpus/interaction/acquisition-binding.yml
  - .yidam/corpus/inventory-item.ont.yml
  - .yidam/corpus/inventory-item/stackable-item.yml
  - .yidam/corpus/conversation.ont.yml
  - .yidam/corpus/world-flag.ont.yml
  - .yidam/corpus/world-flag/latched-flag.yml
  - .yidam/corpus/failure-mode/unwinnable-state.yml
  - .yidam/corpus/verb/give-to.yml
  - .yidam/catalog/ags-manual.md
  - .yidam/catalog/lsl5-source.md
  - .yidam/skills/reachability.md
  - .yidam/skills/vocabulary-coverage.md
derives_effects_from:
  - interaction:sets
  - interaction:opens
  - interaction:consumes
  - conversation:sets
  - conversation:returns-control-to
  - inventory-item:acquired-by
  - actor:occupies
---

# what the specimens found

Six games authored in the language proposed by
[`../bounded-expression-language.md`](../bounded-expression-language.md), each written to
break it.

| | [`lighthouse.game`](lighthouse.game) | [`ferry.game`](ferry.game) | [`nightwatch.game`](nightwatch.game) | [`patrol.game`](patrol.game) | [`lock.game`](lock.game) | [`relay.game`](relay.game) |
|---|---|---|---|---|---|---|
| written for | *can this say a game at all?* | *does `count item` earn its place?* | *does `cutscene` earn its place?* | *does an actor's position need to be state?* | *does control transfer need more than a flag?* | *does `or` earn its place?* |
| committed defect | consumption | ordering | consumption | ordering | ordering | consumption |
| reachable states | 91 of 12,288 (0.74%) | 20 of 120 (16.7%) | 14 of 64 (21.9%) | 20 of 128 (15.6%) | 58 of 768 (7.6%) | 480 of 1,536 (31.3%) |

**The sixth is the widest of them and that is what two routes to one place means.** Every other
specimen reaches under a quarter of its declared space; this one reaches **31.3%**, because a
disjunction is an ordering constraint removed. It is also the most stranded — one consumption trap
that kills 28 states where the largest before it killed four — for the same reason: a game with
more reachable states has more of them to lose.

**And a fifth thing they found is not in that table, because nothing counted it for eleven
phases.** The five carry 154 authored response clauses and **four of them are selected in no
reachable state** — written, and unreachable. Three are in `lighthouse.game` and are not
authoring slips: a binary entry is keyed by the object carried, so a binding that spends that
object cannot be asked again and therefore cannot decline, and the *already done* refusal under
each of the three is dead the moment it becomes true. The fourth is `ferry.game:61`, the
ferryman's *"He is already unmooring."*, which is true only after a crossing the specimen
documents as one-way — so the flag becomes true and the room holding the line that reads it
becomes unreachable in the same act. `.yidam/decisions/orphaned-lines.yml`, and
[`../bounded-expression-language.md`](../bounded-expression-language.md) §4 is where the
prediction that this would be detectable was written down.

**Each was written to answer one question and none of the six answered only that.** The list
below is in the order they were written; `patrol` is the first written *after* a real game's
source had been read, and it is the only one that exists because a shipped game was found doing
something this vocabulary could not say.

Those fractions were published as 0.05%, 4% and 5.5% for several phases. They were too small by
2^(takeable objects): the metric multiplied in one bit per acquisition site for the checker's own
`taken` bookkeeping, which the item count already determines. See **Scale**, below, for how that
was found and what it cost.

**The single most useful thing either produced is a fact about method.** A specimen measures
where its author could not write something down, and where a tool disagrees with a game. It is
an excellent detector of trouble and a poor diagnostician of it, and every finding below had to
be weighed against what the ontology already said before it counted as a gap.

## lighthouse — the effect vocabulary

Nine sites could not be expressed on first writing: an item leaving the inventory (4), an
object leaving the room (2), an option's availability changing (2), a conversation being left
(1). This document concluded the effect vocabulary "was the whole problem".

**One of the four was real.** The other three were already expressible — an object leaving the
room is entailed by `carried-form-of`, option availability is a conversation-scoped
`world-flag`, and leaving an exchange is `returns-control-to`, which `cutscene` had declared
since it was seeded. Roughly one site in four was what it said it was. The reasoning is in
`.yidam/decisions/effect-vocabulary.yml`.

**The first of those entailments went unchecked for four phases.** An object leaves the room when
`is <item>` and the `give` on that hotspot name the same item, and nothing said so: make
`nightwatch.game`'s wire hand over the ledger and the analysis goes from fourteen states and two
strandings to **six and none**, still winnable, without a word at resolve. An entailment is only
as good as the agreement it rests on, and this list did not ask what its three rested on. The pair
is checked now — `.yidam/decisions/found-in.yml`.

The one that was real mattered more than the count suggested. The lighthouse's deliberate
defect is a can of oil poured into a rusted hinge instead of the lamp — `unwinnable-state` by
**consumption**, the first cause it names — and consumption was the gap. So the game contained
exactly the defect a checker exists to find, and the language could not express the mechanism
producing it. `interaction -consumes-> inventory-item` closed that.

## ferry — the tally

`count item` was admitted to the design to answer *"count an inventory"*, one of the three
cases the open question named as forcing an escape hatch, and the lighthouse never reached for
it. So the ferry is a tally: three coins for a river crossing.

**It was written first with distinct items**, which is what the vocabulary had. Three coins are
three declarations carrying identical labels, the toll is a three-term conjunction, and the
verb has to pick one coin arbitrarily as its second object while the others go by side effect
— `use-with coin-a`, with coin-b and coin-c spent invisibly. Each of those grows with the tally.

**What does not reduce is "any three of five".** With distinct items and no quantifier that is a
ten-term disjunction of three-term conjunctions, growing combinatorially. The design excluded
quantifiers on the stated grounds that they need a collection and the vocabulary has none — so
`count` earns its place exactly if an item may bear a quantity, because a quantity-bearing item
**is** that collection. See
[`stackable-item`](../../.yidam/corpus/inventory-item/stackable-item.yml).

AGS had already told this corpus so, in a phrase sitting in the ledger for five phases:
`AddInventory` "adds **one quantity** of the specified item"
([ags-manual](../../.yidam/catalog/ags-manual.md)) [verified].

**And the cost is not authoring cost.** Every other primitive leaves a game's state space
finite. Quantity does not: an acquisition source that can fire twice hands out an unbounded
supply, and the analysis never terminates. Loops and recursion were excluded so that
*evaluation* terminates; this is *analysis* terminating, a different property that nothing had
been protecting. Items now declare a ceiling as counters declare a range.

## nightwatch — the sequence

Four rooms, two items, two latched flags, and one question: **does `cutscene` earn its place?**

[`cutscene.ont.yml`](../../.yidam/corpus/cutscene.ont.yml) seeded the class while doubting it and
named the reduction to argue against — "an interaction whose response is long and whose input is
locked". This game wants a sequence twice, in two different shapes, so the argument has something
to be about rather than being settled by taste.

**The click-triggered shape reduces, completely.** `study.case` is a whole scripted beat: the wire
goes in past the wafers, the lock turns over, the lid comes up on its own weight. It is *one
binding with one effect*. Everything that makes it a scene is in the `say`, and `say` is
presentation — [`effect-list-semantics`](../../.yidam/decisions/effect-list-semantics.yml) puts it
outside the state model and `Binding` has no field for it. [inference] So "the response is long"
is not a claim about the vocabulary. And "the input is locked" is vacuous: an atomic binding never
yields control, so there is nothing to lock.

**The arrival-triggered shape does not reduce, and what fails is the trigger.** The watchman
intercepts you for *arriving* in the hall carrying the ledger. No binding can say that: an
`interaction` site is a room and a target plus a verb, and there is no verb for "got here".

The workaround was to hang the sequence on the click that precedes each arrival — and it cost
exactly what the specimen was built to make countable:

| | as first written | with `on-enter` |
|---|---|---|
| bindings for the sequence | **2** — one per door into `hall` | 1 |
| bindings for the ordinary passage | 2 | 2 |
| copies of the sequence body | **2** | 1 |
| reachable states / transitions | 14 / 31 | 14 / 31 |

`when` gates a whole binding and `else` carries only a line, so "if the alarm is on do this,
otherwise pass through" was two bindings, not one — and that pair repeated per entrance. A room
with five doors would have carried five copies. [inference]

**The game is now written the second way**, and the last row is the point: the numbers did not
move, so the primitive is faithful to the game rather than a different game that happens to
parse. See [`arrival-trigger`](../../.yidam/decisions/arrival-trigger.yml).

**The duplication is a diagnostic cost too, which is the part that was not expected.** One
authored intent, reported once per copy:

```
  line   68  walk-to on yard.hall-door   [consumption]  spends ledger — 1 state(s)
  line  157  walk-to on study.hall-door  [consumption]  spends ledger — 1 state(s)
```

Nothing in that output says those are one sequence.

**And the copies could come apart silently.** Drifting one copy's guard — `when alarm` to
`when case-open` on the second only — genuinely changed the game: 15 reachable states instead of
14, 33 transitions instead of 31. **Every finding stayed identical.** Same two strandings, same
lines, same cause, same counts, same examples. Only the totals moved, and nothing said that a
total moving means two copies of one sequence disagree. That is
[`puzzle/state-gate`](../../.yidam/corpus/puzzle/state-gate.yml)'s complaint — "a gate depending
on three flags is stated three times, or more, and nothing checks that the statements agree" —
arriving from a direction that node did not name.

That test is gone, along with the thing it tested: there is one copy now and nothing to keep in
step. **The two strandings remain and are no longer duplication.** There are two doors into the
hall, so there are two ways for a player to make the mistake, and a report naming both is
describing the game. What changed is that neither names the sequence — the blame lands on the
door the player chose, because a forced binding is not something anybody chose.

**The verdict is in
[`cutscene/scripted-sequence`](../../.yidam/corpus/cutscene/scripted-sequence.yml).** In short:
every property the class was seeded on reduces, and what does not reduce is a trigger that
belongs to no class — the same arrival trigger
[`walk-to`](../../.yidam/corpus/verb/walk-to.yml) needed for exits two phases ago.

**What this specimen did not reach**, stated because a reduction argued from one small game is
worth exactly what it cost. The watchman moves — he crosses the hall, takes the ledger, opens the
far door — and *all of that is prose in a `say`*. Nothing in the game depends on where he is, so
this specimen never tested whether an NPC's position needs to be state, which is the one thing a
real cutscene does that nothing here does.

**And `cutscene.trigger`'s third case is still untested here and is no longer undocumented.** This
document guessed *undocumented in either engine* from not having looked, and an engine has since
been read: AGS's `repeatedly_execute` is a declared event slot running every game loop, and the
manual's own worked example on that page **is** a threshold — a counter incremented, then tested
against 800 ([ags-manual](../../.yidam/catalog/ags-manual.md)). [verified] What is documented is
the *occasion*; what goes in it is arbitrary script, and there is no declarative form of *when this
value reaches that* in either era of that system. So the sentence was reaching for something true
and the truth is narrower and better evidenced than a silence.
`.yidam/decisions/threshold-trigger.yml` closed the question outright — **`cutscene` has no open
questions left**, and this document went on carrying one of them for two phases.

## lock — the second character

Three rooms, two actors, one item, five flags, and one question: **does transferring control
between characters need vocabulary beyond a world flag naming the controlled one?**

[`actor/player-character`](../../.yidam/corpus/actor/player-character.yml) has asked it since the
root commit and it is the oldest question in the corpus.
[`resolver-kind`](../../.yidam/decisions/resolver-kind.yml) classified it as the one question here
that only a **specimen** can close and observed that none of the four above switches at all. This
game does exactly what the question proposes — one flag, `ashore`, naming the controlled character
— and the answer is that the flag is the cheapest part of it.

**The player is not an actor in this format, and that is the whole finding.** A player is a
nameless slot with a room and an inventory; an `actor` is a name with a room and no inventory.
`Effect::Move`'s own doc comment has said so since the day it took an operand — *"`None` is the
player, and the asymmetry is the point"* — and read as a note about NPCs it is correct. Read
against a game with two controlled characters it is the defect.

The node said this first and in the opposite direction, from UFO, at genesis: *"modeling the
controlled one as a distinct kind means the transfer is a change of kind — which is exactly the
thing UFO says a kind cannot undergo."* **The corpus made `player` a role and the format made it a
kind**, and nothing noticed for eleven phases because no game had two.

It is visible in two lines of this game. `start` and `holds boatman` sit one under the other in the
same room section, saying the same thing about the same person, and nothing relates them —
declare the boatman in a different room from the one play begins in and the file parses clean,
resolves clean, plays, and is winnable at 61 states instead of 58, with nothing attributing the
difference to the contradiction. That is [`found-in`](../../.yidam/decisions/found-in.yml)'s shape,
found this time in a fact the format *cannot* hold rather than one it forgot to.

**A transfer is not a verb applied to a target, so it has no site.** Layer 0 offers two — a verb on
a hotspot or an actor, and arrival in a room — and switching is neither. Four of this game's six
switches hang on scenery declared for the purpose: `mate-above`, *"the mate, up on the coping"*, is
a hotspot standing for a character who is **not in the room**, because an `actor` site is live only
where the actor is, and the whole point is that she is elsewhere.

**Both games that shipped this agree it is not a binding**, and they disagree about everything
else. *Maniac Mansion* made it a verb — a **New Kid** verb that displayed a list of the kids.
*Thimbleweed Park*, by the same designers thirty years later, took it out of the verb space
altogether: number keys 1 to 5, *"There is no UI for the character switching yet, and we don't know
exactly how we're going to do it, but it's completely functional"*, and the reason the old answer
was refused is a budget — *"We don't have the space for another verb"*
([twp-dev-diary](../../.yidam/catalog/twp-dev-diary.md)). [verified] One put it in the verb bar and
one refused to, and neither hung it on a target.

**The destination is a literal, and a binding has one guard and one effect list.** So a switch
cannot say *go to where she is*; it names a room, and a site is needed per ordered pair of
positions. The bound is **twice the product of the two characters' room sets** — two rooms each is
the smallest map that lets both of them move, and it already wants eight sites. This game writes
six switch sites, and the two it leaves out are the pair that never co-occurs.

Two of the six cost nothing. Once both characters stand in the same room an `actor` site is live
and the transfer is one binding with **no destination in it at all** — so the only natural site
control transfer has here is the case where nobody has to move.

**An item cannot change hands.** `inventory-item` has three states — unclaimed, carried, spent —
and none of them is *in the other character's hands*; `is <item>` gives an item exactly one
hotspot, so a thing put down cannot be picked up by somebody else, and `spend` is terminal. The
windlass is an item for two bindings and a flag for the other four: it goes up onto the coping as
`spend windlass` plus `set windlass-up`, and the visible cost is one line — the mate winds the
paddle with `walk-to` and not with `use-with windlass`, **because she cannot carry it**. The binary
verb is lost with the item.

That is the one of these findings a shipped game reports as load-bearing. Asked whether the GIVE
verb exists and whether item transfers between characters are necessary for puzzle solutions,
Gilbert answers *"Yes to both questions"*
([twp-dev-diary](../../.yidam/catalog/twp-dev-diary.md)). [verified] Characters there have
individual inventories and what is picked up goes into the active one — which is the second thing
a flag cannot do, and the engine does it without the author writing anything, because
`currentActor` is not a value an author reads but a parameter every primitive is threaded through.

**The flag itself is read three times and all three are in one room.** Everywhere else the room the
player is standing in already says who they are, because the boatman cannot get out of the chamber
and the mate cannot get into it. `lockside` at the end is the one room that holds both, and it is
the only place `ashore` carries anything — which is the `holds <item>` question asked from the
other side: not *does anything read it* but *is there anywhere it could be read*.

**And it is the first flag any specimen has ever put back.** Six games declare seventeen flags and
until this one no binding had ever cleared a single one; `clear-flag` sat on the never-written list
for eleven phases and `world-flag.reversible` has had a value with no authored instance since
genesis. The reason is not a gap in the games. Every flag in the other four is a thing that
happened to a world, and worlds do not un-happen — a crate is not re-nailed, a toll is not un-paid.
**Which character the player is, is the only fact in any of these games that goes both ways**, and
it is not a fact about the world at all.

**The last measurement is the one nobody was looking for.** The four games before this wrote the
binary connectives **once between them** — a single `and`, no `or` — which made
[`interaction-table`](../../.yidam/corpus/data-format/interaction-table.yml)'s claim that
composition is this language's one advantage an argument rather than a measurement, and this
document said so. `lock.game` writes eleven more, and every one is about who the player is or where
the other character is standing: `gates-shut and paddle-up and boatman in lockside and mate in
lockside`. A game with one protagonist asks about the world, and a world's facts are independent,
so its guards are single terms. A game with two asks about the world **and** about the pair.
[inference] — `crates/game-format/tests/coverage.rs`, and
`crates/game-format/tests/transfer.rs` holds the rest.

## relay — the disjunction

**Written for one question: does `or` earn its place?** Five games had written `and` twelve times
and `or` **not once**, which left the claim above — that composition is the one thing this language
has and the interaction table does not — resting on the half of composition nobody had used.

A relay hut on a moor in winter, one operator, a call to pass on. Two independent ways to get power
to the set, and one place where the light to work by can come from either of two things that are
**not the same kind of fact**: the yard lamp, which is a world flag, or the hand-lamp, which is an
item you are carrying. `carrying lamp` is read off the inventory and nothing can write it, so no
flag stands for *there is light*: a latched flag cannot be cleared when the lamp is spent, and an
unlatched one maintained by hand at every acquisition site is the drift
[`delegation`](../../.yidam/decisions/delegation.yml) found in a shipped game. Only a disjunction
joins them.

**It is the widest specimen and the most stranded, for one reason.** 480 reachable states of 1,536
— **31.3%**, where no other reaches a quarter of its declared space — and one consumption trap that
kills 28 states where the largest before it killed four. [verified] —
`crates/reachability/tests/written.rs`. A disjunction is an ordering constraint removed, and a
wider reachable set has more to lose.

**And it could not be written, three times over.** The cell check refused a disjunction that no
state satisfies; the reader lexed `(engine-running` as a name; the writer put every predicate into
disjunctive normal form, which handed the author back their conjunct **restated**. All three rest
on the same misreading of this document's own grammar — `atom := '(' pred ')'` is the fourth
production and it recurses to the top, and three separate checks read the first three and stopped.
Nothing found out for eighty-eight phases, because each check was tested against predicates authors
had written and no author had written a disjunction.
[`or-earns-it`](../../.yidam/decisions/or-earns-it.yml).

**What that says about `named-condition` is worth more than the count.**
[`restated-guard`](../../.yidam/decisions/restated-guard.yml) declined a way to name a condition and
set the terms for reopening: *a second game that restates a conjunction.* This is a third composing
game, written to need composition, and it restates nothing — because the one thing it wanted to say
twice is written once, in brackets. **A restatement is what an author writes when the language
cannot group**, so part of the pressure that primitive was weighed against was a missing production
rather than a missing primitive.

## Three defects in the calculator, all found by the second specimen

A second game is worth more than a second reading of the first, and this is the evidence.

**An ending that writes nothing was filtered out as a no-op.** The ferry's ending opens a gate
and sets no state, so the search had no goal at all and reported the whole game unreachable.
The lighthouse hid it: its ending also sets a counter and spends an item, so it survived the
filter for reasons unrelated to being an ending.

**The cause classifier was reading the effect list.** A binding that spends *and* moves carries
both kinds of effect, and reading the cause off the list picks one by fiat — the ferry's toll
spends three coins and crosses a river, and the coins are not why the player is stranded. It
now takes the spending away and asks whether that rescues the state. Both specimens classify
correctly, one cause each.

**The search did not stop.** A mutation clearing the flag that guarded an acquisition source
made it unbounded, and a test hung for two minutes. That is the finding above, discovered by
running rather than by reasoning.

## Grammar coverage

**The heading is half right now and the premise is why.** `bounded-expression-language.md` used to
specify one thing in one notation; it specifies two, in two, because the two halves of this format
are not the same kind of object. The predicate layer has a grammar — it nests, and `unary := 'not'
unary | atom` has no bound on depth. The declaration half has a **table** of twenty written line
forms, because a declaration does not nest and BNF for it is twenty productions of one alternative
apiece. `.yidam/decisions/line-format.yml`.

The count below spans both and is unaffected: fifty constructs is fifty keywords, effects,
comparisons and predicate atoms, and which document specifies each was never what it measured. What
the split does change is the sentence at the end of this paragraph. Grouping parentheses are in the
grammar — `atom := '(' pred ')'` — and are not in the fifty, which is the distinction it was
reaching for and did not have the words for.

Unexercised across all six games: **none.** Every one of the fifty constructs the surface declares
has an author behind it, which it did not until the sixth game. [verified] —
`crates/game-format/tests/coverage.rs`.

They came off in three batches and none by being argued for again. `count` went when a game was
written to tally; `clear-flag` when a game needed to hand control back; and the last five —
**`or`, `exit`, `!=`, `<=`, `>`** — together, in one game written to need the first of them
([`relay.game`](relay.game), [`or-earns-it`](../../.yidam/decisions/or-earns-it.yml)). The last
four are cheap: `exit` is a declaration that arrived after the first five specimens had written
its expansion longhand, and the three comparisons are one finding — an ordered comparison is only
distinguishable from `=` when a counter holds values *between* its bounds, and a counter gets
values only from a literal in an effect, so a staged counter is the whole of the demand for them.

**Grouping parentheses are the exception that proves what a census over specimens can miss.** They
came off the count by being counted: the check reads constructs the surface declares, and
punctuation is not one. But `atom := '(' pred ')'` has been in the grammar since it was written,
`atom()` has always accepted it, and **no game had ever written a group** — because none had
written a disjunction. Three separate checks concluded from the first three productions that the
reader spells a disjunction of conjunctions and cannot build anything else; the fourth production
recurses to the top. The writer was normalising every predicate to close a gap that was not there,
and what normalising cost was a restated conjunct. All of that surfaced the day an author wrote
`aerial = 2 and (engine-running or carrying lamp)`.

**The proposal's own first open item is settled and this is half of what settled it.** Section 5
called the surface abstract and forwarded a reader to
[`interaction-table.yml`](../../.yidam/corpus/data-format/interaction-table.yml) for an unresolved
tension between keying and reading like writing; the guarded response list — `when <predicate>
"<line>"` — does both on one line, and six games are written against fifty constructs.

**And the marker at the forwarding address has been settled, and a sixth game re-took the
measurement without moving it.** It asked
whether a file that carries the writing reads like writing, which is a measurement over 904 authored
lines rather than a position anybody can take. Taken: **a response line is 84% sentence** — 51.9
characters of writing behind 9.2 of prefix — where the same 154 responses keyed as one flat row each
would be 66%, and a binding here carries **2.02** characters of writing per character of frame
against **0.56** in a shipped, hand-authored game counted the same way. The keying is moved above
the line rather than removed, at a median twenty-two-line reach back to the room.

These files are the material and they were the material all along; what they were waiting for was
somebody to say what counting them would answer. `.yidam/decisions/resolver-kind.yml`,
`.yidam/decisions/reads-like-writing.yml`, `.yidam/decisions/settled-surface.yml`.

**One specimen is much better evidence that a primitive was needed than that one is not**, and
five is only a little better. A hundred and twenty bindings across five deliberately small games is
a thin basis for removing anything — and the list above is the measurement, not the backlog.
`.yidam/decisions/unwritten.yml`.

## The verb vocabulary

**Used and not declared: `walk-to`**, at **six** sites across the two games — four in the
lighthouse, two in the ferry. This document said eight for two phases; the number was never
counted from the files and is corrected here by counting them.

Movement is a verb in both systems this corpus has read — AGS ships Mode 0 (Walk) and its
Tumbleweed template carries `eMG_WalkTo` ([ags-manual](../../.yidam/catalog/ags-manual.md))
[verified] — and no `verb` node declared it. That was `vocabulary-coverage`'s second list, and it
had one member for three phases.

**That skill is retired, and this document is the reason.** Both of its lists were produced here
rather than by the calculator, over games rather than over the corpus it was specified to read —
and its own page came to say that a coverage report over verbs *"cannot tell you which class the
gap is in, and this one was read as though it could"*, which is exactly what happened to the
member below. `.yidam/decisions/calculators.yml` retires it and `collapse-signature` beside it: all
three calculators approved at genesis were answered by reading a game, and a specimen is corpus
input for none of them.

**Now assessed, and the answer is that these six sites are not verb bindings at all.**
[verb/walk-to](../../.yidam/corpus/verb/walk-to.yml) finds that no engine lets an author write a
response to walking: AGS's `unhandled_event` matrix has fallback rows for cursor modes the manual
never names and no row for Walk. [verified]

The specimens say the same thing from this side, and it is checkable rather than documentary.
**Every one of the six carries `do move <room>` and nothing else.** The verb contributed no effect
— `move` is not its own, and the ferry's toll moves the player under `use-with coin`. It
contributed a *trigger*. What these sites are writing is an **exit**, which
[`room -connects-to-> room`](../../.yidam/corpus/room.ont.yml) has declared since genesis and this
game format has no syntax for. [inference]

One of the six is the reason `connects-to` is not simply the answer:

```
    walk-to:
      when  door-open
      do    move base
      else  "Locked, and the salt has taken the hinges."
```

A guard and a refusal, on an edge between two rooms. That is a binding whose trigger is arrival
rather than a verb applied to a target — and the interaction table indexes on verb and target, so
it has nowhere to put one.

**Built, and it did not need a third site.** `exit <name> to <room>` is a declaration that expands
at parse time into exactly the binding above, so the guard and the refusal are written under it and
the table indexes nothing new. The specimens turn out to supply half the evidence for its
shape without having been asked: **23 exits, 17 bare and 6 carrying a guard or refusal**, against
Delores's 20, 18 bare and 2 gated. Same count, same dominant shape, arrived at independently.
`.yidam/decisions/exit-archetype.yml`.

**And for five games no specimen wrote it.** `exit` appeared zero times, and `lock.game` added
eleven more `walk-to` sites of the long form it was built to replace. That was this document's own
last question — *whether a specimen should be revised to exercise a primitive it argued for* —
with a second instance, in a section eleven headings above the one that asks it.

**It is answered from the other side and the question is left standing.**
[`relay.game`](relay.game) was written after the declaration existed and reaches for it at every
room connection it has; it is the only specimen that writes one. [verified] —
`crates/game-format/tests/written.rs`. So a primitive can leave the unexercised list without
anybody revising a specimen to exercise it — **by somebody writing the next game** — and that says
nothing about whether the five earlier ones should be rewritten. What it does say is that the list
empties on its own if games keep being written, which is an argument for writing them rather than
for going back.

**Declared and not used: `give-to`, and the fifth specimen is where that verdict's evidence base
shows.** No natural site arose in any of the four, and the silence was counted rather than
observed: **120 bindings across 904 lines and not one is a `give-to`**, where the other five verbs
are all used. What none of the four could supply is a *second* character to give to — giving has no
natural site in a game with one protagonist, which is the one condition every specimen shared. The
fifth has two and wants the hand-off badly enough to spend an item to fake it, and still writes no
`give-to`, because the verb was withdrawn and because what it wanted was not a verb. [verified] —
`crates/reachability/tests/verbs.rs`, and
[`verb/give-to`](../../.yidam/corpus/verb/give-to.yml) is where it bears weight. This document
said *"either game"* for four phases after there were four; the node it fed took the observation,
called it *"weak on its own — one small game"*, and did the counting, and the count did not come
back here.

## What the corpus has no class for

Both specimens need `start` and `ending`, which belong to the game rather than to any part of
it. Fifteen classes model rooms, verbs, bindings, flags and failures, and none models the
artifact they compose into. Noted rather than proposed — and noted more carefully after
`effect-vocabulary`, since most of what this document last called missing was not.

## Using them as fixtures

**220 tests**, in `crates/reachability/tests` and `crates/game-format/tests`. This document said
thirty-eight for six phases; the figure was never derived from anything and is held by a check now
(`crates/design-gate/tests/watermark.rs`).

**And three of them are here to say what these games could not have shown.**
`crates/game-format/tests/declared.rs` counts the sixteen items across the six and finds every one
reached by a binding — not because the games were written that way but because an item in this
format is a name and a ceiling, so no specimen however authored could have produced the
counterexample [`portable-object`](../../.yidam/corpus/inventory-item/portable-object.yml) asked a
fifth specimen for. `.yidam/decisions/declared-reach.yml`.

- **lighthouse, winnable** — and **unwinnable by consumption** if the oil goes into the hinge.
- **ferry, winnable** — and **unwinnable by ordering** if the river is crossed without the
  lantern, because the boat does not come back.
- **nightwatch, winnable** — and **unwinnable by consumption** if the hall is entered carrying
  the ledger, reported once per door because there are two ways in.
- **two rooms that throw each other**, reported as a chain that never settles rather than
  chased — the decidability guard `on-enter` needed.
- **lighthouse + a two-line mutation** for ordering, applied in memory: `do clear door-open` on
  the quay door's `walk-to` and `when door-open` on the far side's.
- **ferry + a one-line deletion** for an unbounded supply — an omission rather than an edit,
  since `effect-list-semantics` made the old set-then-clear version a dead write.
- **a cleared latch**, reported as contradicting its declaration — cleared from a *different*
  binding, because within one list it is a dead write and no latch is broken.

The first two carry exactly one defect and assert `strandings.len() == 1`; a fixture with two
makes a checker's output ambiguous about which it found.

**`nightwatch` asserts two, and it means something different now than when it was written.** It
used to mean "one authored defect, reported once per copy of a duplicated sequence". It means
"one authored sequence, and two doors a player can walk through to reach it". The number is the
same and the reason is not, which is worth noticing: an assertion that survives a change to what
it is about was not testing what it looked like it was testing.

## Scale — the claim these four could not test, tested

Four games small enough to enumerate by hand cannot say whether enumeration scales, and this
document said so for six phases. It read *"three"* for several more, having been written when
there were three and not revisited when `patrol` arrived. `crates/reachability/tests/scaling.rs` **generates** the large
games instead. That costs realism and buys a curve: one game is one point, and tractability is a
question about growth.

Two families, both legitimate adventure-game shapes.

| | **corridor** — door *i* needs the key behind door *i-1* | **hub** — *n* tokens, no order, all wanted at one gate |
|---|---|---|
| reachable states | 3n+1 | **2ⁿ** |
| fraction of declared space | → 0 | **100.000%, at every size** |
| n = 16 | 49 states | 65,536 states |
| n = 22 | — | 4.2 million states, 36 seconds |
| n = 128 | 385 states, 16 ms | — |

**The collapse is a property of the game's dependency structure, not of the language.** Same
language, same checker; the difference is entirely whether the author ordered the puzzles. And a
hub is not a contrived shape — *find the three map pieces*, *collect the four keys*.

**The sharper half: both families are made entirely of monotone variables.** Nothing is ever
unset in either. So monotonicity is not what makes the search cheap —
[`latched-flag`](../../.yidam/corpus/world-flag/latched-flag.yml) had said it was, and that claim
is withdrawn. Monotonicity bounds the *path*, not the size of the set, and a lattice over *n*
independent monotone variables is every combination.

**And the metric could not have found this.** Counting `taken` gave the hub 2ⁿ states against a
reported 4ⁿ space — a "collapse" to 2⁻ⁿ that was pure redundancy, and which looked exactly like
the corridor's. The old number reported ~0% for a linear family and an exponential one alike;
the corrected one separates them completely. A number that measures the representation rather
than the game is worse than no number, because it looks like evidence.

**What survives.** Decidability is untouched — the space is finite and the search terminates,
which is what a bounded predicate layer buys and what no escape hatch can offer.

### And the narrower question has an answer

*How ordered are a real game's puzzles?* Ron Gilbert invented the notation for arranging them —
the Puzzle Dependency Chart — and states its shape as a rule: "Solving one puzzle should open up
2 or 3 new ones", each act ending in "a bottle neck to the next act", with **both** extremes
named as failures, "too linear, or too unwieldy with choice (also bad)"
([grumpy-gamer-pdc](../../.yidam/catalog/grumpy-gamer-pdc.md)). [verified]

A third generated family is that shape — *k* acts of *w* parallel puzzles, each act gated on all
of them. Holding the puzzle count at forty-eight and varying only the width:

| acts × width | reachable states |
|---|---|
| 48 × 1 | 97 |
| 24 × 2 | 97 |
| **16 × 3** | **129** |
| 12 × 4 | 193 |
| 8 × 6 | 513 |
| 6 × 8 | 1,537 |
| 4 × 12 | 16,385 |
| 3 × 16 | 196,609 |

Same forty-eight puzzles, a factor of two thousand. **The reachable set is linear in the number
of puzzles and exponential in the chart's width.** At the prescribed width the count stops
mattering: six hundred puzzles in acts of three is 1,601 states and 73 milliseconds — larger than
any published adventure game, and cheaper to check than the lighthouse.

So the checker is tractable exactly on games whose charts are the shape their designers already
aim for. That is contingent where the collapse claim was pretending not to be, and it is a
better position for being so. [inference]

**There is a measured number now, and it did not need a chart.** Delores's dependency graph, read
off its source: 354 units, 115 edges, a step that unblocks anything opening 2.45 on average and 2
at the median — but **43% opening exactly one**, and two gated exits in twenty where the argument
wants a bottleneck per act. [verified] See `.yidam/decisions/measured-width.yml`. What is still
missing is a *second* game, and a full-length one: Thimbleweed Park's source is not public and its
chart is blurred on purpose. [open]

**The second game has been read, and it is full length.** *Leisure Suit Larry 5*'s source
([lsl5-source](../../.yidam/catalog/lsl5-source.md)) is eight acts behind gates its code
enforces, and its whole-game antichain, 191, is its act widths added together, 198. [verified]
So the `acts × width` shape in the table above is the shape a full-length game has. Its `w` is
the part still unsettled: 3 to 35 units between gates, at most 17 of the designer's scored steps.
That sits between the table's middle rows and past its last one, and units over-count puzzles.
See `.yidam/decisions/second-game-width.yml`. [inference]

## What these specimens are not

- **Not evidence the language is pleasant to write.** All three were written by the party that
  proposed it, which is the weakest possible authoring trial.
- **Not a test of scale, and the test of scale has since been run without them.** See below.
- **Their original analysis is not runtime evidence.** The specimens now run through
  [`engine/play`](../../engine/README.md); its transcript tests and graphical client provide
  separate evidence about execution. The counts here describe what was authored and computed.

## patrol — a person standing in a doorway

Written for one question, from an `[open]` two classes had been carrying:
[`cutscene/scripted-sequence`](../../.yidam/corpus/cutscene/scripted-sequence.yml) said it did
not know whether a non-player actor's position needs to be state, from a specimen too small to
tell, and [`actor/non-player-character`](../../.yidam/corpus/actor/non-player-character.yml) is
founded on an actor being able to "occupy a position that changes".

**The demand was already visible on disk.** `nightwatch.game`'s watchman carries a `look-at`
reading *"Not here. He is somewhere down the hall"* — an actor site sitting in a room the actor
is not in, with prose covering for the format.

Two primitives, added together because **an effect nothing can read is not state**:
`do move <actor> to <room>` writes `actor -occupies-> room`, `when <actor> in <room>` reads it,
and `holds <actor>` in a room section says where they start.

**The committed defect is a new mechanism for an old cause.** The porter stands in the corridor
and will not let you into the strongroom; the kettle brings him to the canteen; the key is in the
canteen. Boil before you take the key and you are stranded — reported as `ordering`, and blamed on
**leaving** the canteen rather than on boiling it, which is correct: while you are still in there
with him, nothing is lost.

### what it cost, and where the cost came from

An actor is tracked when a predicate names them **or an effect moves them**, and the second
condition had to be discovered rather than designed. `offered` gates an actor's site on where the
actor is — so a site is itself a read, and there is no such thing as moving an actor nobody reads.

An actor nobody moves and nobody asks about is absent from the state altogether, which is every
actor in the three specimens above; their figures are unchanged by this addition. A game pays
rooms^n for the *n* actors it actually moves, and nothing otherwise.

### what it did not settle

`nightwatch` still says its watchman is down the hall in prose. Rewriting it to use the primitive
would make the specimen test what it was written to say it could not, and would move figures three
phases of records quote. It has not been done, and whether a specimen should be revised to
exercise a primitive it argued for is a question this directory has not had to answer before.
