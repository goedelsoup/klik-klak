//! **Everything an author writes reaches the parse tree, and the state model still ignores half
//! of it.**
//!
//! For six phases `parse.rs` matched `"say" | "otherwise" => {}`, a `when <pred> "<line>"` kept
//! the predicate and dropped the line beside it, and a declaration's trailing quoted string was
//! never looked at. **52% of a specimen was discarded at the parse** — 6,994 of 13,344 characters
//! the reader looks at, across the four. (The census that opened this phase counted two more of
//! each: `ferry.game` escaped a quotation the format needs no escape for, and nothing could tell
//! until the text was kept. See below.)
//!
//! `.yidam/decisions/effect-list-semantics.yml` stated that as a position: *"`say` is
//! presentation and not an effect… `Binding` has no field for response text. That is now the
//! stated position rather than an omission in a parser."* That is two claims. The first is about
//! the **state model** — a line writes no state, so it is not an edge, so the search need not see
//! it — and it is right, load-bearing, and untouched. The second is about the **reader**, and it
//! was the consequence of there being one thing downstream rather than an argument for anything.
//!
//! `.yidam/decisions/authored-text.yml` separates them. The tree keeps the text; the search goes
//! on not reading it. **The evidence that the position held is that not one number this project
//! publishes moved** — 91 bindings, 103 of 12,288 reachable, eleven items, four reaches, fifty
//! constructs, all pinned by tests written before this phase and all still green.

use game_format::{parse, resolve, Game, Kind};
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
/// The annotation rule is the lexer's — `??` runs to end of line unless it falls inside quotes —
/// and the quoting rule is too: **a line's text runs from its first quote to its last**, which is
/// why an embedded quotation needs no escape and why a line cannot end with one.
///
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

/// Every string the parse tree holds, in no particular order.
fn kept(game: &Game) -> Vec<String> {
    let mut out: Vec<String> = game.labels.values().cloned().collect();
    for b in &game.bindings {
        out.extend(b.says.iter().chain(&b.refusals).map(|r| r.text.clone()));
    }
    out.sort();
    out
}

/// **Every quoted string an author wrote is in the tree, and it is half the file.**
///
/// Compared as multisets rather than by membership, so a line kept once and written twice is a
/// failure. Measured rather than quoted: the proposal's sentence *"half the characters an author
/// writes"* is derived here.
///
/// **It found one on its first run.** `ferry.game` wrote `else "\"Three,\" he says…"`, and the
/// reader has no escape processing — it takes a line from its first quote to its last — so the
/// backslashes were two characters of the response. Nothing could have noticed while the response
/// was thrown away. The specimen is corrected rather than the lexer, because the first-to-last
/// rule already admits an embedded quotation and an escape would be a fifty-first construct.
#[test]
fn every_quoted_string_in_a_specimen_reaches_the_parse_tree() {
    let (mut chars, mut resp, mut resp_n, mut labels, mut labels_n) = (0, 0, 0, 0, 0);
    for (name, src, game) in specimens() {
        let a = authored(&src);
        chars += a.chars;
        let mut written: Vec<String> = Vec::new();
        for (_, keyword, text) in &a.quoted {
            let n = text.chars().count() + 2; // the quotes are authored too
            if is_response(keyword) {
                resp += n;
                resp_n += 1;
            } else {
                labels += n;
                labels_n += 1;
            }
            written.push(text.clone());
        }
        written.sort();
        assert_eq!(written, kept(&game), "{name} — written against kept");
    }
    assert_eq!(
        (chars, resp_n, resp, labels_n, labels),
        (19_968, 154, 7_996, 103, 2_671),
        "authored chars, response sites and chars, label sites and chars"
    );
    assert_eq!((resp + labels) * 100 / chars, 53, "percent of the file");
}

/// **The search reads none of it, the components read all of it, and the seam is checked rather
/// than asserted.**
///
/// The position in `effect-list-semantics` is that presentation is separable from state. The cheap
/// way to lose it is for something in the checker to start reading a line — a refusal changing a
/// count, a label reaching a predicate — so the seam is held against source.
///
/// It got sharper when the evaluator moved. There are three readers now and each takes a different
/// half: `crates/reachability` searches the state graph and touches no text; `engine/verb-menu`
/// reads the **labels**, because a menu has to say what it is pointing at;
/// `engine/interaction-resolver` reads the **lines**, because saying them is what it is for.
/// `.yidam/decisions/two-components.yml`.
///
/// A name check, saying nothing about behaviour — the same limit `engine/README.md` states about
/// its own gate. The behavioural evidence is the rest of the suite: 158 fixtures in
/// `crates/reachability/tests` passed unchanged through the phase that filled these fields, and
/// again through the phase that moved the functions reading them into another workspace.
///
/// **The seam was `.says` for three phases and is `.text` now, which is narrower and is the rule
/// that was meant.** `.yidam/decisions/orphaned-lines.yml` gave the search a reason to hold a
/// response for the first time: a clause no reachable state selects is authored text nobody can
/// read, and naming which clause needs the clause. What it takes off it is `.line`. **The search
/// counts lines and reads none**, which is the sentence `two-components` drew the seam for — the
/// earlier spelling forbade the word rather than the act.
#[test]
fn the_search_reads_none_of_it_and_the_components_read_it() {
    let read = |rel: &str| std::fs::read_to_string(root().join(rel)).expect("readable");

    let search = read("crates/reachability/src/lib.rs");
    for reading in [".text", ".labels", "as_str()", "chars()"] {
        assert!(
            !search.contains(reading),
            "the reachability search reached for {reading}, which is the seam this holds"
        );
    }

    // **Two exceptions, and both are the seam drawn more finely rather than crossed.** The search
    // reads `b.refusals` to report a binding that is shown, declines, and has no line for the
    // state it declined in (`.yidam/decisions/playable.yml`), and it reads both lists to report a
    // clause no reachable state selects (`.yidam/decisions/orphaned-lines.yml`). Neither looks at
    // a character. One asks whether a line exists and the other asks where it was written, and
    // both answers are about the graph rather than about the writing.
    assert_eq!(
        search.matches("&b.refusals").count(),
        3,
        "the search asks three times whether a refusal has a line, and reads no line"
    );
    assert_eq!(
        search.matches("r.line").count(),
        5,
        "and five times where one was written"
    );
    assert!(
        search.contains("pub silent: Vec<String>,")
            && search.contains("pub orphaned: Vec<String>,"),
        "and what it does with both answers is report them"
    );
    assert!(
        search.contains("b.effects") && search.contains("resolver::apply("),
        "and it still reads the half that is state, through the component that owns it"
    );

    let menu = read("engine/verb-menu/src/lib.rs");
    assert!(
        menu.contains("w.game.label(target)")
            && !menu.contains(".says")
            && !menu.contains(".refusals"),
        "the menu reads what a target is called and none of what it says"
    );

    let resolver = read("engine/interaction-resolver/src/lib.rs");
    assert!(
        resolver.contains("&b.says") && resolver.contains("&b.refusals"),
        "and the resolver reads both response lists"
    );
    assert!(
        !resolver.contains(".labels"),
        "and not the labels, which are the menu's half"
    );
}

