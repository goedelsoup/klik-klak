//! **The writer is total, because the grammar is disjunctive normal form.**
//!
//! `.yidam/decisions/round-trip.yml` closed on it: *"`Pred` can represent `(a or b) and c`, which
//! no `.game` file can write and this writer cannot print; the reader cannot build one, because
//! `conj` only ever nests `unary`. **The tree is larger than the language**, the writer's
//! precedence assertion is where that is stated, and it has never fired."*
//!
//! It had never fired because nothing builds such a tree — the parser cannot, and no consumer
//! transforms one. The defect was invisible in every way a defect can be: `print(&Game) ->
//! String` was total in its type, partial in fact, and the input that would show it was one this
//! repository had no way to produce.
//!
//! **What is smaller is the set of shapes, not the set of formulas.** `disj := conj ('or'
//! conj)*`, `conj := unary ('and' unary)*`, `unary := 'not' unary | atom` — a disjunction of
//! conjunctions of literals, which is **disjunctive normal form**, and every propositional
//! formula has one. `(a or b) and c` is spelled `a and c or b and c`. [`Pred::normal`] is that
//! rewriting, `print` calls it, and there is no predicate this language cannot write.
//! `.yidam/decisions/normal-form.yml`.

use game_format::*;

fn atom(name: &str) -> Pred {
    Pred::Flag(name.to_string())
}

/// Every predicate over three flags, to the given depth.
fn generated(depth: usize) -> Vec<Pred> {
    if depth == 0 {
        return vec![atom("p"), atom("q"), atom("r")];
    }
    let below = generated(depth - 1);
    let mut out = below.clone();
    out.extend(below.iter().map(|x| Pred::Not(Box::new(x.clone()))));
    for x in &below {
        for y in &below {
            out.push(Pred::And(Box::new(x.clone()), Box::new(y.clone())));
            out.push(Pred::Or(Box::new(x.clone()), Box::new(y.clone())));
        }
    }
    out
}

/// The value of a generated predicate under one assignment to `p`, `q`, `r`.
fn truth(p: &Pred, v: [bool; 3]) -> bool {
    match p {
        Pred::Flag(n) => v["pqr".find(n.as_str()).expect("a generated flag")],
        Pred::Not(x) => !truth(x, v),
        Pred::And(x, y) => truth(x, v) && truth(y, v),
        Pred::Or(x, y) => truth(x, v) || truth(y, v),
        other => unreachable!("{other:?} is not generated here"),
    }
}

/// **The writer writes the shape it used to abort on.**
///
/// `And(Or(a, b), c)` is `(a or b) and c`. `Game` can hold one, `print` takes a `&Game`, and the
/// call used to abort the process. It comes out as `a and c or b and c` — the same formula, in
/// the form the grammar spells — and reads back as a game that says the same thing.
#[test]
fn the_writer_writes_the_shape_it_used_to_abort_on() {
    let source = "\
flag a \"a\"
flag b \"b\"
flag c \"c\"
room r \"R\"
room r
  start
  hotspot h \"h\"
    look-at:
      when a and c or b and c
      \"x\"
      ending
";
    let (mut spelled, problems) = parse(source, "t");
    assert!(problems.is_empty(), "{problems:?}");
    assert!(resolve(&mut spelled).is_empty());

    // the same game with the guard written the way the reader cannot
    let mut unspellable = parse(source, "t").0;
    resolve(&mut unspellable);
    unspellable.bindings[0].when = Some(Pred::And(
        Box::new(Pred::Or(Box::new(atom("a")), Box::new(atom("b")))),
        Box::new(atom("c")),
    ));

    let written = print(&unspellable);
    assert!(
        written.contains("when  a and c or b and c"),
        "distributed into the shape the grammar spells:\n{written}"
    );
    let (back, problems) = parse(&written, "t");
    assert!(problems.is_empty(), "and it reads: {problems:?}");
    assert_eq!(
        format!("{:?}", back.bindings[0].when),
        format!("{:?}", spelled.bindings[0].when),
        "as the same predicate the author could have written by hand"
    );
}

