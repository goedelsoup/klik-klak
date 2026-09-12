//! **`print` is a partial function and its signature says otherwise.**
//!
//! `.yidam/decisions/round-trip.yml` closed on it: *"`Pred` can represent `(a or b) and c`, which
//! no `.game` file can write and this writer cannot print; the reader cannot build one, because
//! `conj` only ever nests `unary`. **The tree is larger than the language**, the writer's
//! precedence assertion is where that is stated, and it has never fired."*
//!
//! It has never fired because nothing builds such a tree — the parser cannot, and no consumer
//! transforms one. So the defect is invisible in every way a defect can be: `print(&Game) ->
//! String` is total in its type, partial in fact, and the input that would show it is one this
//! repository has no way to produce.
//!
//! **And the shape of the language is the thing that makes it fixable.** `disj := conj ('or'
//! conj)*`, `conj := unary ('and' unary)*`, `unary := 'not' unary | atom` — a disjunction of
//! conjunctions of literals, which is **disjunctive normal form**. Whether a formula the reader
//! cannot spell is a formula the language cannot *say* is a different question from the one that
//! record asked, and this census is what asking it needs.

use game_format::*;

fn atom(name: &str) -> Pred {
    Pred::Flag(name.to_string())
}

/// **The writer panics on a shape its argument type admits.**
///
/// `And(Or(a, b), c)` is `(a or b) and c`. `Game` can hold one, `print` takes a `&Game`, and the
/// call aborts the process.
#[test]
fn the_writer_panics_on_a_predicate_its_argument_type_admits() {
    let mut game = Game {
        source: "t".into(),
        ..Default::default()
    };
    game.rooms.push("r".into());
    game.start_room = Some("r".into());
    game.bindings.push(Binding {
        site: Site::InRoom {
            room: "r".into(),
            target: "h".into(),
        },
        scope: Scope::Room("r".into()),
        verb: "look-at".into(),
        object: None,
        when: Some(Pred::And(
            Box::new(Pred::Or(Box::new(atom("a")), Box::new(atom("b")))),
            Box::new(atom("c")),
        )),
        refuses: false,
        says: Vec::new(),
        refusals: Vec::new(),
        effects: Vec::new(),
        ending: true,
        line: 1,
    });

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(move || print(&game));
    std::panic::set_hook(hook);
    assert!(
        out.is_err(),
        "`(a or b) and c` prints now, which is what this census is about"
    );
}

/// **Nothing in either workspace can build one, which is why it has never fired.**
///
/// The parser builds `And` only from two `unary`s and `Not` only over a `unary`, so every tree it
/// produces is already in normal form. Nothing else constructs a `Pred` at all: `resolve` matches
/// on them to compare two guards and builds none, and no engine crate names the type.
#[test]
fn no_source_outside_the_parser_builds_a_predicate() {
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
                    || p.file_name().is_some_and(|n| n == "parse.rs")
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
        "something builds a predicate outside the parser, so the shape could reach the writer \
         from inside: {building:?}"
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
