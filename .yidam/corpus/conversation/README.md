# Conversation

A conversation is a structured exchange with an actor in which the choices available change
as the exchange proceeds. Its claim to being irreducible rests on that last clause: a
dialogue node's availability depends on the path taken through the conversation, and no
verb-target binding records a path. Whether that is enough to keep it out of `interaction`
is a live question in this corpus.

**What an instance of this class is:** an authored structure of choices offered against an actor,
in either of two shapes. The flat persistent list is the base form — AGS's `return` goes back to
the choices list, and `stop` is what ends a conversation — and the branching tree is what an author
builds by adding navigation to it. This corpus had the two the other way round until 2026-09-09;
see `.yidam/decisions/conversation-boundary.yml`.

The class is **not** founded on branching. It is founded on what both shapes share: a conversation
is entered and left as a mode, and it carries which of its choices are currently available. A set
of bindings has neither property.

Class definition: [`conversation.ont.yml`](../conversation.ont.yml)
