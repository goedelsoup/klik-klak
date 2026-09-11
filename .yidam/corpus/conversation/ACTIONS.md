# Actions — Conversation

Operations, queries and skills that apply to instances of [`conversation`](../conversation.ont.yml). Stub entries are expected; this file grows as the
corpus does.

- **Who an exchange is with** — `yidam query 'conversation -held-with-> actor'`
- **What an exchange writes** — `yidam query 'conversation -sets-> world-flag'`
- **Which exchanges strand a branch** — `yidam query 'failure-mode -arises-from-> conversation'`
- **Whether the class reduces** — it does, and it keeps its place anyway:
  [conversation-reduction](../../decisions/conversation-reduction.yml) found it reduces to a room
  exactly and that what does not reduce is `leave`. The class turns on the return.
