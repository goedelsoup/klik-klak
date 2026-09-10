---
title: A bounded expression language for authored interaction
status: proposal
rests_on:
  - .yidam/corpus/data-format/interaction-table.yml
  - .yidam/corpus/world-flag/boolean-flag.yml
  - .yidam/corpus/world-flag/counter-flag.yml
  - .yidam/corpus/conversation.ont.yml
  - .yidam/corpus/conversation/topic-list.yml
  - .yidam/corpus/interaction/acquisition-binding.yml
  - .yidam/corpus/interaction/binary-binding.yml
  - .yidam/corpus/interaction/default-binding.yml
  - .yidam/corpus/failure-mode/unwinnable-state.yml
  - .yidam/corpus/failure-mode/orphaned-dialogue-branch.yml
  - .yidam/catalog/ags-manual.md
  - .yidam/catalog/scumm-v5-opcode-reference.md
  - .yidam/catalog/ags-272-manual.md
  - .yidam/catalog/ags-source.md
derives_effects_from:
  - interaction:sets
  - interaction:opens
  - conversation:sets
  - cutscene:sets
  - cutscene:returns-control-to
  - inventory-item:acquired-by
  - actor:occupies
  - interaction:consumes
  - conversation:returns-control-to
---

# A bounded expression language for authored interaction

[`data-format/interaction-table.yml`](../.yidam/corpus/data-format/interaction-table.yml)
carries this corpus's most valuable open question, and it ends by narrowing it to something
nobody has answered:

> Nobody has tried the middle. AGS's escape goes straight from eight commands to a full
> scripting language; SCUMM had no declarative layer to escape *from*. Whether a bounded
> expression language — arithmetic, comparison, inventory counts, and nothing else — would
> absorb the cases that force the door open is untested by either system this corpus has read.

**The first sentence of that quote is withdrawn, and this document is the thing that has to
answer for it.** AGS shipped an Interaction Editor for room, hotspot, object and character
interactions through version 2.72 and removed it in 3.0 — declarative, closed command set,
ordered action lists, typed parameters, and at least one conditional
([ags-manual](../.yidam/catalog/ags-manual.md) [verified];
[ags-272-manual](../.yidam/catalog/ags-272-manual.md) [inference]). Section 1 now covers both
AGS experiments, because this document had been reasoning from the wrong one.

The proposal survives it and the *argument* for the proposal is weaker than it was. Read section
1 before section 2.

This is a proposal and not a finding: nothing here is implemented, no engine reads it, and it is
filed in `design/` rather than in the corpus for exactly that reason.

## 1. What AGS's two experiments actually show

The corpus read AGS's dialog script as *a bounded language was tried and was not enough*. That
reading does not survive looking at the command set as a set.

All eight, as the manual documents them
([ags-manual](../.yidam/catalog/ags-manual.md)) [verified]:

| Command | Kind |
|---|---|
| `goto-dialog X` | control flow |
| `goto-previous` | control flow |
| `return` | control flow |
| `stop` | control flow |
| `option-on X` | effect |
| `option-off X` | effect |
| `option-off-forever X` | effect |
| `run-script X` | escape |

**Not one of them reads anything.** [verified] There is no comparison, no flag reference, no
count and no conditional in the set — four commands move control, three write option state, and
one leaves. The sub-language cannot ask a question about the world, so any option whose
availability depends on the world must be switched from outside it.

That the *sub-language as a whole* therefore contains no conditional is `[inference]` and not
`[verified]`: the page introduces these as the commands "available" and never claims the list is
complete. The eight and their descriptions are quotable; exhaustiveness is not. That distinction
is the one this corpus got wrong about AGS's cursor modes, and it is being kept explicitly this
time.

`run-script X` is the tell, and it is weaker than a function call. It does not run script in
place — it "calls dialog_request event handler if one is present in the game script, passes X as
an integer parameter" [verified], so a conditional branch in a conversation leaves the
conversation file entirely and reappears in a global handler switching on an integer tag. The
other escape is whitespace: "Do NOT indent these lines with spaces or tabs. Indented lines
signify that AGS should interpret the line as a normal scripting command." [verified]

So what AGS built was a bounded **command list**, not a bounded **language**. It fails at the
first case that needs to ask anything, because it has no expressions at all — and a command list
with no expressions is not the artifact the open question is about.

