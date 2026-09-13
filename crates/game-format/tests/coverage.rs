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

/// **Every construct in the surface has an author, for the first time since there was a surface.**
///
/// The evidence that this vocabulary is the right size is six games. A construct with no author
/// behind it is not thereby wrong — `exit` and `or` were both admitted by argument rather than by
/// demand — but a language proposed as the smallest that can express the form should be able to
/// say which of its parts have carried weight and which are there on reasoning alone. Until now
/// the honest answer was *five of fifty are there on reasoning alone*.
///
/// They came off in three batches and none of them by being argued for again.
///
/// - **`clear-flag`** was on this list for eleven phases and `lock.game` took it off. Four games
///   wrote 105 response clauses and never put a flag back, because every flag in them is a thing
///   that happens to a world and worlds do not un-happen. The fifth has two characters and *which
///   one* is the only fact in any specimen that goes both ways.
///   `.yidam/decisions/control-transfer.yml`.
/// - **`or`, `exit`, `!=`, `<=` and `>`** came off together, in one game written to need the
///   first of them. `.yidam/decisions/or-earns-it.yml`.
///
/// **The last five are not five findings.** `exit` was written because six sites in the first
/// specimen wrote the longhand and a sixth author reaching for a room connection now meets the
/// declaration instead — it is sugar and it took an author who arrived after it. The three
/// comparisons are one finding: an ordered comparison is only distinguishable from `=` when a
/// counter has values **between** its bounds, and a counter gets values only from a literal in an
/// effect, so a progression counter is the whole of the demand for them. `relay.game`'s aerial has
/// four and reads it at four widths.
///
/// `or` is the one that was worth a game. It has its own test below.
///
/// **The thin end barely moved and that is the other half of the result.** Fifteen constructs were
/// written twice or fewer before this game and fourteen are now — a sixth of the corpus's authored
/// lines arrived and the tail lost one member. What a new game does is empty the *zero* column;
/// what it does not do is thicken the rest, because an author writing a game reaches for the same
/// dozen constructs the last five did. The head and the tail are a property of the form rather
/// than of how much has been written in it.
#[test]
fn every_construct_in_the_surface_has_an_author() {
    let n = written();
    let never: Vec<&String> = n.iter().filter(|(_, c)| **c == 0).map(|(k, _)| k).collect();
    assert!(
        never.is_empty(),
        "written by no specimen, where the whole surface once had five: {never:?}"
    );

    let by = |lo: usize, hi: usize| -> Vec<&String> {
        n.iter()
            .filter(|(_, c)| (lo..=hi).contains(*c))
            .map(|(k, _)| k)
            .collect()
    };
    assert_eq!(
        by(1, 1).len(),
        9,
        "written exactly once, in six games: {:?}",
        by(1, 1)
    );
    assert_eq!(
        by(0, 2).len(),
        14,
        "written twice or fewer, which is the thin end and no longer the empty end: {:?}",
        by(0, 2)
    );
}

/// **Composition is this language's one advantage over the table it was designed against, and
/// eleven of its twelve authored instances are in the one game with two characters.**
///
/// `data-format/interaction-table` compared the two item for item and landed it exactly: *"The gap
/// is real and it is exactly one thing: composition."* AGS had predicates over world state and no
/// way to combine them — no `or`, no grouping, nesting yielding conjunction only.
///
/// For eleven phases the measurement embarrassed the argument. Four specimens wrote the binary
/// connectives **once between them**: a single `and`, and no `or` at all. The note that stood here
/// said so — that this was an argument rather than a measurement, and that the node should say so.
///
/// **`lock.game` writes eleven more, and every one of them is about who the player is or where the
/// other character is standing.** `gates-shut and paddle-up and boatman in lockside and mate in
/// lockside`; `gates-open and not ashore`; `windlass-up and not paddle-up`. A game with one
/// protagonist asks about the world, and a world's facts are independent, so its guards are single
/// terms. A game with two asks about the world **and** about the pair, and the pair is what makes
/// a conjunction — which is a claim about the subject rather than about the language, and it is
/// the first evidence this corpus has that composition is demanded rather than argued.
/// `.yidam/decisions/control-transfer.yml`.
///
/// `or` is still unwritten in five games. `not` is unary and combines nothing.
#[test]
fn the_binary_connectives_are_written_twelve_times_and_eleven_are_one_game() {
    let n = written();
    assert_eq!(
        (n["and"], n["or"], n["not"]),
        (14, 2, 16),
        "conjunction, disjunction, and the unary that combines nothing"
    );
    let comparisons: usize = CMPS.iter().map(|c| n[*c]).sum();
    assert_eq!(comparisons, 21, "every comparison site in every specimen");
    assert_eq!(
        n["count"], 1,
        "and the quantifier a specimen was written to exercise"
    );
}
