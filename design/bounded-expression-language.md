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

This is a proposal for that middle. It is a proposal and not a finding: nothing here is
implemented, no engine reads it, and it is filed in `design/` rather than in the corpus for
exactly that reason.

## 1. What AGS's experiment actually shows

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

That objection does not rescue the corpus's original reading — it replaces it. AGS is not
evidence that a bounded expression language is insufficient. **It is a system that had no reason
to try one.** The gap stands and its shape is different: the middle is untested rather than
tested and failed.

SCUMM is the other half and says less. Its v5 opcode table has `print` and `printEgo` and
nothing that manages a menu of options
([scumm-v5-opcode-reference](../.yidam/catalog/scumm-v5-opcode-reference.md)) [inference] — there
was no declarative layer, so there was nothing to bound.

## 2. The proposal

Three parts, and the split between the first two is the whole design.

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

Two properties follow directly, and they are the reason to want this rather than a VM:

- **Termination is structural, not proven.** An expression is a finite tree with no loops, no
  recursion and no calls. Evaluation is linear in its size.
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
- **Quantifiers** (`any`, `all`). They need a collection and the vocabulary has no collection
  type. `world-flag`'s `domain` property lists `set` among its values and no instance uses one;
  that absence is the answer for now.
- **Anything that can fail to terminate.** No loops, no recursion, no user-defined functions.

### Layer 2 — effects, which are not designed here

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
  comes from a system that already had somewhere to escape to.
- **What it costs an author.** Every claim above is about what is checkable, and the argument
  that a bounded language is analysable says nothing about whether it is bearable. Three rooms
  have now been written in it, by the same party that proposed it, which is the weakest possible
  authoring trial and is not evidence of pleasantness.
- **Whether `count item` earns its place.** The specimen never reached for it, along with `or`,
  `!=`, `<=`, `>` and grouping. One small game is thin evidence for removing anything.
- **Nothing here is implemented.** No engine reads this. It is now exercised by one specimen,
  which is a different thing from being tested by a running program.