### The objection, which is strong and survives

The obvious rejoinder is that AGS's authors knew their users and shipped `run-script`
deliberately. A sharper version of it is better still: **AGS's dialog script sits inside a system
that already had a general-purpose scripting language.** Adding `if` to the sub-language would
have duplicated something one indent away. The absence of a predicate is entirely consistent
with a deliberate judgement that the escape hatch *is* the conditional, and this corpus has no
evidence about which it was. Nobody here has read the AGS project's design discussions, and the
manual does not reason about the choice.

That objection does not rescue the corpus's original reading — it replaces it. The dialog script
is not evidence that a bounded expression language is insufficient, because it contains no
expressions to be insufficient.

### The other experiment, which is the one that mattered

**"A system that had no reason to try one" was this document's conclusion and it was wrong.**
AGS tried one, at this document's exact scope, and ran it for years.

Through version 2.72 a hotspot carried an "Interaction..." button. It opened a list of events;
double-clicking an event opened an action editor whose combobox defaulted to "Do nothing"; an
author picked a command from a closed list, filled in its typed parameters, and could "add as
many commands as you like to be run when an event happens", shown as an ordered tree
([ags-272-manual](../.yidam/catalog/ags-272-manual.md)). [inference]

**The list is forty-eight commands and it is not lost.** The 2.x-to-3.0 conversion still ships in
the AGS editor, so the definition of the deleted layer is in the repository
([ags-source](../.yidam/catalog/ags-source.md)). [verified] `NUM_ACTION_TYPES 48`; six argument
types including `ARG_TYPE_VARONLY`; four action flags, one of which — `AFLG_COND` — marks an
action as a predicate. **Seven carry it**: if an inventory item was used, if the player has an
inventory item, if a character is moving, **if a variable is set to a certain value**, if the
player character is, if the mouse cursor mode is, if the player has been in room. They nest: a
command carries `Children`, and the converter emits the command, the child list, then a closing
brace. Bounded at 30 events per entity, 40 commands per list, 5 arguments per command.

Then: "The Interaction Editor has been removed from AGS 3.0. When you import your game, AGS will
attempt to convert all your interactions into scripts."
([ags-manual](../.yidam/catalog/ags-manual.md)) [verified]

Three things follow, and only the third is comfortable.

**It is not the middle, and the distance is now measured rather than asserted.** Set the
forty-eight against Layer 1 below, term by term. AGS had predicates over world state — seven of
them, including one over a named variable. It had **no `or`**: nesting yields conjunction only. No
`not`. No grouping. **No comparison beyond equality** — "if a variable is set to a certain value"
takes a variable and an integer and tests one thing. No arithmetic; `Game - Set variable value`
assigns and nothing increments. Nothing counts an inventory. [verified]

Every one of those is in the grammar in section 2, and none of them is in the table. **So the gap
this document exists to fill is exactly one thing: composition.** Having predicates is not what
distinguishes this proposal from what AGS shipped. Combining them is.

That is a smaller claim than this document opened with and a much harder one to wave at. It also
means the proposal cannot be defended as *adding conditionals to a declarative layer* — someone
did that, at this scope, and it is the composition that is untried.

**Why it was removed is unknown and the manual does not reason about it.** It names migration
hazards and never the decision, and 3.0 was a complete editor rewrite — "this new editor", "a
massive improvement over the old one" — so removal is as consistent with a rewrite's scope as
with a verdict on declarative authoring. [inference] Reading it as a verdict would be this
project's characteristic error, committed twice already in this document's own sources.

**What the manual says about the difference is not expressiveness. It is time — and the table
shows how that layer handled it.** Blocking was not a default; it was a **typed argument on the
commands that take time**. `Object - Move object` and `Character - Move character` each end in a
`BOOL` the editor labelled "Wait for move to finish"; `Character - Run animation loop` blocks
always; `Game - Pause command processor for a set time` is an explicit wait; `Stop running more
commands` is an early exit from the list. [verified] Sequential execution, with duration declared
per command by the author.

That is a worked answer to a question section 5 records this proposal has never asked.

So the gap is real and narrower than this document claimed. The middle — composition, not a
conditional command — is still untried. What is no longer available is the comfort of "nobody
went near it."

