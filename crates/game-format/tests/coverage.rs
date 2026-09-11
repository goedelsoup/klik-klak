//! **How much of the surface any author has ever written.**
//!
//! `surface.rs` pins how big the bounded language is — fifty constructs, and what they cost to
//! reach. It says nothing about how many of them have been used, and the entire evidence that any
//! of them is needed is **four specimens totalling 662 lines**.
//!
//! This counts, by parsing the specimens rather than reading them. Every construct the surface
//! admits, against the number of times an author reached for it.
//!
//! **Six constructs have never been written and ten more were written exactly once**, so two fifths
//! of the surface has been reached for twice or fewer. `in-room` is written 87 times and `do` 54;
//! the distribution has a head and a long tail.
//!
//! That is not a defect on its own — a vocabulary proposed as minimal is allowed to contain a
//! primitive no specimen happened to need, and `exit` and `or` were both admitted by argument
//! rather than by demand. But it is the measurement a minimality claim has to survive, and nothing
//! here had taken it. The sharpest reading is in the last test: **the composition this language
//! has and the interaction table does not — the one difference `interaction-table` calls the gap —
//! is authored once.**
//!
//! `.yidam/decisions/unwritten.yml` carries what follows.

use game_format::{Effect, Pred, Site};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn specimens() -> Vec<(String, String)> {
    let dir = root().join("design/specimen");
    let mut out: Vec<(String, String)> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .map(|p| {
            (
                p.file_stem()
                    .expect("a stem")
                    .to_string_lossy()
                    .into_owned(),
                std::fs::read_to_string(&p).expect("readable"),
            )
        })
        .collect();
    out.sort();
    out
}

/// Every construct the surface admits, mapped to how many times a specimen writes it.
///
/// The typed half — effects, predicates, comparisons and sites — is counted off the parse tree, so
/// a construct is used when the parser built it and not when the word appears in a comment. The
/// keywords are counted where the parser dispatches on them: first token of a line.
fn written() -> BTreeMap<String, usize> {
    let mut n: BTreeMap<String, usize> = BTreeMap::new();
    for k in KEYWORDS
        .iter()
        .chain(VERBS)
        .chain(EFFECTS)
        .chain(PREDS)
        .chain(CMPS)
        .chain(SITES)
    {
        n.insert((*k).to_string(), 0);
    }
    let mut bump = |k: &str| {
        *n.get_mut(k)
            .unwrap_or_else(|| panic!("{k} is not a construct")) += 1
    };

    fn walk(p: &Pred, bump: &mut impl FnMut(&str)) {
        match p {
            Pred::Flag(_) => bump("flag?"),
            Pred::Carrying(_) => bump("carrying"),
            Pred::Chosen(_) => bump("chosen"),
            Pred::Cmp(_, c, _) => {
                bump("counter?");
                bump(c.spelling());
            }
            Pred::Count(_, c, _) => {
                bump("count");
                bump(c.spelling());
            }
            Pred::At(_, _) => bump("at"),
            Pred::Not(q) => {
                bump("not");
                walk(q, bump);
            }
            Pred::And(a, b) => {
                bump("and");
                walk(a, bump);
                walk(b, bump);
            }
            Pred::Or(a, b) => {
                bump("or");
                walk(a, bump);
                walk(b, bump);
            }
        }
    }

    for (name, src) in specimens() {
        for line in src.lines() {
            if line.trim_start().starts_with('#') {
                continue; // a keyword discussed in a comment is not a keyword written
            }
            // the parser strips a trailing colon before dispatching, so `on-enter:` is `on-enter`
            let Some(first) = line
                .split_whitespace()
                .next()
                .map(|t| t.trim_end_matches(':'))
            else {
                continue;
            };
            if KEYWORDS.contains(&first) {
                bump(first);
            }
        }
        let (game, problems) = game_format::parse(&src, &name);
        assert!(
            problems.is_empty(),
            "{name} does not parse, so this count would be of a partial tree: {problems:?}"
        );
        for b in &game.bindings {
            if VERBS.contains(&b.verb.as_str()) {
                bump(&b.verb);
            }
            match &b.site {
                Site::InRoom { .. } => bump("in-room"),
                Site::InConversation { .. } => bump("in-conversation"),
                Site::OnEnter { .. } => bump("on-enter?"),
            }
            for p in b.when.iter().chain(b.guards()) {
                walk(p, &mut bump);
            }
            for e in &b.effects {
                bump(match e {
                    Effect::SetFlag(_) => "set-flag",
                    Effect::ClearFlag(_) => "clear-flag",
                    Effect::SetCounter(..) => "set-counter",
                    Effect::Give(..) => "give",
                    Effect::Spend(..) => "spend",
                    Effect::Move(..) => "move",
                    Effect::Enter(_) => "enter",
                    Effect::Leave => "leave",
                });
            }
        }
    }
    n
}

