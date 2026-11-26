# Working in this repository

This is a knowledge artifact about the authoring vocabulary of point-and-click adventure
games, maintained alongside the development of an engine for them in Rust.

This is a yidam-derived repository: a living knowledge artifact whose git history is the
knowledge graph. It has already been bootstrapped — there is no bootstrap mode here, and
the prelude's old `bootstrap` skill is history, not an instruction (upstream no longer vendors it).

Read [AGENTS.md](../AGENTS.md) before taking substantive action. It names the prelude files
that govern conduct here and the gate that CI runs.

## The short version

- **Nodes** are files in [`.yidam/corpus/`](../.yidam/corpus/). One concept per file. Every
  node needs at least one outgoing link.
- **Sources** are files in [`.yidam/catalog/`](../.yidam/catalog/). Corpus nodes cite them by
  link rather than embedding citations inline.
- **Claims** carry `[verified]`, `[inference]`, or `[open]`. Untagged inference is a defect.
- **Commits** are knowledge events. Epistemic commits (understanding changed) and operational
  commits (a pipeline ran) are written in visibly different styles and never mixed.
- **`.yidam/.vendor/` is read-only.** Fix prelude defects by re-vendoring
  (`mise run yidam-vendor-update`), never by editing in place.
- **Read the corpus with `yidam`, not with `grep`.** It is a typed graph and `grep` is a text
  tool; see below.
- **`crates/` is not the engine.** The engine is what the corpus describes; `crates/` is the
  instrument that queries the corpus. Both are Rust, and conflating them is this repository's
  characteristic mistake.

## Before writing

```
yidam due                                    # what is owed — stale index, open questions
yidam query 'interaction -applies-> verb'    # which verbs the seeded interactions exercise
yidam neighbors verb/use-with.yml            # one node's neighbourhood
yidam pack 'prior-art -exhibits-> failure-mode' --budget 4000   # that answer as prose
```

The loop is **find a gap → retrieve → correct → settle a phase**, and the first two steps are
these. `grep` finds a string; `query` finds a node, an edge, or a coverage gap — and when it
matches nothing it says *which kind* of nothing, which a zero-hit `grep` cannot. Full surface:
[reading the corpus](../.yidam/.vendor/prelude/guidelines/reading-the-corpus.md).

## Before committing

```
mise run regen           # refresh REGEN blocks, then commit the result as `regen:`
mise run ci              # graph-check, graph-lint, regen --check — the whole gate
```

`mise run ci` is what CI runs, and the two are held together by a test upstream. Run it
rather than its parts: `graph-check` reads the graph and says nothing about a stale REGEN
block, which is a failing build all the same.
