//! **Every construct accounts for every token on its line, and no keyword crashes on a bare
//! one.**
//!
//! It was 229 of 349 lines accepting a word the reader never looked at, and eight keywords
//! aborting the process on a line with nothing after them.
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
//! The check is by mutation: put a junk token at the end of the bare-token part of every line of
//! every specimen, in turn, and see whether anything complains.
//!
//! **Twice, with two words, because one answer flatters the reader.** An undeclared word was
//! caught 76 times by the admission rule in `resolve` — `look-at XYZZY` failed because nothing
//! declares `XYZZY`, never because anything noticed a unary verb had been handed an object.
//! Mutating with a **declared** name takes the admission rule out of the answer and leaves the
//! reader on its own. Both runs are kept, because a check that holds for the wrong reason holds
//! right up until the reason moves.

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
    line.split_whitespace()
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

/// **Nothing on a line goes unread, in either mutation.**
///
/// The rule `parse.rs` states in its first paragraph, held against 698 mutants. Both words,
/// because the two ask different questions: `XYZZY` asks whether anything notices an undeclared
/// name, and a declared name asks whether the **reader** notices a token the construct does not
/// take. The second is the one that was failing.
#[test]
fn nothing_on_a_line_goes_unread() {
    for declared in [false, true] {
        let (rows, quiet, total) = census(declared);
        assert_eq!(total, 349, "mutable lines in the four specimens");
        let loud: Vec<&(String, usize, usize)> = rows.iter().filter(|(_, s, _)| *s > 0).collect();
        assert!(
            quiet == 0,
            "declared={declared}: {quiet} line(s) accept a word the reader never looks at: {loud:?}"
        );
    }
}

/// **Sixteen constructs, each stating the form it reads — and none of them stating it here.**
///
/// This check used to be the specification. It pinned the six `const FORM` literals and the six
/// inline ones as a list, *"because the list is the format's declaration half written down — there
/// is no grammar document for it"*, and that was true when it was written.
///
/// There is one now. `game_format::FORMS` holds all twenty rows, the reader reads its error text
/// out of them, `design/bounded-expression-language.md` quotes them and
/// `crates/game-format/tests/forms.rs` holds the three together. So the list moved out of this file
/// and what is left here is the half that is about `parse.rs` rather than about the format: that
/// operands are read through the helper rather than by indexing, and that the indexing left over
/// cannot outrun its length check. `.yidam/decisions/line-format.yml`.
#[test]
fn every_construct_states_the_form_it_reads() {
    let src = include_str!("../src/parse.rs");

    // No form is written here any more: they live in `forms.rs` and the reader looks them up.
    assert!(
        !src.contains("const FORM: &str"),
        "a branch names its own form, which is a second copy of the specification"
    );
    for row in game_format::FORMS {
        if row.written != row.keyword {
            assert!(
                !src.contains(&format!("\"{}\"", row.written)),
                "the reader writes the form `{}` as a literal",
                row.written
            );
        }
    }

    // every construct that reads an operand goes through the helper rather than indexing
    assert_eq!(
        src.matches("p.at(no, &t,").count(),
        10,
        "operands read through the helper"
    );
    // What indexing is left, and every one of it is behind a length check it cannot outrun:
    // two inside the `room` declaration's `t.len() != 2`, one behind `t.len() > 2` in `item`,
    // and two in the effect parser, which has already matched an arity.
    let indexing = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .map(|l| l.matches("t[1]").count() + l.matches("t[2]").count() + l.matches("t[3]").count())
        .sum::<usize>();
    assert_eq!(
        indexing, 5,
        "an operand is read by indexing somewhere new, which is what crashed on a bare line"
    );
}

/// **No keyword crashes on a line with nothing after it.**
///
/// The other side of indexing: `t[1]` where the line has three tokens dropped the third, and
/// `t[1]` where it has one aborted the process. Eight did — `room`, `item`, `flag`, `counter`,
/// `conversation`, `actor`, `hotspot`, `option`. A reader whose stated purpose is to report
/// rather than ignore must do neither.
#[test]
fn no_keyword_crashes_on_a_line_with_nothing_after_it() {
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
    let mut silent = Vec::new();
    for (name, src) in cases {
        let owned = src.to_string();
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let out = std::panic::catch_unwind(move || complains(&owned, "t"));
        std::panic::set_hook(hook);
        match out {
            Err(_) => crashed.push(name),
            Ok(false) => silent.push(name),
            Ok(true) => {}
        }
    }
    assert!(crashed.is_empty(), "keywords that panic: {crashed:?}");
    assert!(
        silent.is_empty(),
        "keywords that accept a bare line: {silent:?}"
    );
}

/// **The specimens did not move, which is what makes this a reader change.**
///
/// All four read clean before and after. 50 constructs before, 50 after, and every figure four
/// phases of records quote is computed from the same parse trees — the evidence that nothing was
/// taken out of the format to make the reader strict.
#[test]
fn the_specimens_read_clean_and_the_surface_did_not_move() {
    for (name, _, src) in ALL {
        assert!(!complains(src, name), "{name} no longer reads clean");
    }
    let mut bindings = 0;
    let mut declared = 0;
    for (name, _, src) in ALL {
        let (game, _) = parse(src, name);
        bindings += game.bindings.len();
        declared += game.declared.len();
    }
    assert_eq!(
        (bindings, declared),
        (91, 81),
        "bindings and declared names across the four, unchanged by the reader getting strict"
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
    // and the one place it is kept by an explicit check. The message it raises used to be a
    // literal here and is read out of `forms.rs` now, so what this can still hold is the check
    // itself and the fact that its text comes from the table.
    assert!(
        src.contains("if t.len() != 2 {") && src.contains("a room declaration is `{}`"),
        "the one arity check in the file"
    );
}