SCUMM is the other half and says less. Its v5 opcode table has `print` and `printEgo` and
nothing that manages a menu of options
([scumm-v5-opcode-reference](../.yidam/catalog/scumm-v5-opcode-reference.md)) [inference] — there
was no declarative layer, so there was nothing to bound.

## 2. The proposal

Three parts, and the split between the first two is the whole design.

### Layer 0 — where a binding hangs, which used to go without saying

A binding is sited at a **verb applied to a target**, and for a binary verb a second object. That
was the whole of it until two assessments — [`verb/walk-to`](../.yidam/corpus/verb/walk-to.yml),
reading engine documentation, and
[`cutscene/scripted-sequence`](../.yidam/corpus/cutscene/scripted-sequence.yml), assessed against
a specimen — arrived at the same missing site by different routes.

There is a second site now:

```
site    :=  verb target [ object ]        -- what the player did
         |  'on-enter' room               -- where the player got to
```

An `on-enter` binding is **forced**. The player is not offered it; it fires on arrival. That makes
the state it fires from *transient* — nobody rests there — which is a property no other binding
has, and is the reason this is a site rather than a verb nobody types. It carries an ordinary
precondition and **cannot carry a refusal**: a refusal answers an attempt, and nobody attempted
anything.

**It is the second primitive here to cost decidability**, after item quantity, and by a different
mechanism. Two rooms whose entry bindings throw the player into each other never settle, so the
game has no resting state and a checker that chases it does not stop. The guard is exact rather
than a step budget: the states visited in one settling chain are remembered, and a repeat is a
cycle. Two cases are why a budget was rejected — a description-only entry binding, and one that
moves the player into the room they are already in — and both are in `crates/reachability/tests/entry.rs`.

What it bought is measured rather than argued. `nightwatch.game` carried this sequence as four
bindings across two doors, with the body written twice and nothing relating the copies; drifting
one changed the game while every finding stayed identical. It is one binding now, and the report
is unchanged at 14 states and 31 transitions. `.yidam/decisions/arrival-trigger.yml` records the
rest, including the three defects it found in the checker.

**What it does not do.** It does not discharge `walk-to`'s exit residue, and this document said it
would. An exit refuses when the player *clicks a door* — "Locked, and the salt has taken the
hinges" — and a binding that fires on entering the far room is too late to say so. The two
assessments wanted arrival at two granularities: a room, and a place within a room. Only the first
was demonstrated by a specimen and only the first was built.

### Layer 1 — predicates

Total, pure, read-only. Used wherever the vocabulary currently says *precondition*: an
interaction's gate, a conversation option's availability, a puzzle's gate.

```
pred    :=  disj
disj    :=  conj ( 'or' conj )*
conj    :=  unary ( 'and' unary )*
unary   :=  'not' unary | atom
atom    :=  '(' pred ')'
         |  flag                    -- a boolean world-flag; true iff set
         |  'carrying' item
         |  'chosen' option         -- this choice has been taken before
         |  value cmp value
value   :=  integer | counter | 'count' item
cmp     :=  '=' | '!=' | '<' | '<=' | '>' | '>='
```

**The admission rule, which is what keeps it finite:** *every operand names something the corpus
already declares.* `flag` and `counter` resolve to `world-flag` instances, `item` to an
`inventory-item`, `option` to a choice within a `conversation`. Integer literals are the single
free operand and are bounded by the counter they are compared against. There are no variables,
no binding forms, no user-defined names and no strings — there is nothing to name that the game
has not declared.

**And a name is scoped to the thing that contains it.** The rule was not part of this design
at first, and a specimen written without it put a `hotspot lamp` in one room and an `option
lamp` in a conversation. They collided — one flat space of names for rooms, items, flags,
hotspots and dialogue options alike — and it went unnoticed for two phases because the gate
then in use matched names against one flat set too.

The scoping rule is **read off the ontology rather than invented**, which is why it costs
nothing to state: a name belongs to whatever contains it, and containment is already declared.
A room owns its hotspots (`room -contains-> hotspot`); an exchange owns its options and its
conversation-scoped flags (`world-flag.scope` has allowed that since genesis); everything else
is the world's, because nothing contains it. An item cannot be room-scoped — it moves between
rooms, which is what `inventory-item` being a UFO *role* means — and a flag cannot be, because
`world-flag` is described as the only thing that carries meaning across rooms.