/// **Every predicate normalises, means the same thing, and lands in a shape the reader could have
/// produced.**
///
/// 1,200 predicates over three flags to depth two — every combination of `and`, `or` and `not` —
/// checked in all eight valuations. Generated rather than sampled from the specimens, because the
/// specimens contain one `and` and no `or` and would exercise none of it.
#[test]
fn normalising_preserves_meaning_and_lands_in_normal_form() {
    let all = generated(2);
    assert_eq!(all.len(), 1200, "predicates to depth two over three flags");
    for p in &all {
        let n = p.normal();
        assert!(
            n.is_normal(),
            "not a shape the reader could produce: {p:?} -> {n:?}"
        );
        for i in 0..8u8 {
            let v = [i & 1 != 0, i & 2 != 0, i & 4 != 0];
            assert_eq!(
                truth(p, v),
                truth(&n, v),
                "the meaning moved: {p:?} -> {n:?} at {v:?}"
            );
        }
        assert_eq!(
            format!("{:?}", n.normal()),
            format!("{n:?}"),
            "not idempotent: {p:?}"
        );
    }
}

/// **A double negation is cancelled only where it is in the way.**
///
/// `not not a` is writable — `unary := 'not' unary` — so cancelling it would be a simplification,
/// and a writer that simplifies loses the shape an author wrote. `not not (a and b)` is unwritable
/// at any depth, and there the cancellation is the only way through. The reader can produce the
/// first and not the second.
#[test]
fn a_double_negation_survives_unless_it_is_in_the_way() {
    let kept = Pred::Not(Box::new(Pred::Not(Box::new(atom("p")))));
    assert_eq!(
        format!("{:?}", kept.normal()),
        format!("{kept:?}"),
        "`not not a` is written as it was"
    );

    let blocked = Pred::Not(Box::new(Pred::Not(Box::new(Pred::And(
        Box::new(atom("p")),
        Box::new(atom("q")),
    )))));
    let out = blocked.normal();
    assert!(out.is_normal());
    assert_eq!(
        format!("{out:?}"),
        format!("{:?}", Pred::And(Box::new(atom("p")), Box::new(atom("q")))),
        "`not not (a and b)` has nowhere else to go"
    );
    for i in 0..4u8 {
        let v = [i & 1 != 0, i & 2 != 0, false];
        assert_eq!(truth(&blocked, v), truth(&out, v));
    }
}

/// **It is the identity on everything the reader builds.**
///
/// Every predicate in the four specimens, unchanged. A normaliser that moved a parsed tree would
/// be doing something other than spelling — and the round trip in `round_trip.rs` would stop
/// returning the same game.
#[test]
fn normalising_changes_no_predicate_any_specimen_wrote() {
    const ALL: [(&str, &str); 4] = [
        ("ferry", include_str!("../../../design/specimen/ferry.game")),
        (
            "lighthouse",
            include_str!("../../../design/specimen/lighthouse.game"),
        ),
        (
            "nightwatch",
            include_str!("../../../design/specimen/nightwatch.game"),
        ),
        (
            "patrol",
            include_str!("../../../design/specimen/patrol.game"),
        ),
    ];
    let mut seen = 0;
    for (name, source) in ALL {
        let (mut game, problems) = parse(source, name);
        assert!(problems.is_empty(), "{name}: {problems:?}");
        assert!(resolve(&mut game).is_empty());
        for p in game.bindings.iter().flat_map(|b| {
            b.when.iter().chain(
                b.says
                    .iter()
                    .chain(b.refusals.iter())
                    .filter_map(|r| r.when.as_ref()),
            )
        }) {
            seen += 1;
            assert!(p.is_normal(), "{name}: the reader built {p:?}");
            assert_eq!(
                format!("{:?}", p.normal()),
                format!("{p:?}"),
                "{name}: normalising moved a predicate an author wrote"
            );
        }
    }
    assert_eq!(seen, 36, "guards and guarded clauses across the four");
}

