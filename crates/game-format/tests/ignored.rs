//! **Two hundred and twenty-nine of three hundred and forty-nine lines accept a word the reader
//! never looks at, and eight keywords crash on a line with nothing after them.**
//!
//! [`parse.rs`](../src/parse.rs) opens by stating the opposite:
//!
//! > *"Anything it does not recognise is reported rather than ignored — a reader that silently
//! > skips what it does not understand would let a specimen claim coverage it does not have,
//! > which is the one failure this crate exists to prevent."*
//!
//! Nothing has ever checked it. `.yidam/decisions/verb-names.yml` found one violation by
//! accident — `conversation <name> with <actor>` read `t[1]` and never `t[2]`, so `beside keeper`
//! parsed identically — and repaired that one construct without asking how many others were the
//! same shape.
//!
//! This census asks, by mutation: put a junk token at the end of the bare-token part of every
//! line of every specimen, in turn, and see whether anything complains.
//!
//! **Twice, with two words, because the first answer is wrong in the reader's favour.** An
//! undeclared word is caught 76 times by the admission rule in `resolve` — `look-at XYZZY` fails
//! because nothing declares `XYZZY`, not because anything noticed a unary verb had been handed an
//! object. Mutating with a **declared** name takes the admission rule out of the answer and
//! leaves the reader on its own, and `look-at crowbar:` is then accepted: a unary verb silently
//! becomes a binding the menu will only show while the player is carrying a crowbar.

use game_format::{parse, resolve};

/// Each specimen, and one name it declares — the second mutation word.
const ALL: [(&str, &str, &str); 4] = [
    (
        "ferry",
        "coin",
        include_str!("../../../design/specimen/ferry.game"),
    ),
    (
        "lighthouse",
        "crowbar",
        include_str!("../../../design/specimen/lighthouse.game"),
    ),
    (
        "nightwatch",
        "ledger",
        include_str!("../../../design/specimen/nightwatch.game"),
    ),
    (
        "patrol",
        "key",
        include_str!("../../../design/specimen/patrol.game"),
    ),
];

/// Does the reader say anything at all about this source — at parse or at admission?
fn complains(src: &str, name: &str) -> bool {
    let (mut game, problems) = parse(src, name);
    !problems.is_empty() || !resolve(&mut game).is_empty()
}

/// The same line with a junk token appended to its bare-token part.
///
/// Before the first quote, so the mutation lands in the grammar rather than inside a response;
/// before `??`, so it is not eaten as an annotation. `None` where there is nothing to mutate.
fn junk(line: &str, word: &str) -> Option<String> {
    let body = line.split("??").next().unwrap_or(line);
    let head = match body.find('"') {
        Some(i) => &body[..i],
        None => body,
    };
    if head.trim().is_empty() || head.trim().starts_with('#') {
        return None;
    }
    let cut = head.trim_end().len();
    Some(format!("{} {word}{}", &line[..cut], &line[cut..]))
}

fn keyword(line: &str) -> String {
    line.trim()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(':')
        .to_string()
}

/// Every construct, and how many of its lines accept a word the reader never looks at.
///
/// `declared` chooses the mutation word: a name the specimen declares, which `resolve` will
/// admit, or `XYZZY`, which it will not.
fn census(declared: bool) -> (Vec<(String, usize, usize)>, usize, usize) {
    let mut silent: std::collections::BTreeMap<String, usize> = Default::default();
    let mut seen: std::collections::BTreeMap<String, usize> = Default::default();
    let (mut quiet, mut total) = (0, 0);
    for (name, item, src) in ALL {
        assert!(!complains(src, name), "{name} does not read clean as it is");
        let word = if declared { item } else { "XYZZY" };
        let lines: Vec<&str> = src.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let Some(mutant) = junk(line, word) else {
                continue;
            };
            total += 1;
            let k = keyword(line);
            *seen.entry(k.clone()).or_insert(0) += 1;
            let mut copy = lines.clone();
            copy[i] = &mutant;
            if !complains(&copy.join("\n"), name) {
                quiet += 1;
                *silent.entry(k).or_insert(0) += 1;
            }
        }
    }
    let rows = seen
        .into_iter()
        .map(|(k, n)| {
            let s = silent.get(&k).copied().unwrap_or(0);
            (k, s, n)
        })
        .collect();
    (rows, quiet, total)
}

/// **Only the bounded expression language accounts for its tokens. Nothing else in the format
/// does.**
///
/// `when`, `do` and `else` hand the whole token slice to a sub-grammar that must consume all of
/// it, and they are the three. Every other construct reads `t[1]`, sometimes `t.get(2)`, and
/// drops the tail without looking — 229 lines of 349.
///
/// `design/bounded-expression-language.md` is named after the half that works. The other half has
/// no grammar at all: it is fifteen hand-written index expressions, and the one arity check in the
/// file sits on one of them.
///
/// **`room` is the construct on both sides of the line and shows the cause bare.** `room <name>
/// "<label>"` is a declaration and asserts `t.len() != 2`; `room <name>` opens a section and
/// asserts nothing. Same keyword, same file, fourteen lines each way.
#[test]
fn only_the_expression_grammar_accounts_for_what_is_on_the_line() {
    let (rows, quiet, total) = census(true);
    assert_eq!(
        (quiet, total),
        (229, 349),
        "lines accepting a declared word the reader never looks at, of lines"
    );

    let strict: Vec<&str> = rows
        .iter()
        .filter(|(_, s, _)| *s == 0)
        .map(|(k, _, _)| k.as_str())
        .collect();
    assert_eq!(strict, ["do", "else", "when"], "constructs with a grammar");

    let covered: usize = rows
        .iter()
        .filter(|(_, s, _)| *s == 0)
        .map(|(_, _, n)| n)
        .sum();
    assert_eq!(covered, 106, "lines those three cover");

    // every other construct is silent on every line of it, `room` excepted
    for (k, s, n) in &rows {
        if *s == 0 {
            continue;
        }
        if k == "room" {
            assert_eq!(
                (*s, *n),
                (14, 28),
                "the section form and the declaration form"
            );
            continue;
        }
        assert_eq!(
            s, n,
            "`{k}` is silent on {s} of {n} lines, which is not all of them"
        );
    }
}

