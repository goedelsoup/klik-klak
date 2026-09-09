# Inventory Item

An inventory item is what an object plays once the player is carrying it and can offer it as
the second object of a binary verb. It is typed as a UFO **role** rather than a kind, which
is a claim: the same object was a hotspot in the room before it was picked up, and carriage
is contingent. An engine that models the two as separate kinds of thing has to reconcile
them at pickup time, and that reconciliation is where a class of authoring bug lives.

Class definition: [`inventory-item.ont.yml`](../inventory-item.ont.yml)