/// **The only thing that builds a predicate outside the parser is the normaliser.**
///
/// This is why the partiality was invisible: the parser cannot produce an unspellable tree, and
/// for four phases nothing else produced one at all. `Pred::normal` is now the second producer,
/// and what it produces is normal by construction — which is the test above rather than this one.
///
/// Anything else that starts building predicates is a third producer with no such guarantee, and
/// this is where that shows up.
#[test]
fn only_the_parser_and_the_normaliser_build_a_predicate() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("two levels below the root")
        .to_path_buf();

    let mut building = Vec::new();
    for workspace in ["crates", "engine"] {
        let mut stack = vec![root.join(workspace)];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().is_some_and(|n| n == "target" || n == "tests") {
                        continue;
                    }
                    stack.push(p);
                    continue;
                }
                if p.extension().is_some_and(|x| x != "rs")
                    || p.file_name()
                        .is_some_and(|n| n == "parse.rs" || n == "lib.rs")
                {
                    continue;
                }
                let Ok(src) = std::fs::read_to_string(&p) else {
                    continue;
                };
                for line in src.lines() {
                    // a constructor call, not a pattern: `Pred::Or(Box::new(` builds one
                    if line.contains("Pred::") && line.contains("Box::new(") {
                        building.push(format!("{}: {}", p.display(), line.trim()));
                    }
                }
            }
        }
    }
    assert!(
        building.is_empty(),
        "something builds a predicate outside the parser and the normaliser, so a shape could \
         reach the writer with no guarantee about it: {building:?}"
    );

    // and the normaliser is in the model, where the claim it makes belongs
    let lib = std::fs::read_to_string(root.join("crates/game-format/src/lib.rs")).expect("it");
    assert!(
        lib.contains("pub fn normal(&self) -> Pred {") && lib.contains("fn distribute(p: &Pred)"),
        "`normal` is a property of the language rather than of printing"
    );
    let writer = std::fs::read_to_string(root.join("crates/game-format/src/print.rs")).expect("it");
    assert!(
        writer.contains("let p = &p.normal();"),
        "and the writer calls it rather than re-deriving it"
    );
}

/// **The grammar is disjunctive normal form, which is what makes the gap a spelling and not a
/// limit.**
///
/// Three levels, each nesting only the one below it. `And`'s children are `unary`, so a
/// disjunction cannot sit under a conjunction; `Not`'s operand is a `unary`, so a conjunction
/// cannot sit under a negation. What the reader can spell is a disjunction of conjunctions of
/// literals, exactly.
#[test]
fn the_reader_spells_a_disjunction_of_conjunctions_of_literals() {
    let src = include_str!("../src/parse.rs");
    for (level, nests) in [
        ("fn disj<'a>", "let (mut left, mut rest) = conj(t)?;"),
        ("fn conj<'a>", "let (mut left, mut rest) = unary(t)?;"),
    ] {
        let at = src.find(level).expect("the level");
        assert!(
            src[at..at + 400].contains(nests),
            "{level} nests only the level below it"
        );
    }
    let at = src.find("fn unary<'a>").expect("the level");
    let body = &src[at..at + 400];
    assert!(
        body.contains("\"not\"") && body.contains("unary(") && body.contains("atom("),
        "`not` takes a unary, which is a literal under any number of negations"
    );

    // and the four shapes, checked by reading them back
    for (source, writable) in [
        ("a and b or c", true),
        ("a or b and c", true),
        ("not a or not b", true),
        ("not a and not b", true),
    ] {
        let (game, problems) = parse(
            &format!(
                "flag a \"a\"\nflag b \"b\"\nflag c \"c\"\nroom r \"R\"\nroom r\n  start\n  \
                 hotspot h \"h\"\n    look-at:\n      when {source}\n      \"x\"\n      ending\n"
            ),
            "t",
        );
        assert_eq!(
            problems.is_empty(),
            writable,
            "`{source}` should {} read",
            if writable { "" } else { "not" }
        );
        if writable {
            assert!(
                game.bindings.iter().any(|b| b.when.is_some()),
                "`{source}` read as a guard"
            );
        }
    }
}
