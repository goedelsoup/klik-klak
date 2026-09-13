//! **One conjunction is restated, four times, in one game — and the shipped game composes thirteen
//! times and never needs a name for one.**
//!
//! [`puzzle/state-gate`] asked at genesis whether the vocabulary should have a name for a condition
//! itself, and called it *"the strongest live case for adding a primitive rather than removing
//! one"*: **a gate depending on three flags is stated three times, or more, and nothing checks that
//! the statements agree.** `.yidam/decisions/named-condition.yml` declined on a census finding one
//! compound predicate in thirty-six, and `.yidam/decisions/unread-specimen.yml` found that census
//! had been reading four of five specimens. It is eight in sixty, and the question reopened.
//!
//! **The sentence is now instantiated and the evidence under it is one instance.** Exactly one
//! compound predicate in the corpus appears in more than one guard: `gates-shut and paddle-up`, in
//! four of `lock.game`'s guards. Every other compound in five games appears once.
//!
//! And what a name would buy is small on both measures this corpus uses:
//!
//! - **characters** — the conjunction is 24 characters written four times; a declaration plus four
//!   uses of a four-letter name is 79 against 96. **Seventeen characters across 904 lines.**
//!   `.yidam/decisions/frame-width.yml`'s criterion returns nothing at this size, and the figure
//!   came out smaller than the estimate that opened the phase, which had it at forty.
//! - **expressiveness** — nothing. The conjunction is already writable; a name is a second way to
//!   say what the language says. `exit` is the one construct here admitted purely as a convenience
//!   and it was admitted on **two independent bodies of authored work** — 20 exits in a shipped
//!   game, 18 bare, against 23 here, 17 bare (`.yidam/decisions/exit-archetype.yml`). This has one
//!   body and one instance.
//!
//! **The shipped game is the leg that was not counted.** Delores writes **141 guards, 79 of them
//! real expressions**, using thirteen composition operators between them — `&&` 7, `and` 3, `||` 3
//! — including a three-atom conjunction in a dialogue guard
//! (`.yidam/catalog/delores-source.md`). Dinky and Yack have **no named-condition construct**. A
//! person shipped a game with exactly this shape in it and never wanted a name for one.
//!
//! `.yidam/decisions/restated-guard.yml`.
//!
//! [`puzzle/state-gate`]: ../../../.yidam/corpus/puzzle/state-gate.yml

mod common;

use game_format::Pred;
use std::collections::BTreeMap;

/// Every compound sub-expression of a predicate, written canonically.
///
/// A conjunction nested inside a wider one counts: `gates-shut and paddle-up` is the same condition
/// whether it stands alone or carries `and boatman in chamber` after it, and a name would abbreviate
/// it in both places. That is the reading most favourable to the primitive, and it is the one taken.
fn compounds(p: &Pred, out: &mut Vec<String>) {
    match p {
        Pred::And(a, b) | Pred::Or(a, b) => {
            out.push(format!("{p:?}"));
            compounds(a, out);
            compounds(b, out);
        }
        Pred::Not(inner) => compounds(inner, out),
        _ => {}
    }
}

/// Every guard in every specimen: the binding's own, and the one on a guarded response clause.
fn guards() -> Vec<(String, Pred)> {
    let mut out = Vec::new();
    for (name, game) in common::games() {
        for b in &game.bindings {
            for p in b.when.iter().chain(
                b.says
                    .iter()
                    .chain(b.refusals.iter())
                    .filter_map(|r| r.when.as_ref()),
            ) {
                out.push((name.clone(), p.clone()));
            }
        }
    }
    out
}

