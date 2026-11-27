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

mod common;

use game_format::{parse, print, resolve, Game};

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
    for (name, src) in common::sources() {
        let first = load(&src, &name);
        let written = print(&first);
        let second = load(&written, &name);
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

/// **299 of 904 lines do not survive, and none of it is meaning.**
///
/// 186 comments, 3 `??` annotations and 185 blank lines: a third of a `.game` file is for whoever
/// reads the file. A writer that reproduced them would be claiming they are in the tree.
///
/// The share held at a third across a fifth game, which is worth more than the figure: a ratio
/// that survives an author who was not writing to it is a fact about the format rather than about
/// four files.
#[test]
fn a_third_of_the_file_is_for_whoever_reads_the_file() {
    let (mut before, mut after) = (0, 0);
    for (name, src) in common::sources() {
        before += src.lines().count();
        after += print(&load(&src, &name)).lines().count();
    }
    assert_eq!((before, after), (1115, 751), "lines written, lines kept");
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
    for (_, src) in common::sources() {
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
    assert_eq!((say, otherwise, bare), (53, 26, 45), "the three spellings");

    // and after a round trip there is one spelling and the same number of lines
    let (mut written_say, mut written_other) = (0, 0);
    let mut clauses = 0;
    for (name, src) in common::sources() {
        let game = load(&src, &name);
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

/// **The tree is not larger than the language, and this test said so twice in opposite
/// directions.**
///
/// It began as *"`Pred::And(Box::new(Pred::Or(..)), ..)` is `(a or b) and c`: constructible in the
/// type, and unwritable in a `.game` file, which has no parentheses."* That was corrected once, to
/// *the grammar is disjunctive normal form, so it is written `a and c or b and c`* — the writer
/// normalising, `.yidam/decisions/normal-form.yml` — and the correction kept the premise: that the
/// nesting is one-way and the reader cannot build the shape.
///
/// **The premise was wrong both times.** `atom := '(' pred ')'` is the fourth production and it
/// recurses to the top, so the reader has always built `(a or b) and c` from a file that writes
/// it. Nothing found out for eighty-eight phases because no author had written a disjunction, and
/// the checks that would have found it read the first three productions and stopped.
///
/// So the writer prints brackets and the round trip below is a **structural** identity rather than
/// a logical one: every predicate comes back as the tree the author wrote, not as a formula equal
/// to it. `.yidam/decisions/or-earns-it.yml`.
#[test]
fn the_reader_builds_only_shapes_the_writer_writes_without_rewriting() {
    let src = include_str!("../src/parse.rs");
    assert!(
        src.contains("fn conj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String>")
            && src
                .contains("fn disj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String>"),
        "the two levels that set the precedence"
    );
    assert!(
        src.contains("Some(&\"(\") => {"),
        "and the production that makes the nesting two-way after all"
    );
    let writer = include_str!("../src/print.rs");
    assert!(
        !writer.contains(".normal()") && writer.contains("format!(\"({s})\")"),
        "the writer brackets what binds looser rather than distributing it"
    );

    // every predicate in the four specimens prints and reads back as itself, unrewritten
    for (name, src) in common::sources() {
        let game = load(&src, &name);
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
        // Not `is_normal` any more. That assertion was checking the premise above rather than
        // anything about the round trip, and it read as a property of the reader when it was a
        // property of what six authors happened to have written. What matters is the line below:
        // whatever the reader builds, the writer writes back as itself.
        let after: Vec<String> = load(&print(&game), &name)
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