/// **Seventy-six of the lines that look strict are the admission rule doing the reader's job.**
///
/// Mutate with `XYZZY` instead and the count falls to 153, because `resolve` rejects an operand
/// nothing declares. That is a real check and it is not this one: it says *that name means
/// nothing*, never *this construct does not take a name there*. The four verbs it appears to make
/// strict — `look-at`, `pick-up`, `talk-to`, `walk-to` — take no operand at all, and hand one a
/// **declared** name and the reader takes it.
///
/// What it takes it as is the part that matters. A unary verb given an object becomes a binding
/// with an object, and `verb_menu::live` only shows such a binding while the player is carrying
/// the thing — so `look-at crowbar:` is a description that appears and disappears, written by a
/// typo, with nothing anywhere reporting it.
#[test]
fn the_admission_rule_is_what_makes_a_unary_verb_look_strict() {
    let (_, loud, total) = census(false);
    assert_eq!((loud, total), (153, 349), "with a word nothing declares");
    let (_, quiet, _) = census(true);
    assert_eq!(
        quiet - loud,
        76,
        "lines `resolve` catches and `parse` does not"
    );

    // and what the reader makes of a unary verb handed a declared name
    let source = ALL[1].2.replace(
        "  hotspot lamp \"the great lamp\"\n    look-at:",
        "  hotspot lamp \"the great lamp\"\n    look-at crowbar:",
    );
    assert!(source != ALL[1].2, "the mutation applied");
    assert!(!complains(&source, "lighthouse"), "and nothing complains");
    let (game, _) = parse(&source, "lighthouse");
    let bound = game
        .bindings
        .iter()
        .find(|b| b.verb == "look-at" && b.object.as_deref() == Some("crowbar"))
        .expect("a unary verb with an object on it");
    assert!(
        matches!(&bound.site, game_format::Site::InRoom { target, .. } if target == "lamp-room.lamp"),
        "on the hotspot it was written under"
    );
}

/// **Eight keywords panic on a line with nothing after them.**
///
/// The same defect from the other side: `t[1]` where `t.len() == 3` drops a token, and `t[1]`
/// where `t.len() == 1` is an index out of bounds. A reader whose stated purpose is to report
/// rather than ignore does neither — it aborts the process.
///
/// `is`, `holds` and `on-enter` are the three that say something, and all three say it because
/// somebody wrote the check by hand at that one site.
#[test]
fn eight_keywords_crash_on_a_line_with_nothing_after_them() {
    let cases: [(&str, &str); 11] = [
        ("room", "room"),
        ("item", "item"),
        ("flag", "flag"),
        ("counter", "counter"),
        ("conversation", "conversation"),
        ("actor", "room r\nactor"),
        ("hotspot", "room r\nhotspot"),
        ("option", "conversation c with a\noption"),
        ("is", "room r\nhotspot h\nis"),
        ("holds", "room r\nholds"),
        ("on-enter", "room r\non-enter"),
    ];
    let mut crashed = Vec::new();
    for (name, src) in cases {
        let src = src.to_string();
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let out = std::panic::catch_unwind(move || complains(&src, "t"));
        std::panic::set_hook(hook);
        if out.is_err() {
            crashed.push(name);
        }
    }
    assert_eq!(
        crashed,
        [
            "room",
            "item",
            "flag",
            "counter",
            "conversation",
            "actor",
            "hotspot",
            "option"
        ],
        "keywords that panic rather than report"
    );
}

/// **The file says it does the opposite, in its first paragraph.**
///
/// Pinned because it is the claim the census is read against, and because it is the kind of
/// sentence that stays true-sounding for as long as nobody tests it.
#[test]
fn the_reader_states_the_rule_it_does_not_keep() {
    let src = include_str!("../src/parse.rs");
    let flat = src
        .lines()
        .map(|l| l.trim().trim_start_matches("//!").trim())
        .collect::<Vec<_>>()
        .join(" ");
    let flat = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains(
            "Anything it does not recognise is reported rather than ignored — a reader that \
             silently skips what it does not understand would let a specimen claim coverage it \
             does not have, which is the one failure this crate exists to prevent."
        ),
        "the rule"
    );
    // and the one place it is kept by an explicit check
    assert!(
        src.contains("if t.len() != 2 {")
            && src.contains("a room declaration is `room <name> \\\"<label>\\\"`"),
        "the one arity check in the file"
    );
}
