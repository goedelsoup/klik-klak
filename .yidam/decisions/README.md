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

- **:135** — **What this does not settle.** Whether a class-level question should have a tagged home of its own. The two live ones are parked on instances because that is where a marker can go, and both are really about the class: walkbox adjacency is a question about the relationship vocabulary, and *"is that a…

### [actor-position](actor-position.yml)

- **:112** — **What is not done.** `nightwatch.game` still says its watchman is down the hall in prose. Rewriting it would make that specimen test what it was written to say it could not, and would move figures that three phases of records quote. Whether a specimen should be revised to exercise a primitive it ar…

### [arity-cost](arity-cost.yml)

- **:96** — And the other thing the measurement did not touch: `ambiguous_cells` proves disjointness rather than overlap, so `or` is reported as inseparable. Nothing in the corpus writes one in a colliding cell, so the conservative branch has never been exercised on real authored work.

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

### [collected-verdict](collected-verdict.yml)

- **:89** — **What this does not settle.** Whether a record should carry figures at all. The convention says keep them out of the summary and this record moves two out of one; what neither says is where they should go instead, and the answer here was available only because `surface.rs` existed to point at. Reco…

### [conversation-reduction](conversation-reduction.yml)

- **:81** — **What this does not settle.** Whether an exchange should be enterable from more than one place at all — no specimen has one, and the two-entrance case above is constructed. If exchanges are always entered from one room, the return is free and the class's last ground is a convenience rather than a n…

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

- **:93** — **What this does not settle.** The identity half is untouched and stands where [hotspot-identity](hotspot-identity.yml) left it — evidenced and named as `parent`, unadopted because this vocabulary has no geometry. And the tractability figures are still measured on games no author would produce, whic…
- **:103** — **One correction fell out of the measurement and is larger than the finding.** Counting frame against prose across the shipped game's verb slots meant counting `VERB_OK`, and this corpus had it at zero authored sites in three places. There are 62. The search was for `okverb_`, the name the macro exp…

### [hotspot-identity](hotspot-identity.yml)

- **:75** — **What this does not decide.** Whether a design that did include geometry should have one relation or three. Nothing here bears on it, and the three are genuinely different — a part is contained by its whole, two regions of one thing are interchangeable, and two things that read alike are not interc…

### [inert-reports](inert-reports.yml)

- **:109** — **What this does not settle.** Whether the corpus should have an index at all. Three reports nagging about its absence is not evidence that it is wanted — `yidam query` answers structural questions and `pack` answers retrieval ones, both without embeddings, and no phase in this repository's life has…

### [inward-turn](inward-turn.yml)

- **:107** — **What this does not settle.** Whether a record resting on the gate alone is the same *kind* as one resting on a document. The directory holds one kind and `records.rs` gates them identically, but `record-layer` drew a line between a claim about adventure games and a claim about this project on exac…

### [marker-count](marker-count.yml)

- **:79** — **What this does not settle.** Whether a marker should carry the phase that opened it. Every one of the five stale markers could have been caught by re-reading a record from the phase that followed, and nothing connects the two — a marker names no date, no phase and no record, so "has this been answ…

### [measured-width](measured-width.yml)

- **:88** — **The `[open]` that remains is narrower and no longer about retrieval.** Not *is there a measured width* — there is one now — but whether a whole-game antichain can stand in for the width of an act when the game has no acts, and what a second, larger game would say. Thimbleweed Park's source is not …

### [priority-edge](priority-edge.yml)

- **:76** — **What this does not settle.** Whether the comparative tier wants a relationship of its own. The argument for one is in this record: `originated-in` cannot reach a 2017 engine, and the tier's whole purpose is comparison across time. The argument against is that nothing in the corpus has needed the e…

### [prose-edges](prose-edges.yml)

- **:131** — **What this does not settle.** Whether the second graph should have a form. Every citation in it resolves, so the four spellings and three bases cost nothing today; what they cost is every tool that was never written, and this phase wrote one of them and found the count had drifted in a document nob…

### [record-layer](record-layer.yml)

- **:123** — **What this does not settle.** Whether the migration should be reversed rather than gated. Every phase for six phases has answered a corpus question in a record, and the corpus node it answers now often carries a pointer where it used to carry the argument — which is legible to a reader following li…

### [record-lifecycle](record-lifecycle.yml)

- **:77** — **What this does not settle.** Whether the four post-creation `decide:` commits should have been new records. Each is a later phase's decision editing a record it did not write — `actor-position`, `availability`, `default-binding`, `observed-state` — and the prelude's remedy for exactly that case is…