A bare reference means the local one, then the world's. A name carrying a `.` is explicit:
`chosen north.greet` reaches across from anywhere. Scoping widens the space of legal names and
does not remove the rule — two hotspots named `door` in one room is still a collision.

AGS numbers dialogue options *for the current topic*
([ags-manual](../.yidam/catalog/ags-manual.md)) [verified], so this is what the one system that
built a bounded conversation vocabulary did. See `.yidam/decisions/name-scoping.yml`.

Two properties follow directly, and they are the reason to want this rather than a VM:

- **Termination is structural, not proven.** An expression is a finite tree with no loops, no
  recursion and no calls. Evaluation is linear in its size.

  That is *evaluation* terminating. **Analysis terminating is a different property, and the
  grammar does not buy it** — a distinction this document did not draw until a specimen
  forced it. A game's state space is finite only if every variable in it is bounded, and an
  item bearing a quantity is bounded only if its supply is: an acquisition source that can
  fire twice hands out an unbounded number and the search never closes. Found by running it.
  Items therefore declare a ceiling exactly as counters declare a range, defaulting to one.
- **The set of predicates expressible about a given game is finite and enumerable from that
  game's own declarations.** This is the property section 4 is about.

Deliberately excluded, each with the reason, because a negative result about expressiveness is
worth more here than a permissive grammar:

- **Arithmetic on values** (`count rope + count hook >= 2`). No case in this corpus motivates
  it. Comparison against a literal covers the ordered sequence, which is the only motivating
  case there is — see
  [`world-flag/counter-flag.yml`](../.yidam/corpus/world-flag/counter-flag.yml).
- **`visited room`.** Every engine has something like it, and it is a boolean flag the engine
  could set implicitly. Admitting it means admitting `talked-to`, `seen`, `entered-carrying` and
  the rest, one convenience at a time. The line goes at *the author declared the flag*.
- **Quantifiers** (`any`, `all`) over an arbitrary collection. `count item` is the one
  quantifier here and it earns its place: an item bearing a quantity **is** the collection this
  exclusion said the vocabulary lacked. Without it, "any three of five" is a ten-term
  disjunction of three-term conjunctions and grows combinatorially; with it, `count coin >= 3`.
  The exclusion and `inventory-item/stackable-item` stand or fall together, and a second
  specimen written as a tally is what settled which.

  `world-flag`'s `domain` still lists `set` among its values with no instance using one; a
  quantifier over *that* remains excluded and unmotivated.
- **Anything that can fail to terminate.** No loops, no recursion, no user-defined functions.

### Layer 2 — effects, whose vocabulary is not designed here and whose semantics are

Write-only, unconditional, and **read off the ontology rather than invented.** An effect is a
declared relationship that changes world state, and the corpus declares these:

| Effect | Relationship | Declared by |
|---|---|---|
| write a flag | `sets` → `world-flag` | `interaction`, `conversation`, `cutscene` |
| enter an exchange | `opens` → `conversation` | `interaction` |
| return control | `returns-control-to` → `actor` | `cutscene` |
| acquire an item | `acquired-by` → `interaction`, read backwards | `inventory-item` |
| move the player | `occupies` → `room` | `actor` |
| spend an item | `consumes` → `inventory-item` | `interaction` |
| leave a mode | `returns-control-to` → `actor` | `cutscene`, `conversation` |

The last row was not in this table when it was written. It was added after
[the specimen](specimen/README.md) needed to move a player between rooms at four sites and found
the relationship declared since genesis — this table had read `occupies` as a static fact about
where an actor is rather than as something a binding writes. Five effects, not four, found by
needing one rather than by re-reading.

This is the part the gate checks, and the reason `design/check.sh` exists in the shape it does:
a design that names an effect the ontology does not declare is either wrong or has just found a
missing relationship. **Two turned up, and neither was being looked for.**

#### What a *list* of them means

The vocabulary is read off the ontology; what happens when a binding writes several is a choice
this document has to make, and for five phases it did not — while `crates/reachability` made it
anyway. It is settled in
[`effect-list-semantics`](../.yidam/decisions/effect-list-semantics.yml) and the four words are:

