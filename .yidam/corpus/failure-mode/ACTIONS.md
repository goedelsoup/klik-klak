# Actions — Failure Mode

Operations, queries and skills that apply to instances of [`failure-mode`](../failure-mode.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What produces it** — `yidam query 'failure-mode -arises-from-> puzzle'` and the same
  from `conversation`
- **What stops it** — `yidam query 'failure-mode -prevented-by-> data-format'` and the same
  to `subsystem`; a mode with neither is one this project has no answer to yet
- **Who shipped it** — `yidam query 'prior-art -exhibits-> failure-mode'`
- *Stub:* `reachability` would detect `unwinnable-state` mechanically, given a game to read.
