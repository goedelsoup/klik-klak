//! How big the bounded language actually is.
//!
//! `data-format/interaction-table` has carried, since genesis, *"whether a bounded expression
//! language avoids [a general-purpose escape] is `[open]` and is the most valuable open question
//! in this corpus."* Answering it needs the size of the thing, stated where it cannot drift.
//!
//! This is that: every construct the format admits, enumerated. The point is not that the numbers
//! are small — it is what they cost to reach. Nineteen of this corpus's twenty-six decision
//! records weigh a candidate against the authoring language. **Five constructs were admitted, one
//! was withdrawn, and twelve candidates were weighed and refused** — and Delores, 203 bindings and
//! 1,328 call sites across 107 distinct functions, was read in full against it without the surface
//! moving to meet it.
//!
//! A change here is not a failure. It is a signal that `.yidam/decisions/escape-hatch.yml` needs
//! reading again, which is the whole reason the figures are pinned rather than described.

use game_format::{Cmp, Effect, Kind, Pred, Site};

/// Every keyword the parser dispatches on, read off the parser rather than listed by hand.
fn keywords() -> Vec<String> {
    let src = include_str!("../src/parse.rs");
    // Match arms, including or-patterns: `"when" | "else" | "otherwise" => {`.
    let mut v: Vec<String> = src
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('"') && l.contains("=>"))
        .flat_map(|l| {
            l.split("=>")
                .next()
                .unwrap()
                .split('|')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect::<Vec<_>>()
        })
        .filter(|k| !k.is_empty() && k.chars().all(|c| c.is_ascii_lowercase() || c == '-'))
        .collect();
    v.sort();
    v.dedup();
    v
}

/// The closed verb set, likewise.
fn verbs() -> Vec<String> {
    let src = include_str!("../src/parse.rs");
    let line = src
        .lines()
        .find(|l| l.trim_start().starts_with("\"look-at\""))
        .expect("the VERBS array is one line");
    let mut v: Vec<String> = line
        .split(',')
        .filter_map(|s| {
            s.trim()
                .trim_matches(|c| c == '"' || c == ']' || c == ';')
                .into()
        })
        .map(|s: &str| s.trim().trim_matches('"').to_string())
        .filter(|s| s.contains('-'))
        .collect();
    v.sort();
    v
}

#[test]
fn the_whole_authoring_surface_is_fifty_one_constructs() {
    // Exhaustive matches: a new variant does not compile until it is named here.
    let effects: usize = {
        fn each(e: &Effect) -> u8 {
            match e {
                Effect::SetFlag(_) => 1,
                Effect::ClearFlag(_) => 2,
                Effect::SetCounter(..) => 3,
                Effect::Give(..) => 4,
                Effect::Spend(..) => 5,
                Effect::Move(..) => 6,
                Effect::Enter(_) => 7,
                Effect::Leave => 8,
            }
        }
        let _ = each;
        8
    };
    let predicates: usize = {
        fn each(p: &Pred) -> u8 {
            match p {
                Pred::Flag(_) => 1,
                Pred::Carrying(_) => 2,
                Pred::Chosen(_) => 3,
                Pred::Cmp(..) => 4,
                Pred::Count(..) => 5,
                Pred::At(..) => 6,
                Pred::Not(_) => 7,
                Pred::And(..) => 8,
                Pred::Or(..) => 9,
            }
        }
        let _ = each;
        9
    };
    let kinds: usize = {
        fn each(k: &Kind) -> u8 {
            match k {
                Kind::Room => 1,
                Kind::Actor => 2,
                Kind::Item => 3,
                Kind::Flag => 4,
                Kind::Counter => 5,
                Kind::Conversation => 6,
                Kind::Hotspot => 7,
                Kind::Option_ => 8,
            }
        }
        let _ = each;
        8
    };
    let comparisons: usize = {
        fn each(c: &Cmp) -> u8 {
            match c {
                Cmp::Eq => 1,
                Cmp::Ne => 2,
                Cmp::Lt => 3,
                Cmp::Le => 4,
                Cmp::Gt => 5,
                Cmp::Ge => 6,
            }
        }
        let _ = each;
        6
    };
    let sites: usize = {
        fn each(s: &Site) -> u8 {
            match s {
                Site::InRoom { .. } => 1,
                Site::InConversation { .. } => 2,
                Site::OnEnter { .. } => 3,
            }
        }
        let _ = each;
        3
    };

    let verbs = verbs();
    assert_eq!(
        verbs,
        ["give-to", "look-at", "pick-up", "talk-to", "use-with", "walk-to"],
        "the verb set is closed"
    );

    let keywords = keywords();
    assert_eq!(
        keywords,
        [
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
        ],
        "every keyword the parser dispatches on"
    );

    // What an author can write, in total.
    assert_eq!(verbs.len(), 6);
    assert_eq!(keywords.len(), 19);
    assert_eq!(effects, 8, "and eight is the ontology's, not a choice");
    assert_eq!(predicates, 9, "six atoms and three connectives");
    assert_eq!(comparisons, 6);
    assert_eq!(sites, 3);
    let _ = kinds;

    assert_eq!(
        verbs.len() + keywords.len() + effects + predicates + comparisons + sites,
        51,
        "the whole surface an author writes against"
    );
}

/// **The bound is not a budget — it is the ontology, and the code says which relationship.**
///
/// `crates/game-format/tests/effects.rs` holds each effect against the class file that declares
/// its relationship. This holds the shape of that claim: eight effects over six relationships, so
/// two relationships are realised twice and the list cannot grow without the ontology growing.
#[test]
fn eight_effects_realise_six_relationships() {
    let src = include_str!("../src/lib.rs");
    let block = src
        .split("pub enum Effect {")
        .nth(1)
        .expect("the effect enum")
        .split("\n}")
        .next()
        .unwrap();
    let mut rels: Vec<&str> = block
        .lines()
        .filter_map(|l| l.trim().strip_prefix("/// `"))
        .filter_map(|l| l.split('`').next())
        .filter(|l| l.contains("->"))
        .collect();
    rels.sort();
    rels.dedup();
    assert_eq!(
        rels.len(),
        6,
        "eight variants naming six relationships: {rels:?}"
    );
}