- **Sequential** — folded in written order. `set f` then `clear f` leaves `f` clear. Not a set of
  independent writes.
- **Total** — every effect applies. No early exit, no effect that can fail.
- **Instantaneous** — nothing takes time, nothing yields control. No blocking flag, no wait.
- **Atomic** — a binding is one edge in the state graph, and nothing observes a value between two
  effects.

`say` is presentation, not an effect. It is not in this table and never was.

**Duration is a mode in the state, not a pause in a transition.** The vocabulary already works
this way once: `opens → conversation` enters a mode and `returns-control-to → actor` leaves it,
both instantaneously, while the mode itself persists in the state vector and the checker sees it
as a state. Anything that takes time is written the same way.

**The alternative is real, it shipped, and it is refused.** AGS's declarative layer made blocking
a typed argument on the commands that take time (section 1). That is the better *authoring*
answer — it lets an author write "walk there, then speak" without leaving the language. It is
refused because it makes a binding an interval rather than an edge: give an effect duration and
other bindings can fire inside it, so the state space gains a dimension and the search is over
states-and-time. Every number in section 4 is a number about the graph this preserves.

**And it costs something this document should say plainly.** Nothing here can take time except a
mode, so a sequence that moves actors on its own schedule is not an effect list — it is a
`cutscene`. The corpus has that class and **no relationship in it targets `cutscene`**, so a
binding cannot start one. That gap is recorded rather than filled; the decision says why.