/// **A binding the player can fire now carries what it says and what it refuses.**
///
/// `use-with crowbar` on the lighthouse's crate is the shape the whole format is for: a
/// precondition, two effects, and a refusal. Checked against the binding's own `Debug`, because
/// the claim is that the text is *in the model* rather than in one field somebody thought to
/// look at.
#[test]
fn a_binding_the_player_can_fire_carries_what_it_says_and_what_it_refuses() {
    let (_, _, game) = specimens()
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

    assert_eq!(b.says.len(), 1, "one line when it fires");
    assert!(b.says[0].when.is_none(), "a `say` selects on nothing");
    assert!(b.says[0].text.starts_with("The lid comes away"));
    assert_eq!(b.refusals.len(), 1, "one when it declines");
    assert!(b.refusals[0].text.starts_with("It is open already"));

    let model = format!("{b:?}");
    for line in [&b.says[0].text, &b.refusals[0].text] {
        assert!(model.contains(line.as_str()), "and both are in the model");
    }

    // The hotspot is called something now, and the crate's `look-at` is the guarded response
    // list — a predicate and the line it selects, which were two constructs in one place until
    // the line was kept.
    assert_eq!(game.label("quay.crate"), Some("a nailed-shut crate"));
    let look = game
        .bindings
        .iter()
        .find(|b| b.verb == "look-at" && matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "quay.crate"))
        .expect("look-at on the crate");
    assert_eq!(look.says.len(), 2, "a guarded line and a fallback");
    assert!(look.says[0].when.is_some(), "`when crate-open \"…\"`");
    assert!(
        look.says[1].when.is_none(),
        "`otherwise \"…\"`, written last"
    );
    assert_eq!(look.guards().count(), 1, "one predicate selects a line");
}

/// **The trailing quoted string is three different things and `Kind` is what says which.**
///
/// On a `room`, `actor`, `item` or `hotspot` it is a name a player reads. On an `option` it is a
/// line the *player says*, which is player-facing text of the same sort as a response. On a
/// `flag` or a `counter` it is a gloss for whoever reads the file — `"how far the lamp is from
/// lit: 0 bare, 1 wicked, 2 oiled, 3 burning"` is not addressed to a player and cannot be shown
/// to one.
///
/// One construct, three jobs. The map holds the string as written and the declaration's kind is
/// the discriminator, because a reader that starts keeping a string has to know which of the
/// three it kept.
#[test]
fn the_trailing_quoted_string_is_three_different_things() {
    let mut by_keyword: std::collections::BTreeMap<String, usize> = Default::default();
    let mut by_kind: std::collections::BTreeMap<Kind, usize> = Default::default();
    for (_, src, game) in specimens() {
        for (_, keyword, _) in authored(&src).quoted {
            if !is_response(&keyword) {
                *by_keyword.entry(keyword).or_default() += 1;
            }
        }
        for name in game.labels.keys() {
            let (kind, _) = game
                .declared
                .get(name)
                .expect("a label names a declaration");
            *by_kind.entry(*kind).or_default() += 1;
        }
    }
    let seen: Vec<(&str, usize)> = by_keyword.iter().map(|(k, n)| (k.as_str(), *n)).collect();
    assert_eq!(
        seen,
        vec![
            ("actor", 6),
            ("counter", 1),
            ("flag", 14),
            ("hotspot", 50),
            ("item", 12),
            ("option", 3),
            ("room", 17),
        ],
        "declarations carrying a trailing string"
    );
    assert_eq!(
        by_kind.values().sum::<usize>(),
        seen.iter().map(|(_, n)| n).sum::<usize>(),
        "and the tree holds one for each, keyed by what the admission rule keys on"
    );

    let author_facing: usize = by_kind
        .iter()
        .filter(|(k, _)| matches!(k, Kind::Flag | Kind::Counter))
        .map(|(_, n)| n)
        .sum();
    assert_eq!(
        (
            author_facing,
            by_kind.values().sum::<usize>() - author_facing
        ),
        (15, 88),
        "glosses no player can be shown, against text a player reads"
    );
}
