# decisions

A record is a claim about what this project chose and why. A [corpus
node](../corpus/README.md) is a claim about point-and-click adventure games. They are
different kinds, which is why `yidam status` counts 42 nodes and not 84, and why this
directory has its own gate instead of moving: `crates/design-gate/tests/records.rs`.

Everything below the line is generated. `REGEN=1 cargo test -p design-gate --test records` rewrites it, and `cargo test` fails when it is stale.

---

## Open questions

Every live `[open]` in this directory, which no yidam command can enumerate: they are
counted by `status` only when they sit in a corpus node. The convention is
[marker-count](marker-count.yml)'s — the token names a live question and a retraction
does not re-emit it.

### [action-pages](action-pages.yml)

- **:136** — The corpus has a de facto home for these already — [ontology](ontology.yml) holds both live class collapses and has since genesis — and what it does not have is any way to count what is in it. That is [marker-count](marker-count.yml)'s threshold again, reached from a third side: a typed `claim_tag` …

### [actor-position](actor-position.yml)

- **:112** — **What is not done.** `nightwatch.game` still says its watchman is down the hall in prose. Rewriting it would make that specimen test what it was written to say it could not, and would move figures that three phases of records quote. Whether a specimen should be revised to exercise a primitive it ar…

### [adjacency](adjacency.yml)

- **:101** — **What the geometry cannot say is intent, and that is a different question.** Six of the fifteen rooms are in more than one piece with every region present, across gaps of 3 to 16 pixels in rooms 320 to 600 wide, and **nothing distinguishes a gap that means *you may not go there* from one left by a …

### [arrival-realizer](arrival-realizer.yml)

- **:110** — **What this does not settle.** The answer is conditional on two refusals holding, and this is the first place they combine. A design that stored presentation, or that stored affordance instead of computing it, gets the fourth component back — and nothing in this corpus records that choosing to store…

### [availability](availability.yml)

- **:107** — **What this does not settle.** Whether a binding should be able to carry two gates — one withholding, one refusing — as that game's verb slots do. Nothing here has wanted it, and the evidence for it is a single object in a bathroom.

### [binary-dimension](binary-dimension.yml)

- **:79** — **What stays open.** The reduction being free to the analysis is a fact about *this* checker, and this checker treats inventory as a count per item with a declared ceiling. A checker that modelled the item's identity — which of two crowbars — might not be indifferent, and nothing here has one.
- **:84** — And the thing the measurement did not touch: **a binary verb makes the table two-dimensional**, which is `binary-binding`'s combinatorics argument, and n×m cells is a cost to an *author* rather than to a checker. Every measurement in this record is a checker's. The authoring cost of arity is the sam…

### [binding-axes](binding-axes.yml)

- **:87** — **What this does not settle.** Whether the three remaining role-named instances declare a shape they have or a shape they had in mind. `acquisition-binding` says `unary` and the corpus's own `is` sites hand an item over from a hotspot, which is how a *binary* use-with can acquire; `default-binding` …

### [carried-claims](carried-claims.yml)

- **:81** — **What this does not settle.** Whether a node should say when its evidence base is one line. The three that do wrote it as a finding about themselves and it reads well; twelve nodes writing the same paragraph would read as boilerplate, and boilerplate is what a check exists to replace. The check nam…

### [class-evidence](class-evidence.yml)

- **:86** — **What this does not settle.** Whether class prose should state propositions at all. The prelude stops short of forbidding it and this record follows, because the six statements in question are load-bearing: `interaction.ont.yml`'s one-property-not-two decision is unintelligible without the sentence…

### [class-library](class-library.yml)

- **:126** — **What this leaves: a third state the catalog has no spelling for.** An unread-list item is on the list or it came off and the list says where the reading went. *Retrieved, opened, and unreadable here* is neither, and it matters because the next reader would otherwise repeat the attempt. The entry s…

### [collected-verdict](collected-verdict.yml)

- **:89** — **What this does not settle.** Whether a record should carry figures at all. The convention says keep them out of the summary and this record moves two out of one; what neither says is where they should go instead, and the answer here was available only because `surface.rs` existed to point at. Reco…

### [control-transfer](control-transfer.yml)

- **:134** — **What this does not decide: whether this vocabulary should pay for it.** Every cost above is measured and none of them is weighed. A minimal authoring vocabulary is allowed to refuse a feature the form has — this one refuses arithmetic, quantifiers and delegation on exactly that ground — and four o…

### [conversation-reduction](conversation-reduction.yml)

- **:81** — **What this does not settle.** Whether an exchange should be enterable from more than one place at all — no specimen has one, and the two-entrance case above is constructed. If exchanges are always entered from one room, the return is free and the class's last ground is a convenience rather than a n…

### [declared-reach](declared-reach.yml)

