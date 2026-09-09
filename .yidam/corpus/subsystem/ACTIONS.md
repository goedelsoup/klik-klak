# Actions — Subsystem

Operations, queries and skills that apply to instances of [`subsystem`](../subsystem.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What a component makes work** — `yidam query 'subsystem -realizes-> interaction'`
- **What it needs to do that** — `yidam query 'subsystem -depends-on-> subsystem'`
- **What it reads** — `yidam query 'data-format -read-by-> subsystem'`
- **What it makes impossible** — `yidam query 'failure-mode -prevented-by-> subsystem'`;
  this is the engine tier's answer to the comparative tier
