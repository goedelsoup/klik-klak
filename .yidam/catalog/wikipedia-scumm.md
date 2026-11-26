---
name: Wikipedia — SCUMM
description: Tertiary account of the SCUMM engine, its verb interface, and the years it was in use.
type: other
obtained: true
retrieved: 2026-09-09
ttl_days: 730
location:
  - kind: url
    value: https://en.wikipedia.org/wiki/SCUMM
---

**This is an aggregator and it can never support a `[verified]` claim in this corpus.** SCUMM
itself is not public: LucasArts' authoring documentation was never released, and the source of
record is unreachable rather than merely unfetched. That makes this article a republisher of
figures whose origin is elsewhere, and `agent-conduct` is explicit that such figures may be
retrieved, cited and reasoned from at `[inference]` and never promoted past it, however
accurate they are. Every claim in `prior-art/scumm.yml` that rests on this entry stays
`[inference]` *with* a citation, which is a real improvement on `[inference]` from
recollection and is not the same thing as evidence.

## What was read, and what it supports

Quoted from the article: "A large set of these were often featured at the bottom of the screen
in the early games, but by *Full Throttle* (1995) and *The Curse of Monkey Island* (1997) these
had been reduced to a 'verb coin' appearing at the mouse cursor with the option to use
character's eyes (to *look*), hands (to *use*, *pick up*, *push*, *pull*, etc.), or mouth (to
*talk*, *consume*, *inhale*)."

That supports the contraction this corpus claimed at genesis, and it adds something the corpus
did not have: the verb coin is a *third* discovery surface, distinct from both the on-screen
verb bar and AGS's cursor modes.

**Re-read for movement, and the quoted list is usable as a closed one.** Three groups — eyes,
hands, mouth — and no legs. The sentence enumerates the coin's options rather than sampling them
("with the option to use character's eyes…, hands…, or mouth…"), which is what makes the absence
readable at all; it is still an aggregator and still `[inference]`. The same contraction kept
`talk` as a group of its own and folded `pick-up` into hands, and it did not fold walking
anywhere. See `verb/walk-to.yml`.

Also read: SCUMM converted human-readable commands into byte-sized tokens read by an
interpreter; it was developed in 1987 and abandoned in 1998; Humongous Entertainment continued
using it through 2003.

## What was looked for and not found — read this before citing

**The article does not state how many verbs Maniac Mansion presented.** This corpus went
looking for that number specifically, because `prior-art/scumm.yml` declined to assert it at
genesis and this was the retrieval that would have settled it. It did not.

What the article offers instead is a quote from Aric Wilmunder that "the version of SCUMM for
*Maniac Mansion* had about 80% of the commands that would end up being used in the later
versions of the engine" — which is about the *scripting language's* command set and not about
the verbs a player saw, and which points in the opposite direction from a contraction. Reading
it as a verb count would be a straightforward error of the kind this ledger exists to prevent.

Secondary search results reached during the same retrieval — Fandom's Monkey Island and
Lucasfilm wikis, HandWiki — do carry specific counts (fifteen verbs in Maniac Mansion, reduced
to nine, then three). They are aggregators of an aggregator and none of them is cited here. The
figures are recorded in this paragraph as *what was examined and not used*, so that the next
reader knows the numbers exist, knows they were seen, and knows why the corpus does not assert
them.

## What this document holds that nobody here has looked at

Most of the article: the version history of the engine, the games in each version, the
technical details of the interpreter, and the SCUMM-adjacent tooling. The corpus read it for
the verb interface and stopped.
