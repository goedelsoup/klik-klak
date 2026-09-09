# Actions — Conversation

Operations, queries and skills that apply to instances of [`conversation`](../conversation.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Who an exchange is with** — `yidam query 'conversation -held-with-> actor'`
- **What an exchange writes** — `yidam query 'conversation -sets-> world-flag'`
- **Which exchanges strand a branch** — `yidam query 'failure-mode -arises-from-> conversation'`
- *Open:* whether `interaction -opens-> conversation` reduces this class to a response
  type. The edge exists so the reduction can be argued from the graph rather than asserted.
