# Actions — World Flag

Operations, queries and skills that apply to instances of [`world-flag`](../world-flag.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **What a flag holds closed** — `yidam query 'world-flag -gates-> puzzle'`
- **What writes it** — `yidam query 'interaction -sets-> world-flag'` and the same from
  `conversation` and `cutscene`
- **Save surface** — read `save_semantics`; a flag omitted from a save is a flag whose
  puzzle reopens on load
- **What `scope` ranges over** — four values, and the fourth is the one a real game uses most.
  The axis is real and the reduction this page proposed does not reach the dominant value: object
  states are read from other rooms by qualified name. [object-state](object-state.yml) carries the
  evidence.
