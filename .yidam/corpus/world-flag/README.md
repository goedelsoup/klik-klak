# World Flag

A world flag is a unit of persistent state that interactions write and preconditions read.
It is typed as a UFO **mode** because it has no existence apart from the game world it
inheres in. It is the only thing in this vocabulary that carries meaning across rooms —
without it, nothing an author writes in one room is visible from another — which is why a
vocabulary cannot be minimal by dropping it.

Class definition: [`world-flag.ont.yml`](../world-flag.ont.yml)
