# Actions — Interaction

Operations, queries and skills that apply to instances of [`interaction`](../interaction.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What a verb can be applied to** — `yidam neighbors interaction/<instance>.yml`
- **Which bindings write state** — follow `sets` to `world-flag`; a binding that writes
  nothing cannot advance a puzzle
- **Which bindings have no authored refusal** — the `refusal` property absent is the
  precondition failing silently, which is `failure-mode/unwinnable-state`'s ordinary cause
- *Open:* whether a binding whose response is an exchange is still one of these, or whether
  `conversation` is a separate primitive. The `opens` edge is where that argument lands.