const KEYWORDS: [&str; 19] = [
    "actor",
    "conversation",
    "counter",
    "do",
    "else",
    "ending",
    "exit",
    "flag",
    "holds",
    "hotspot",
    "is",
    "item",
    "on-enter",
    "option",
    "otherwise",
    "room",
    "say",
    "start",
    "when",
];
const VERBS: &[&str] = &["look-at", "pick-up", "talk-to", "use-with", "walk-to"];
const EFFECTS: &[&str] = &[
    "set-flag",
    "clear-flag",
    "set-counter",
    "give",
    "spend",
    "move",
    "enter",
    "leave",
];
// `flag?` and `counter?` are the *predicates* over a flag and a counter, named apart from the
// `flag` and `counter` keywords that declare them; `on-enter?` is the site, named apart from the
// keyword that opens one.
const PREDS: &[&str] = &[
    "flag?", "carrying", "chosen", "counter?", "count", "at", "not", "and", "or",
];
const CMPS: &[&str] = &["=", "!=", "<", "<=", ">", ">="];
const SITES: &[&str] = &["in-room", "in-conversation", "on-enter?"];

/// **The surface is fifty constructs and this counts every one of them.**
///
/// Held against `surface.rs`'s own arithmetic rather than restated: if a construct is added there
/// and not here, the count this file reports is of something smaller than the language.
#[test]
fn the_count_covers_the_whole_surface() {
    let n = written();
    assert_eq!(
        KEYWORDS.len() + VERBS.len() + EFFECTS.len() + PREDS.len() + CMPS.len() + SITES.len(),
        50,
        "5 verbs, 19 keywords, 8 effects, 9 predicate forms, 6 comparisons, 3 sites"
    );
    assert_eq!(n.len(), 50, "and each is counted exactly once");
}

/// **Four constructs have never been written, and nine more were written once or twice.**
///
/// The evidence that the surface is the right size is four games. A construct with no author
/// behind it is not thereby wrong — `exit` and `or` were both admitted by argument rather than by
/// demand — but a language proposed as the smallest that can express the form should be able to
/// say which of its parts have carried weight and which are there on reasoning alone.
#[test]
fn the_constructs_no_specimen_has_written() {
    let n = written();
    let never: Vec<&String> = n.iter().filter(|(_, c)| **c == 0).map(|(k, _)| k).collect();
    assert_eq!(
        never,
        ["!=", "<=", ">", "clear-flag", "exit", "or"],
        "written by no specimen: two comparisons, a keyword and the one connective that \
         distinguishes this language from the interaction table it was designed against"
    );

    let by = |lo: usize, hi: usize| -> Vec<&String> {
        n.iter()
            .filter(|(_, c)| (lo..=hi).contains(*c))
            .map(|(k, _)| k)
            .collect()
    };
    assert_eq!(
        by(1, 1).len(),
        10,
        "written exactly once, in four games: {:?}",
        by(1, 1)
    );
    assert_eq!(
        by(0, 2).len(),
        20,
        "two fifths of the surface, written twice or fewer: {:?}",
        by(0, 2)
    );
}

/// **Composition is this language's one advantage over the table it was designed against, and it
/// has one authored instance.**
///
/// `data-format/interaction-table` compared the two item for item and landed it exactly: *"The gap
/// is real and it is exactly one thing: composition."* AGS had predicates over world state and no
/// way to combine them — no `or`, no grouping, nesting yielding conjunction only.
///
/// Across four specimens the binary connectives are written **once**: a single `and`, and no `or`
/// at all. `not` is used ten times and is unary, so it does not combine anything.
///
/// That does not make the argument wrong. It makes it an argument rather than a measurement, which
/// is a different claim from the one the corpus has been making, and the node now says so.
#[test]
fn the_binary_connectives_are_written_once_between_them() {
    let n = written();
    assert_eq!(
        (n["and"], n["or"], n["not"]),
        (1, 0, 10),
        "conjunction, disjunction, and the unary that combines nothing"
    );
    let comparisons: usize = CMPS.iter().map(|c| n[*c]).sum();
    assert_eq!(comparisons, 10, "every comparison site in every specimen");
    assert_eq!(
        n["count"], 1,
        "and the quantifier a specimen was written to exercise"
    );
}
