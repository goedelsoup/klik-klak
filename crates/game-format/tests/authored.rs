//! **Half of what an author writes never reaches the parse tree.**
//!
//! `crates/game-format` reads the `.game` surface into a model a checker can search, and the
//! model has no place to put a word of player-facing text. `parse.rs` matches
//! `"say" | "otherwise" => {}`; a `when <pred> "<line>"` keeps the predicate and drops the line
//! beside it; a declaration's trailing quoted string is never looked at. `Binding` has no text
//! field and `Game` has no labels.
//!
//! **That is the correct position for a checker and it is stated as one.**
//! `.yidam/decisions/effect-list-semantics.yml` says so in as many words — *"`say` is
//! presentation and not an effect… `Binding` has no field for response text. That is now the
//! stated position rather than an omission in a parser."* Nothing below disputes it. What the
//! census holds is that the sentence describes **two different things that were never
//! separated**: the state model ignoring presentation, and the reader throwing it away. Only the
//! first is a position.
//!
//! The consequence is the whole of why `engine/` holds no code. No runtime built on this tree
//! can emit a word, because the words are not in it.
//!
//! These checks pin the defect. They are rewritten into the rule in the same phase.

use game_format::{parse, resolve, Game};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn specimens() -> Vec<(String, String, Game)> {
    let dir = root().join("design/specimen");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            let name = p
                .file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned();
            let src = std::fs::read_to_string(&p).expect("readable");
            let (mut game, _) = parse(&src, &name);
            let _ = resolve(&mut game);
            (name, src, game)
        })
        .collect()
}

/// What an author wrote, split the way the reader splits it.
///
/// The annotation rule is the lexer's: `??` runs to end of line unless it falls inside quotes.
/// Reproduced here rather than shared, because a census that borrows the reader's own splitting
/// cannot report that the reader splits it wrongly.
struct Authored {
    /// characters on lines the reader looks at — not blank, not `#`, annotation removed
    chars: usize,
    /// `(line, keyword, text)` for every quoted string, in file order
    quoted: Vec<(usize, String, String)>,
}

fn authored(src: &str) -> Authored {
    let (mut chars, mut quoted) = (0, Vec::new());
    for (i, raw) in src.lines().enumerate() {
        if raw.trim().is_empty() || raw.trim().starts_with('#') {
            continue;
        }
        let cs: Vec<char> = raw.chars().collect();
        let (mut cut, mut quotes) = (cs.len(), 0usize);
        for j in 0..cs.len() {
            if cs[j] == '"' {
                quotes += 1;
            }
            if cs[j] == '?' && j + 1 < cs.len() && cs[j + 1] == '?' && quotes % 2 == 0 {
                cut = j;
                break;
            }
        }
        let line: String = cs[..cut].iter().collect();
        let line = line.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        chars += line.chars().count();

        let Some(open) = line.find('"') else { continue };
        let Some(close) = line[open + 1..].rfind('"') else {
            continue;
        };
        let head = line[..open].trim();
        let keyword = head
            .split_whitespace()
            .next()
            .map(|k| k.trim_end_matches(':').to_string())
            .unwrap_or_else(|| "(bare)".to_string());
        quoted.push((i + 1, keyword, line[open + 1..open + 1 + close].to_string()));
    }
    Authored { chars, quoted }
}

/// The five spellings that carry a line the game says. Everything else carrying a quoted string
/// is a declaration, and the string is its label.
const SAYING: [&str; 5] = ["say", "otherwise", "when", "else", "(bare)"];

fn is_response(keyword: &str) -> bool {
    SAYING.contains(&keyword)
}

