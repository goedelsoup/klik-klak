# Data Format

A data format is an on-disk representation an author writes and the engine reads. It is
where the vocabulary stops being an idea: a primitive that cannot be written down is not
part of an *authoring* vocabulary, whatever the runtime does with it. The `escape_hatch`
property is the one to read first — what a format cannot express is what sends an author to
a scripting language, which is the boundary the central question is drawn around.

Class definition: [`data-format.ont.yml`](../data-format.ont.yml)
