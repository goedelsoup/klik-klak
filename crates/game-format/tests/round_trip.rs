//! **`parse(print(parse(src))) == parse(src)`, for all four specimens.**
//!
//! The question `.yidam/decisions/parser-silence.yml` could not reach. That phase made the reader
//! account for every token on every line, which answers *did anything go unread*. This answers
//! *did anything go unkept*, and the two fail differently: a token the reader drops is a line
//! nobody meant, and a token the tree drops is a line somebody wrote that no consumer can find.
//!
//! **The tree had lost something three times in seven phases and every one was found from the
//! outside** — labels and response text when a menu and a presenter wanted them, `spoken_with`
//! when a menu asked who a conversation was held with. A fourth was a reasonable expectation.
//! Asking directly costs one module and answers it for every field at once.
//!
//! It also checks two records by *executing* them, which is not what a printer is usually for:
//! `exit-archetype` says an `exit` expands into the binding an author would have written, and
//! `authored-text` says the file's three spellings of an unguarded line are one construct. Both
//! describe an artefact; the writer produces it. `.yidam/decisions/round-trip.yml`.

use game_format::{parse, print, resolve, Game};

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

fn load(source: &str, name: &str) -> Game {
    let (mut game, problems) = parse(source, name);
    assert!(problems.is_empty(), "{name} does not read: {problems:?}");
    let problems = resolve(&mut game);
    assert!(problems.is_empty(), "{name} is not admitted: {problems:?}");
    game
}