/// **Every quoted string an author wrote is dropped at the parse, and it is half the file.**
///
/// Measured rather than quoted, because this repository keeps finding figures that were true
/// when somebody typed them. The proposal says *"half the characters an author writes are
/// discarded at the parse"* and this is where that sentence is derived.
#[test]
fn every_quoted_string_in_a_specimen_is_dropped_at_the_parse() {
    let (mut chars, mut resp, mut resp_n, mut labels, mut labels_n) = (0, 0, 0, 0, 0);
    for (_, src, _) in specimens() {
        let a = authored(&src);
        chars += a.chars;
        for (_, keyword, text) in &a.quoted {
            let n = text.chars().count() + 2; // the quotes are authored too
            if is_response(keyword) {
                resp += n;
                resp_n += 1;
            } else {
                labels += n;
                labels_n += 1;
            }
        }
    }
    assert_eq!(
        (chars, resp_n, resp, labels_n, labels),
        (13_346, 105, 5_141, 80, 1_855),
        "authored chars, response sites and chars, label sites and chars"
    );
    assert_eq!((resp + labels) * 100 / chars, 52, "percent discarded");

    // There is nowhere for any of it to go. Asserted against the model's own source, which is
    // how `declared.rs` states the same kind of absence — a missing field has no value to read.
    let model =
        std::fs::read_to_string(root().join("crates/game-format/src/lib.rs")).expect("readable");
    assert!(
        !model.contains("pub says:") && !model.contains("pub refusals:"),
        "a binding with nothing to say"
    );
    assert!(
        !model.contains("pub labels:"),
        "a game whose declarations have no labels"
    );
    let reader =
        std::fs::read_to_string(root().join("crates/game-format/src/parse.rs")).expect("readable");
    assert!(
        reader.contains(r#""say" | "otherwise" => {}"#),
        "and the two spellings that carry most of it, matched and dropped"
    );
}

/// **A binding a player can fire, and a model that cannot say a word about it.**
///
/// `use-with crowbar` on the lighthouse's crate is the shape the whole format is for: a
/// precondition, two effects, and a refusal. The author wrote a line for the firing and a line
/// for the refusal, and the parse tree holds the predicate that selects each of them and neither
/// line.
///
/// Checked against the binding's own `Debug`, because the claim is that the text is nowhere in
/// the model rather than absent from one field somebody thought to look at.
#[test]
fn a_binding_the_player_can_fire_is_a_transition_with_nothing_to_say() {
    let (_, src, game) = specimens()
        .into_iter()
        .find(|(n, ..)| n == "lighthouse")
        .expect("the lighthouse specimen");

    let b = game
        .bindings
        .iter()
        .find(|b| b.verb == "use-with" && b.object.as_deref() == Some("crowbar"))
        .expect("use-with crowbar on the crate");
    assert!(b.when.is_some(), "it has a precondition");
    assert_eq!(b.effects.len(), 2, "and two effects");
    assert!(
        b.refuses,
        "and the bit that says it declines rather than hides"
    );

    let written: Vec<String> = authored(&src)
        .quoted
        .into_iter()
        .filter(|(no, k, _)| is_response(k) && *no > b.line && *no < b.line + 7)
        .map(|(_, _, t)| t)
        .collect();
    assert_eq!(
        written.len(),
        2,
        "the author wrote two lines for it: {written:?}"
    );
    assert!(written[0].starts_with("The lid comes away"), "{written:?}");
    assert!(written[1].starts_with("It is open already"), "{written:?}");

    let model = format!("{b:?}");
    for line in &written {
        assert!(
            !model.contains(line.as_str()),
            "the model holds none of it, and this one is in it: {line}"
        );
    }

    // And the crate's own name is gone the same way.
    assert!(
        game.declared.contains_key("quay.crate"),
        "the hotspot is declared"
    );
    assert!(
        !format!("{:?}", game.declared).contains("a nailed-shut crate"),
        "and what it is called is not"
    );
}

/// **The trailing quoted string is three different things and only the declaration's kind says
/// which.**
///
/// On a `room`, `actor`, `item` or `hotspot` it is a name a player reads. On an `option` it is a
/// line the *player says*, which is player-facing text of the same sort as a response. On a
/// `flag` or a `counter` it is a gloss for whoever reads the file — `"how far the lamp is from
/// lit: 0 bare, 1 wicked, 2 oiled, 3 burning"` is not addressed to a player and cannot be shown
/// to one.
///
/// One construct, three jobs, and nothing in the surface distinguishes them. Held here because a
/// reader that starts keeping the string has to know which of the three it kept.
#[test]
fn the_trailing_quoted_string_is_three_different_things() {
    let mut by_keyword: std::collections::BTreeMap<String, usize> = Default::default();
    for (_, src, _) in specimens() {
        for (_, keyword, _) in authored(&src).quoted {
            if !is_response(&keyword) {
                *by_keyword.entry(keyword).or_default() += 1;
            }
        }
    }
    let seen: Vec<(&str, usize)> = by_keyword.iter().map(|(k, n)| (k.as_str(), *n)).collect();
    assert_eq!(
        seen,
        vec![
            ("actor", 4),
            ("counter", 1),
            ("flag", 9),
            ("hotspot", 38),
            ("item", 11),
            ("option", 3),
            ("room", 14),
        ],
        "declarations carrying a trailing string"
    );
}