/// **Exactly one condition is written at more than one site.**
///
/// The measurement `state-gate`'s sentence asks for, taken the way the sentence means it: a
/// condition is restated when the same compound appears in two guards, however it is spelled around.
///
/// `gates-shut and paddle-up` is in four of `lock.game`'s guards — twice alone, once carrying
/// `boatman in chamber`, and once inside the widest conjunction in the corpus. It is the thing the
/// game is about: the chamber is full and the boat can be moved. **Every other compound in five
/// games appears once**, including the one `lighthouse.game` writes.
///
/// So the shape `state-gate` predicted is here and the pressure it predicted is not — one author,
/// one game, one sitting, one condition worth naming.
///
/// **`named-condition` said what would reopen it and a game was written that could have.**
/// `.yidam/decisions/restated-guard.yml` declined the primitive a second time and set the terms:
/// *a second game that restates a conjunction, or a restatement that drifts.*
/// `design/specimen/relay.game` is a third composing game, written to need composition — and it
/// restates nothing. It is still one condition at more than one site, in `lock.game`, and the
/// count of games that compose has gone from two to three without the count of restatements
/// moving off one. `.yidam/decisions/or-earns-it.yml`.
///
/// **The reason is worth more than the count.** The one thing relay wanted to say twice —
/// *at the crosstrees, with light to work by* — is written once, in brackets, because
/// `atom := '(' pred ')'` turned out to have been in the grammar all along. A restatement is what
/// an author writes when the language cannot group; the pressure `named-condition` was weighed
/// against is partly a missing production rather than a missing primitive, and this game would
/// have supplied the second instance if the writer had gone on normalising it away.
#[test]
fn one_condition_in_six_games_is_written_at_more_than_one_site() {
    let mut sites: BTreeMap<String, usize> = BTreeMap::new();
    let mut per_game: BTreeMap<String, usize> = BTreeMap::new();
    for (game, p) in guards() {
        let mut here = Vec::new();
        compounds(&p, &mut here);
        here.sort();
        here.dedup();
        for c in here {
            *sites.entry(c).or_default() += 1;
            *per_game.entry(game.clone()).or_default() += 1;
        }
    }

    let restated: Vec<(&String, &usize)> = sites.iter().filter(|(_, n)| **n > 1).collect();
    assert_eq!(
        restated.len(),
        1,
        "conditions written at more than one site: {restated:#?}"
    );
    let (condition, count) = restated[0];
    assert_eq!(
        (condition.as_str(), *count),
        ("And(Flag(\"gates-shut\"), Flag(\"paddle-up\"))", 4),
        "the one restated condition, and how many guards hold it"
    );

    // and it is one game's: two others compose and neither repeats itself
    assert_eq!(
        per_game.get("lock").copied().unwrap_or(0),
        11,
        "compound sub-expressions in the game with a mechanism"
    );
    assert_eq!(
        per_game.get("lighthouse").copied().unwrap_or(0),
        1,
        "and in the only other game that composes at all"
    );
    assert_eq!(
        per_game.len(),
        3,
        "games that compose a condition: {per_game:?}"
    );
}

/// **A name would save seventeen characters, which is what `frame-width`'s criterion returns
/// nothing for.**
///
/// The conjunction is 24 characters. Written into four guards it is 96. A declaration naming it —
/// `condition full  gates-shut and paddle-up  "the chamber is full"` — plus four uses of `full` is
/// 79. The saving is **seventeen characters across 904 authored lines**, because the declaration
/// has to carry a label like everything else declared here and the label is most of it.
///
/// `.yidam/decisions/frame-width.yml` admitted an archetype by ranking constructs on what they cost
/// against what they carry, and `.yidam/decisions/unread-specimen.yml` found that ranking reversed
/// on a six per cent margin. A construct arguing from a forty-character saving is well inside the
/// noise that reversal exposed.
#[test]
fn naming_the_one_restated_condition_saves_seventeen_characters() {
    const CONDITION: &str = "gates-shut and paddle-up";
    let src = common::source("lock");
    let written = src.matches(CONDITION).count();
    assert_eq!(written, 4, "the conjunction, as an author typed it");

    let as_written = CONDITION.len() * written;
    // `condition full  gates-shut and paddle-up  "the chamber is full"` plus four uses of `full`
    let declared =
        "condition full  gates-shut and paddle-up  \"the chamber is full\"".len() + 4 * 4;
    assert_eq!(
        (as_written, declared, as_written - declared),
        (96, 79, 17),
        "characters as written, characters with a name, and what a name buys"
    );
    assert!(
        as_written - declared < 50,
        "the saving has grown past the noise `frame-width` measures in"
    );
}

