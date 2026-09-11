# Actions — Failure Mode

Operations, queries and skills that apply to instances of [`failure-mode`](../failure-mode.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What produces it** — `yidam query 'failure-mode -arises-from-> puzzle'` and the same
  from `conversation`
- **What stops it** — `yidam query 'failure-mode -prevented-by-> data-format'` and the same
  to `subsystem`; a mode with neither is one this project has no answer to yet
- **Who shipped it** — `yidam query 'prior-art -exhibits-> failure-mode'`
- **Detect it mechanically** — `cargo run -p reachability -- design/specimen/lighthouse.game`.
  It had no game to read when this page was written and has four now, finding five strandings
  across them and distinguishing the two causes rather than summing them.