- **:93** — **What this does not settle.** What the channel costs an author. This entry counted declaration sites and did not read `Note.dinky`'s logic or `takephoto`'s body, which is the longest binding in the game — so the claim that 52 declarations expand to 52 bindings is arithmetic over the declarations ra…

### [delegation](delegation.yml)

- **:100** — Not decided here, because one game is one game and because the four rows want three different relations — part-of, same-as, and says-the-same — and a design that collapsed them would be choosing by convenience.

### [duplication](duplication.yml)

- **:113** — It settles one shape and not the pressure. `definePseudoRoom` built 110 hotel rooms, which is a *room* template; an archetype per shape does not reach shapes nobody anticipated, and that was the argument against archetypes to begin with.
- **:119** — **What this does not touch.** Whether the tractability figures should be re-earned on a game that can be written in this language. They are correct measurements of the games measured, and the games are honest instances of the shapes they claim to be; what they cannot be is evidence that an *author* …

### [evidence-kinds](evidence-kinds.yml)

- **:105** — **What this does not settle.** The 150 that name nothing. They are continuations — a node establishes a source once and argues from it for several paragraphs, which is how anyone writes and is the reason the census is a lower bound rather than a measurement. But it means a claim's footing is a fact …

### [exit-archetype](exit-archetype.yml)

- **:79** — **What this does not settle.** It is one archetype, for one shape, chosen because two bodies of work exhibit it. `definePseudoRoom` built 110 hotel rooms and that is a **room** template; nothing here generalises to it, and an archetype per shape does not scale to shapes nobody anticipated — which wa…

### [forty-eight](forty-eight.yml)

- **:106** — **What this does not settle.** Whether presentation belongs in this vocabulary at all. Twenty- three actions of a shipped authoring surface do nothing but play, show, animate and wait, and this design has no way to write any of them — which [effect-half](effect-half.yml) treated as the boundary bein…

### [found-in](found-in.yml)

- **:131** — **There are two of them now.** `interaction/binary-binding` arrived in the same position from [binary-dimension](binary-dimension.yml) — claims measured on committed tests, no catalog entry to cite. One instance was a curiosity about a lint; two is a gap in the evidence vocabulary, and this corpus's…
- **:137** — Whether the check should also refuse a give of some *other* item on a hotspot that stands for one is untested. All eight `is` sites in the four specimens give exactly what they stand for and nothing else [verified], so the corpus has no case where a hotspot legitimately hands over something besides …

### [frame-width](frame-width.yml)

- **:95** — **What this does not settle.** The identity half is untouched and stands where [hotspot-identity](hotspot-identity.yml) left it — evidenced and named as `parent`, and unadopted. The reason has moved since: `.game` still has no geometry and [drawn-boundary](drawn-boundary.yml) made that permanent, bu…
- **:105** — **One correction fell out of the measurement and is larger than the finding.** Counting frame against prose across the shipped game's verb slots meant counting `VERB_OK`, and this corpus had it at zero authored sites in three places. There are 62. The search was for `okverb_`, the name the macro exp…

### [hotspot-identity](hotspot-identity.yml)

- **:77** — **What this does not decide.** Whether a design that did include geometry should have one relation or three. That design exists now, as a scene layer beside `.game`, and it has none of the three: every target is one rectangle and no two overlap. [verified] — `engine/play/tests/screen.rs`, where a sc…

### [inert-reports](inert-reports.yml)

- **:109** — **What this does not settle.** Whether the corpus should have an index at all. Three reports nagging about its absence is not evidence that it is wanted — `yidam query` answers structural questions and `pack` answers retrieval ones, both without embeddings, and no phase in this repository's life has…

### [inward-turn](inward-turn.yml)

- **:107** — **What this does not settle.** Whether a record resting on the gate alone is the same *kind* as one resting on a document. The directory holds one kind and `records.rs` gates them identically, but `record-layer` drew a line between a claim about adventure games and a claim about this project on exac…

### [line-format](line-format.yml)

- **:101** — **What this does not do: derive the reader from the table.** The forms are *held against* the match arms, not generating them — a construct's arity is checked by a row and its meaning is still an arm, which is the same boundary `parser-silence` drew one level in. Deriving a reader from a table means…

### [marker-count](marker-count.yml)

- **:91** — **What this does not settle.** Whether a marker should carry the phase that opened it. Every one of the five stale markers could have been caught by re-reading a record from the phase that followed, and nothing connects the two — a marker names no date, no phase and no record, so "has this been answ…

### [measured-width](measured-width.yml)

- **:88** — **The `[open]` that remains is narrower and no longer about retrieval.** Not *is there a measured width* — there is one now — but whether a whole-game antichain can stand in for the width of an act when the game has no acts, and what a second, larger game would say. Thimbleweed Park's source is not …

### [moved-ground](moved-ground.yml)

