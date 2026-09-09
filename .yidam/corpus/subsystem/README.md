# Subsystem

A subsystem is a runtime component of the engine, with a boundary an author never sees. The
authoring vocabulary says nothing about what runs it — that is the point of separating the
tiers — but the cost of a primitive is only visible on this side, and a primitive whose
realization requires a component nothing else needs is a primitive worth questioning.

Class definition: [`subsystem.ont.yml`](../subsystem.ont.yml)