### [save-boundary](save-boundary.yml)

- **:102** — **What this does not settle.** Whether a save should hold anything a predicate cannot read. There is one candidate and it is not hypothetical: a player's position *within* a room, which [path-solver](../corpus/subsystem/path-solver.yml) owns and no predicate in this vocabulary can ask about. The sta…

### [schema-deferral](schema-deferral.yml)

- **:90** — **What this does not settle.** Whether the corpus should count deferrals at all. This record found four instances of one question by matching a phrase, which worked because the four happened to reach for the same words — and `marker-count`'s question, the one still refused, is exactly the machinery …

### [script-boundary](script-boundary.yml)

- **:96** — **What this does not settle.** What authors actually do with the arithmetic. The cut is clean and the *cost* of the cut is unmeasured: `Globalfunctions_General`, the `Maths` page and the fifty-odd other API pages are still unread, and the question *how often does a shipped AGS game reach the hatch f…

### [testimony](testimony.yml)

- **:68** — **What this does not settle.** Whether testimony and artifact should carry different evidence standing. The prelude is explicit that `[verified]` is about provenance rather than confidence, and a diary entry is a committed source in exactly the way a header file is; `measured-width` quotes Gilbert's…

### [third-population](third-population.yml)

- **:85** — **What this does not settle.** Whether `presentation-layer`'s three-verb width holds against this table. It cannot be read off directly: that record measured what a *binding* writes and found camera and walkbox appear in bindings zero times, belonging to room entry instead — and SCUMM's opcode list …

### [threshold-trigger](threshold-trigger.yml)

- **:89** — **What this does not settle.** Whether duration should have been refused. Everything above is downstream of `effect-list-semantics`, and a design that admitted duration would need this trigger, a `skippable` property, and the save-boundary problem the same decision record quotes Thimbleweed Park pay…

### [unread-clock](unread-clock.yml)

- **:128** — **What this does not settle.** Whether a question's age should be a fact the node carries. Blame is a lower bound and resets on a reword, and the only reason it reads correctly today is that nine phases revised `player-character` around line 14 without touching it — luck rather than design. A `since…

### [unread-state](unread-state.yml)

- **:78** — **What this does not settle.** The rule is now applied by hand in five places and nothing checks that a sixth obeys it. That is the original question returning one level up: `observed-state` asked for a check that fails when an unreadable component is added, and what exists is five components each b…

### [unwritten](unwritten.yml)

- **:96** — **What this does not settle.** Whether the two withdrawn constructs were withdrawn for this reason or in spite of it. `escape-hatch` records two withdrawals and the coverage of the surface at the time is not recoverable — `coverage.rs` reads the specimens as they stand, and a construct's history of …

### [verb-menu](verb-menu.yml)

- **:97** — **What this does not settle.** Whether the derivation is the *right* interface, which is a question about players and has no instrument here. Every figure above says what this design's menu *is*; none of them says a player could use it, and `.yidam/decisions/default-binding.yml`'s observation that t…

**41 open.**

## What nothing cites

Records no other record points at. Not a defect on its own — a record may be the last
word on something nothing else has needed — but it is the shape `orphan-in` reports for
nodes, at four times the rate, and it had never been looked at.

- [binding-axes](binding-axes.yml)
- [class-evidence](class-evidence.yml)
- [design-directory](design-directory.yml)
- [domain-computer](domain-computer.yml)
- [name-scoping](name-scoping.yml)
- [specimen-placement](specimen-placement.yml)

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
- [drawn-boundary](drawn-boundary.yml)
- [inert-reports](inert-reports.yml)
- [inward-turn](inward-turn.yml)
- [marker-count](marker-count.yml)
- [priority-edge](priority-edge.yml)
- [prose-edges](prose-edges.yml)
- [record-layer](record-layer.yml)
- [record-lifecycle](record-lifecycle.yml)
- [schema-deferral](schema-deferral.yml)
- [unread-clock](unread-clock.yml)

**On no evidence path named here** — structural, or resting on the prelude.

- [arrival-trigger](arrival-trigger.yml)
- [conversation-boundary](conversation-boundary.yml)
- [design-directory](design-directory.yml)
- [domain-computer](domain-computer.yml)
- [effect-vocabulary](effect-vocabulary.yml)
- [name-scoping](name-scoping.yml)
- [ontology](ontology.yml)
- [proposals](proposals.yml)
- [seed-scope](seed-scope.yml)
- [specimen-placement](specimen-placement.yml)
- [stale-markers](stale-markers.yml)