- **:84** — **What this does not settle.** Whether `design/` should hold a proposal this long at all. Seven hundred lines resting on fourteen premises is a document that can only be reconciled in one sitting by somebody who has the whole corpus in mind, and the reconciliation above took a phase. A shorter propo…

### [orphaned-lines](orphaned-lines.yml)

- **:99** — **Not proposed here, and this is the third time the same shape has been declined.** A `once` modifier is a 51st construct, and `latch-default` already weighed declaring a property whose truth the checker can compute — *"a declaration that is true and absent costs nothing to make and buys the guarant…

### [parser-silence](parser-silence.yml)

- **:91** — **What this does not do: give the format a grammar.** Sixteen branches each stating their own arity is better than sixteen branches stating nothing, and it is not a specification. The boundary this phase draws is that a construct must account for its line; what a construct *is* remains a match arm. …

### [priority-edge](priority-edge.yml)

- **:76** — **What this does not settle.** Whether the comparative tier wants a relationship of its own. The argument for one is in this record: `originated-in` cannot reach a 2017 engine, and the tier's whole purpose is comparison across time. The argument against is that nothing in the corpus has needed the e…

### [prose-edges](prose-edges.yml)

- **:131** — **What this does not settle.** Whether the second graph should have a form. Every citation in it resolves, so the four spellings and three bases cost nothing today; what they cost is every tool that was never written, and this phase wrote one of them and found the count had drifted in a document nob…

### [record-layer](record-layer.yml)

- **:123** — **What this does not settle.** Whether the migration should be reversed rather than gated. Every phase for six phases has answered a corpus question in a record, and the corpus node it answers now often carries a pointer where it used to carry the argument — which is legible to a reader following li…

### [record-lifecycle](record-lifecycle.yml)

- **:77** — **What this does not settle.** Whether the four post-creation `decide:` commits should have been new records. Each is a later phase's decision editing a record it did not write — `actor-position`, `availability`, `default-binding`, `observed-state` — and the prelude's remedy for exactly that case is…

### [resolver-kind](resolver-kind.yml)

- **:94** — **And it does not retire the three.** A question nothing in reach can answer is still a live question about adventure games, and `testimony` is a standing account of why rather than a retirement. [calculators](calculators.yml) retired two skills because *the thing that wanted them went away*; nothin…

### [retraction-reach](retraction-reach.yml)

- **:19** — > *Script Classes for Adventure Games* is in the retrieved set, **unread**, and is where a verb > bar would be declared if Sierra declared one.
- **:81** — **What this does not settle.** Whether the corpus or the record is the authority anywhere other than a marker. Nothing checks that a node and the record that argued about it agree on anything else, and the two copies have been drifting in whichever direction each phase was looking. What would close …

### [schema-deferral](schema-deferral.yml)

- **:90** — **What this does not settle.** Whether the corpus should count deferrals at all. This record found four instances of one question by matching a phrase, which worked because the four happened to reach for the same words — and `marker-count`'s question, the one still refused, is exactly the machinery …

### [script-boundary](script-boundary.yml)

- **:96** — **What this does not settle.** What authors actually do with the arithmetic. The cut is clean and the *cost* of the cut is unmeasured: `Globalfunctions_General`, the `Maths` page and the fifty-odd other API pages are still unread, and the question *how often does a shipped AGS game reach the hatch f…

### [second-document](second-document.yml)

- **:91** — **What this does not settle.** Whether a specimen should be revised to exercise a primitive it argued for. The document's own question, and it now has two instances rather than one — the watchman who moves in prose, and `exit`, which this document's argument built and none of the four games writes. …

### [settled-surface](settled-surface.yml)

- **:95** — **What this does not settle.** Whether an unsettled list should be a list at all. Three items here are struck through and kept, which makes the section a record of what was asked as well as what is open — good for a reader who wants the history and bad for one who wants the current state, since eigh…

### [testimony](testimony.yml)

- **:68** — **What this does not settle.** Whether testimony and artifact should carry different evidence standing. The prelude is explicit that `[verified]` is about provenance rather than confidence, and a diary entry is a committed source in exactly the way a header file is; `measured-width` quotes Gilbert's…

### [third-population](third-population.yml)

- **:85** — **What this does not settle.** Whether `presentation-layer`'s three-verb width holds against this table. It cannot be read off directly: that record measured what a *binding* writes and found camera and walkbox appear in bindings zero times, belonging to room entry instead — and SCUMM's opcode list …

### [threshold-trigger](threshold-trigger.yml)

- **:107** — The question itself stays open and is narrower than it was. It is answerable only by a design that admitted duration, which is not this one; what has changed is that the trade can now be examined rather than only asserted, and fifteen is the number of arguments that would have to be re-read.

### [two-bars](two-bars.yml)