/// The tree with its line numbers taken out.
///
/// Line numbers are the one thing a round trip may move, because the writer emits no comments
/// and no blank lines and so cannot land a construct where the author left it. Everything else
/// must come back — and taking the numbers out is what makes *everything else* checkable rather
/// than drowned.
fn shape(game: &Game) -> String {
    format!("{game:#?}")
        .lines()
        .map(|l| {
            let t = l.trim();
            if let Some(i) = l.find("line: ") {
                format!("{}line: <n>,", &l[..i])
            } else if t
                .trim_end_matches(',')
                .chars()
                .all(|c| c.is_ascii_digit() && !t.is_empty())
                && !t.is_empty()
            {
                "    <line>,".to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// **Every specimen survives a round trip whole, and the writer is idempotent.**
///
/// The tree is the thing compared and not the text, because the text carries a third of itself
/// for whoever reads it — comments, annotations, blank lines — and none of that is in the tree
/// or should be.
#[test]
fn every_specimen_comes_back_as_the_same_game() {
    for (name, src) in ALL {
        let first = load(src, name);
        let written = print(&first);
        let second = load(&written, name);
        assert_eq!(
            shape(&first),
            shape(&second),
            "{name} does not come back as itself"
        );
        assert_eq!(
            print(&second),
            written,
            "{name}: the writer is not idempotent"
        );
    }
}

/// **221 of 662 lines do not survive, and none of it is meaning.**
///
/// 133 comments, 3 `??` annotations and 143 blank lines: a third of a `.game` file is for
/// whoever reads the file. A writer that reproduced them would be claiming they are in the tree.
#[test]
fn a_third_of_the_file_is_for_whoever_reads_the_file() {
    let (mut before, mut after) = (0, 0);
    for (name, src) in ALL {
        before += src.lines().count();
        after += print(&load(src, name)).lines().count();
    }
    assert_eq!((before, after), (662, 441), "lines written, lines kept");
}

/// **An `exit` comes back as the binding an author would have written, which is the claim.**
///
/// `.yidam/decisions/exit-archetype.yml`: *"It **expands at parse time** into the binding an
/// author would otherwise have written — a hotspot, a `walk-to`, an `Effect::Move` — so nothing
/// downstream knows it exists."* Printing the tree produces that binding and it is those three.
///
/// On a fixture, because no specimen writes an `exit`. Worth saying rather than hiding: the
/// construct carrying the strongest claim in this format is the one nothing uses.
#[test]
fn an_exit_comes_back_as_the_binding_it_expands_into() {
    let src = "\
item k \"a key\"
room a \"A\"
room b \"B\"
room a
  start
  exit door to b
room b
  hotspot y \"another\"
    look-at:
      \"ho\"
      ending
";
    let game = load(src, "exits");
    let written = print(&game);
    assert!(
        written.contains("  hotspot door\n    walk-to:\n      do    move b\n"),
        "an exit is written back as a hotspot, a walk-to and a move:\n{written}"
    );
    assert!(
        !written.contains("exit "),
        "and never as an exit, because nothing downstream knows it exists"
    );
    assert_eq!(
        shape(&game),
        shape(&load(&written, "exits")),
        "and the two are one game"
    );
}

/// **Three spellings of an unguarded line come back as one, and nothing moves.**
///
/// `say` 28 times, `otherwise` 11, a bare quoted line 34 — and the tree holds 73 identical
/// `Response`s with no predicate. The writer emits `say` for all three. What tells them apart is
/// where they sit in the list, which is order, which the tree keeps: first-match-wins is why
/// `otherwise` is a fallback, and `.yidam/decisions/authored-text.yml` says so.
#[test]
fn three_spellings_of_a_line_come_back_as_one() {
    let (mut say, mut otherwise, mut bare) = (0, 0, 0);
    for (_, src) in ALL {
        for line in src.lines() {
            let t = line.trim_start();
            if t.starts_with("say ") {
                say += 1;
            } else if t.starts_with("otherwise") {
                otherwise += 1;
            } else if t.starts_with('"') {
                bare += 1;
            }
        }
    }
    assert_eq!((say, otherwise, bare), (28, 11, 34), "the three spellings");

    // and after a round trip there is one spelling and the same number of lines
    let (mut written_say, mut written_other) = (0, 0);
    let mut clauses = 0;
    for (name, src) in ALL {
        let game = load(src, name);
        clauses += game
            .bindings
            .iter()
            .flat_map(|b| b.says.iter())
            .filter(|r| r.when.is_none())
            .count();
        for line in print(&game).lines() {
            let t = line.trim_start();
            if t.starts_with("say ") {
                written_say += 1;
            } else if t.starts_with("otherwise") {
                written_other += 1;
            }
        }
    }
    assert_eq!(clauses, say + otherwise + bare, "one thing in the tree");
    assert_eq!(
        (written_say, written_other),
        (clauses, 0),
        "and one spelling on the way out"
    );
}

/// **The tree is larger than one spelling of the language, and the writer closes the gap rather
/// than reporting it.**
///
/// This test read: *"`Pred::And(Box::new(Pred::Or(..)), ..)` is `(a or b) and c`: constructible in
/// the type, and unwritable in a `.game` file, which has no parentheses."* The first half stands
/// and the conclusion does not. The grammar is **disjunctive normal form** — `disj := conj ('or'
/// conj)*`, `conj := unary ('and' unary)*` — and every propositional formula has one, so `(a or b)
/// and c` is written `a and c or b and c`. [`Pred::normal`] is that rewriting and `print` calls
/// it. `.yidam/decisions/normal-form.yml`.
///
/// What the reader still cannot build is the *shape*, and that is why normalising is the identity
/// on every predicate in every specimen — the round trip below is unchanged by any of it.
#[test]
fn the_reader_builds_only_shapes_the_writer_writes_without_rewriting() {
    let src = include_str!("../src/parse.rs");
    assert!(
        src.contains("fn conj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String>")
            && src
                .contains("fn disj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String>"),
        "the two levels that make the nesting one-way"
    );
    let writer = include_str!("../src/print.rs");
    assert!(
        writer.contains("let p = &p.normal();"),
        "and the writer normalises rather than asserting about the language"
    );

    // every predicate in the four specimens prints and reads back as itself, unrewritten
    for (name, src) in ALL {
        let game = load(src, name);
        let before: Vec<String> = game
            .bindings
            .iter()
            .flat_map(|b| {
                b.when
                    .iter()
                    .chain(b.says.iter().filter_map(|r| r.when.as_ref()))
            })
            .map(|p| format!("{p:?}"))
            .collect();
        for p in game.bindings.iter().flat_map(|b| {
            b.when
                .iter()
                .chain(b.says.iter().filter_map(|r| r.when.as_ref()))
        }) {
            assert!(
                p.is_normal(),
                "{name}: the reader built an unspellable shape"
            );
        }
        let after: Vec<String> = load(&print(&game), name)
            .bindings
            .iter()
            .flat_map(|b| {
                b.when
                    .iter()
                    .chain(b.says.iter().filter_map(|r| r.when.as_ref()))
            })
            .map(|p| format!("{p:?}"))
            .collect();
        assert_eq!(before, after, "{name}: a predicate changed shape");
        assert!(!before.is_empty(), "{name} has guards to check");
    }
}