**Four gaps were reported here and one was real.** They were written down as gaps, then each
was weighed against what the ontology already declares — which is the rule this repository
states and which this document had not applied to itself. The account below is what survived;
the reasoning is in `.yidam/decisions/effect-vocabulary.yml` and the method finding, which is
the more useful one, is in [the specimen's findings](specimen/README.md).

**Real: nothing said an item is spent.** `inventory-item` declared `found-in`, `carried-form-of`
and `acquired-by`, all three about an object arriving or being. It does not reduce to a flag: a
flag standing in for consumption means every consumable item's availability is authored twice,
once structurally by the format and once by hand, with nothing holding them in step. Now
`interaction -consumes-> inventory-item`, with `interaction/consuming-binding` as its site. It
is the row in the table above that this document had to earn rather than read.

**Not a gap: an object leaving the room.** `inventory-item` is typed a UFO *role* and
`carried-form-of` names the hotspot the same object is when not carried, so acquiring it and the
room losing it are one change described twice. `acquisition-binding`'s atomicity worry survives,
relocated: the window belongs to an engine that keeps two records, not to a vocabulary with one
fact.

**Not a gap: option availability.** `world-flag.scope` has read "local to a conversation" since
genesis, and `conversation -sets-> world-flag` with it. One third of it did not reduce —
`option-off-forever` is permanent, and a clearable flag cannot express an unclearable one — and
that wanted a **property**, `world-flag.reversible`, rather than an edge.

**Not a gap: leaving a conversation.** `cutscene -returns-control-to-> actor` was declared when
that class was seeded and fits `conversation` without alteration. Two modes that seize control
and give it back: a structure this ontology had encoded once and not seen.

### Layer 3 — which turns out not to be a layer

*Pick among several lines* was one of the three cases the open question named as forcing the
escape. It needs nothing new: a guarded list is `[predicate → line]` evaluated first-match,
which is Layer 1 plus an ordering rule. That collapse is worth stating because the corpus's
class list is a standing claim about minimality and this is one fewer thing in it.

## 3. The three named cases, answered

The open question listed what a declarative table cannot do: *"count an inventory, compare two
flags, pick among several lines."*

| Case | Where it lands |
|---|---|
| count an inventory | `count item >= 3` — Layer 1 |
| compare two flags | `not rope-cut and bridge-down` — Layer 1 |
| pick among several lines | a guarded list — Layer 1 plus first-match |

All three are inside the predicate layer. The claim this supports is narrow and should not be
read wider: **the cases this corpus named do not force an escape.** It is not a claim that no
case does, and section 5 says what would settle that.

It also answers, conditionally, a question posed as if unconditional.
[`counter-flag.yml`](../.yidam/corpus/world-flag/counter-flag.yml) asks whether an integer
domain earns its place in the vocabulary at all. Without a predicate layer, a counter cannot be
*read* by anything an author writes, and the author is back to booleans and hand-written
ordering. With one, the ordered sequence is both expressible and checkable. The answer is *yes
given the middle, no without it* — and the open question did not have the middle as a variable.

## 4. Why build it: the argument neither engine had a reason to make

Not authoring convenience. **Analysis.**

[`unwinnable-state`](../.yidam/corpus/failure-mode/unwinnable-state.yml) is marked
`detectable_at: authoring`, and justifies that on the grounds that *"both causes are decidable
from the authored content alone — no play is required."* That is true only if the authored
content is itself decidable. The moment a precondition may be an arbitrary script expression,
reachability over world state is undecidable and the node's own justification fails.

The same holds for
[`orphaned-dialogue-branch`](../.yidam/corpus/failure-mode/orphaned-dialogue-branch.yml).
Whether a subtree is still reachable is a question about the option-availability graph, and it
is answerable exactly when option state is written by a closed effect list under decidable
guards. AGS took the escape hatch instead, and that node is the price: its manual documents the
mechanism — `option-off-forever` is permanent, "no other command can ever turn it back on
again" [verified] — and nothing in the authoring surface detects the consequence.

So the case for the middle is not that authors want it. It is:

> **A bounded predicate language is what makes the checker possible. A general-purpose escape
> hatch is what makes it impossible.** The cost of the escape is not paid at the moment it is
> used; it is paid by every check that can no longer be run over the whole game.

Neither engine had a reason to make that argument. SCUMM had no declarative layer to analyse.
AGS's analysis ended at the first indented line, and the indentation is invisible.

**The specimen showed this argument was necessary and not sufficient, and then it was made
sufficient.** [`lighthouse.game`](specimen/lighthouse.game) carries one deliberate reachable
unwinnable state whose mechanism is consumption — the oil is spent on a hinge and the lamp can
never be filled. Consumption was the one real gap, so the game contained exactly the defect a
checker exists to find, the language could not express the mechanism producing it, and a checker
would have reported it winnable.

`consumes` closes that. The point that survives is the general one, and it is not about escape
hatches: **a bounded language buys a checker only to the extent that the effect vocabulary can
describe how a game breaks.** Grammar was never the binding constraint. Four of the specimen's
nine unsayable sites were this single defect, and the other five were this document's author
failing to read an ontology that was already in front of them.

**The argument is no longer an argument.** `crates/reachability` enumerates the specimen's
state space — 103 reachable states, 193 transitions — finds the ending reachable, finds 16
states from which it is not, and names the single binding that strands them: `use-with oil-can
on hinge`, classified as consumption, with an example of what was lost. That is the trap, found
mechanically, and it is the first claim in this document that was executed rather than
asserted.

What that demonstrates is narrower than it looks and worth stating at its true size. It shows
the computation is *possible* on a bounded language, which was the whole claim. It does not
show the language is adequate for a real game, that the state space stays enumerable at scale,
or that anybody would enjoy writing in it.

## 5. What this does not settle

- **Concrete syntax.** Everything above is abstract. The surface belongs with
  [`interaction-table.yml`](../.yidam/corpus/data-format/interaction-table.yml) and its
  unresolved tension between being keyed by verb and target and reading like writing.
- **Whether the effect list stays closed.** The whole design rests on it. The first effect that
  must be conditional — *set this flag only if* — either becomes a guarded effect, which is
  Layer 1 applied to Layer 2 and costs nothing, or forces the door open. Nothing here has met
  that case yet.
- **Whether an escape hatch is needed anyway.** This design proposes none, which is either the
  point of it or a fatal omission. The one group who ran the experiment concluded an escape was
  needed and wrote so in their own manual. That is a single data point, it is against, and it
  comes from a system that already had somewhere to escape to. **It is now two data points from
  the same system**: the second is a declarative interaction layer at this document's own scope,
  removed in favour of script, for reasons its manual does not give. See section 1.
- ~~Whether effects are sequential, and whether any of them takes time.~~ **Settled.** Sequential,
  total, instantaneous, atomic — see Layer 2 and
  [`effect-list-semantics`](../.yidam/decisions/effect-list-semantics.yml). AGS's per-command
  blocking flag is the alternative, it is the better authoring answer, and it is refused because
  it makes a binding an interval rather than an edge.

  What the decision opened in its place is smaller and sharper: **nothing in this vocabulary can
  take time except a mode, and the class for things that take time cannot be started.** No
  relationship in the corpus targets `cutscene`. `interaction -enters-> cutscene` is the obvious
  mirror of `opens → conversation` and is deliberately unwritten until a game needs one, because a
  declared relationship no instance authors is a defect this corpus already carries two of. `[open]`

  A `say` that blocks and a `move` that does not are already two different kinds of thing, and the
  specimens write them in one list.
- **What it costs an author.** Every claim above is about what is checkable, and the argument
  that a bounded language is analysable says nothing about whether it is bearable. Three rooms
  have now been written in it, by the same party that proposed it, which is the weakest possible
  authoring trial and is not evidence of pleasantness.
- ~~Whether `count item` earns its place.~~ **Settled.** A second specimen written as a tally
  reduced first to distinct items, which costs a declaration and a conjunction term per unit
  and cannot express "any k of n" at all. See
  [`stackable-item`](../.yidam/corpus/inventory-item/stackable-item.yml). Still unexercised
  across both specimens: `or`, `!=`, `<=`, `>` and grouping.
- ~~Whether the state space stays enumerable at scale.~~ **Answered, and the answer is that it
  depends on the game and this language does nothing to make it so.**

  Two generated families settle it. A **corridor** — every door needing the key behind the last
  one — has 3n+1 reachable states and analyses 128 rooms in sixteen milliseconds. A **hub** — *n*
  tokens in one room, no order between them, all wanted at the gate — has 2^n reachable states
  and **100.000% of the declared space at every size**, reaching 4.2 million states and
  thirty-six seconds at twenty-two tokens. `crates/reachability/tests/scaling.rs` generates both.

  Both families are made entirely of **monotone** variables, which is the part that matters:
  nothing is ever unset in either. So monotonicity is not what makes the search cheap — it bounds
  the path and not the size of the set, and a lattice over *n* independent monotone variables is
  every combination. What makes the search cheap is **dependency**, and a hub is an ordinary
  adventure-game shape: find the three map pieces, collect the four keys.

  The numbers this document used to quote were also too small by 2^(takeable objects), because
  the metric counted the checker's own bookkeeping as world state. Corrected: 103 of 49,152,
  20 of 120, 14 of 64. That old metric reported ~0% for *both* generated families, so it could
  not have found any of this.

  **What survives.** Decidability is not in question — the space is finite and the search
  terminates, which is what Layer 1 buys and no general-purpose escape hatch can offer.
  Tractability is a different claim and this document had been making it on three small games.

  **And the narrower question it became has an answer.** *How ordered are a real game's puzzles?*
  — asked of Ron Gilbert, who invented the notation for arranging them. His rule: "Solving one
  puzzle should open up 2 or 3 new ones", each act ending in "a bottle neck to the next act",
  with **both** extremes named as failures — too linear, or "too unwieldy with choice (also bad)"
  ([grumpy-gamer-pdc](../.yidam/catalog/grumpy-gamer-pdc.md)). [verified]

  A third generated family measures what that shape costs. Forty-eight puzzles arranged in acts
  of two are 97 reachable states; of three, 129; of eight, 1,537; of sixteen, 196,609. Same
  count, a factor of two thousand, decided by width alone. At width three the count scales
  linearly — six hundred puzzles is 1,601 states and 73 milliseconds. **The reachable set is
  linear in the number of puzzles and exponential in the width of the dependency chart.**
  [verified]

  So the tractability claim stands again, on a footing this document did not have and should
  state plainly: **not as a property of this language, but as a property of games designed the
  way the form's inventor designs them.** A bounded predicate layer makes the question decidable;
  a bounded chart width makes it cheap; and only the first is anything this design controls.
  [inference]

  Still missing is a **measured** width from a shipped game. Gilbert prescribes 2–3 and reports
  4–5 for unfinished boxes in Thimbleweed Park, and does not have Monkey Island's chart. `[open]`
- **Nothing here is implemented as an engine.** No engine reads this. It is exercised by one
  specimen and analysed by `crates/reachability`, which is a checker rather than a runtime —
  the analysis says a game *can* be finished, and nothing here has ever played one.