- **:95** — **What the record leaves open is a different thing: what a declaration buys, measured.** Sierra's menu is declared and its icon bar is not, in one engine, by one set of people, at the same time. That is a natural experiment nothing here has run — the obvious candidate is *enumerability*, and whether…

### [two-components](two-components.yml)

- **:131** — **And whether one crate per subsystem survives a component with parts.** Two crates of one file each is not evidence that the mapping scales, and the first component that wants three modules will make every one of them declare the same `realizes:` line. That is either the rule working or ceremony, a…

### [unread-clock](unread-clock.yml)

- **:128** — **What this does not settle.** Whether a question's age should be a fact the node carries. Blame is a lower bound and resets on a reword, and the only reason it reads correctly today is that nine phases revised `player-character` around line 14 without touching it — luck rather than design. A `since…

### [unread-list](unread-list.yml)

- **:96** — **What this does not settle.** Whether the two live items under the spent one are worth taking. `hints_and_dialogs.html`, `text_lock.html` and `translations.html` cover how the dialogue and text were authored, which the diary retrieval explicitly did not ask about and which [conversation](../corpus/…

### [unwritten](unwritten.yml)

- **:96** — **What this does not settle.** Whether the two withdrawn constructs were withdrawn for this reason or in spite of it. `escape-hatch` records two withdrawals and the coverage of the surface at the time is not recoverable — `coverage.rs` reads the specimens as they stand, and a construct's history of …

### [verb-menu](verb-menu.yml)

- **:97** — **What this does not settle.** Whether the derivation is the *right* interface, which is a question about players and has no instrument here. Every figure above says what this design's menu *is*; none of them says a player could use it, and `.yidam/decisions/default-binding.yml`'s observation that t…

### [verb-names](verb-names.yml)

- **:77** — **What the menu now reads, and why it reads slightly oddly.** *"Use a short crowbar with a nailed-shut crate"* — because an author writes hotspot labels as noun phrases with articles, and a template that assumed otherwise would be telling authors how to write. Delores's authors face the same seam an…

### [walked-in](walked-in.yml)

- **:97** — Whether a runtime should be allowed to ask is a real question and is not answered here. A game that told the player *"you cannot win from here"* would be a different genre; one that quietly declined to let them reach the state would be a different design; one that shipped with the answer computed at…
- **:111** — **And the second defect is the one worth carrying forward.** *"Selected in the state it fired from"* is a rule about ordering, and this vocabulary has exactly one construct where ordering inside a single transition is observable — a chain of forced bindings. [effect-list-semantics](effect-list-seman…

**57 open.**

## What nothing cites

Records no other record points at. Not a defect on its own — a record may be the last
word on something nothing else has needed — but it is the shape `orphan-in` reports for
nodes, at four times the rate, and it had never been looked at.

- [binding-axes](binding-axes.yml)
- [declared-reach](declared-reach.yml)
- [domain-computer](domain-computer.yml)
- [name-scoping](name-scoping.yml)
- [settled-surface](settled-surface.yml)

## What each record rests on

A record cites a catalog entry, the engine crates, or the gate that reads this corpus,
and which it cites says what kind of question it answered. All three are legitimate.
The classification was derivable from the day this directory existed and nothing showed
it, which is [inward-turn](inward-turn.yml)'s finding; the run at the end of the first
list is the one that record holds as a tripwire.

**On the gate and nothing else** — a question about this repository.

- [action-pages](action-pages.yml)
- [carried-claims](carried-claims.yml)
- [class-evidence](class-evidence.yml)
- [declared-edge](declared-edge.yml)
- [drawn-boundary](drawn-boundary.yml)
- [engine-directory](engine-directory.yml)
- [inert-reports](inert-reports.yml)
- [inward-turn](inward-turn.yml)
- [marker-count](marker-count.yml)
- [priority-edge](priority-edge.yml)
- [prose-edges](prose-edges.yml)
- [record-layer](record-layer.yml)
- [record-lifecycle](record-lifecycle.yml)
- [resolver-kind](resolver-kind.yml)
- [retraction-reach](retraction-reach.yml)
- [schema-deferral](schema-deferral.yml)
- [unread-clock](unread-clock.yml)
- [unread-spelling](unread-spelling.yml)

**On no evidence path named here** — structural, or resting on the prelude.

- [arrival-trigger](arrival-trigger.yml)
- [conversation-boundary](conversation-boundary.yml)
- [design-directory](design-directory.yml)
- [domain-computer](domain-computer.yml)
- [effect-vocabulary](effect-vocabulary.yml)
- [name-scoping](name-scoping.yml)
- [ontology](ontology.yml)
- [proposals](proposals.yml)
- [runtime-boundaries](runtime-boundaries.yml)
- [seed-scope](seed-scope.yml)
- [specimen-placement](specimen-placement.yml)
- [stale-markers](stale-markers.yml)