/// **The widest guard in the corpus, which is the readability argument and the only one with
/// anything behind it.**
///
/// `lock.game:176` carries five atoms — two flags, two actor positions and a negation:
///
/// ```text
/// when gates-shut and paddle-up and boatman in lockside and mate in lockside and not gates-open
/// ```
///
/// **88 characters of condition, against a median guard of 13.** A name would make it roughly half
/// that, and that is a real gain for one line in five games.
///
/// It is not an argument for a declarable, because it is an argument about one line. The construct
/// that would fix it would exist for that line, and *a primitive with no site* is what this corpus
/// has refused four times — this one has a site, and one occasion.
#[test]
fn the_widest_guard_is_five_atoms_and_there_is_one_of_it() {
    fn atoms(p: &Pred) -> usize {
        match p {
            Pred::And(a, b) | Pred::Or(a, b) => atoms(a) + atoms(b),
            Pred::Not(inner) => atoms(inner),
            _ => 1,
        }
    }
    let all = guards();
    let widest = all.iter().map(|(_, p)| atoms(p)).max().expect("guards");
    assert_eq!(widest, 5, "atoms in the widest guard");
    assert_eq!(
        all.iter().filter(|(_, p)| atoms(p) == 5).count(),
        1,
        "and how many guards are that wide"
    );

    let src = common::source("lock");
    let line = src
        .lines()
        .find(|l| l.contains("boatman in lockside and mate in lockside"))
        .expect("the widest guard");
    let condition = line.trim().strip_prefix("when").expect("a guard").trim();
    assert_eq!(condition.chars().count(), 88, "characters of condition");

    // against the median, which is what makes this one line rather than a shape
    let mut widths: Vec<usize> = Vec::new();
    for (_, src) in common::sources() {
        for l in src.lines() {
            let t = l.trim();
            let Some(rest) = t
                .strip_prefix("else when ")
                .or_else(|| t.strip_prefix("when "))
            else {
                continue;
            };
            let c = rest.split('"').next().unwrap_or("").trim();
            if !c.is_empty() {
                widths.push(c.chars().count());
            }
        }
    }
    widths.sort_unstable();
    assert_eq!(
        (
            widths.len(),
            widths[widths.len() / 2],
            widths[widths.len() - 1]
        ),
        (76, 13, 88),
        "guards, the median width, and the widest"
    );
}

/// **A person shipped a game with this shape in it and never wanted a name for one.**
///
/// The leg that was not counted, and it was in the catalog the whole time. Delores writes **141
/// guards in square brackets**; 62 are bare modifier tags and **79 are real expressions**, and the
/// operators across them include **`&&` 7, `and` 3 and `||` 3** — thirteen compositions, one of them
/// a three-atom conjunction:
///
/// ```text
/// [YACK(knows_about_invicted) and !Bank.open and MainStreet.bankDoorSign.seen]
/// ```
///
/// **Dinky and Yack have no named-condition construct.** The shape `state-gate` worried about is in
/// a shipped, hand-authored adventure game by an author who built the language it is written in,
/// and he did not add a way to name one.
///
/// That is not proof the primitive is wrong — `.yidam/decisions/two-bars.yml` established that a
/// negative claim about an engine needs an enumeration and this is one game's guards rather than a
/// language reference. It is the strongest evidence available on this question, and it points the
/// same way as everything else here.
#[test]
fn the_shipped_game_composes_thirteen_times_and_names_nothing() {
    let catalog = std::fs::read_to_string(common::root().join(".yidam/catalog/delores-source.md"))
        .expect("readable");
    let flat = catalog.split_whitespace().collect::<Vec<_>>().join(" ");
    for held in [
        "141 guards in square brackets",
        "79 are real expressions",
        "`==` 39 · `!` 16 · `&&` 7 · `>` 4 · `and` 3 · `||` 3",
    ] {
        assert!(flat.contains(held), "the catalog no longer says {held:?}");
    }
    assert!(
        !flat.contains("condition ") || !flat.contains("named condition construct"),
        "the catalog has learned of a named-condition construct in Dinky, which is the one \
         retrieval that would reopen this"
    );
}
