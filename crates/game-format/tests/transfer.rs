//! **The player is not an actor in this format, and one game needs it to be.**
//!
//! [`actor/player-character`] has carried this corpus's oldest open question since the root
//! commit: whether transferring control between characters *"needs vocabulary beyond a world flag
//! naming the controlled actor"*. `.yidam/decisions/resolver-kind.yml` classified it as the one
//! question in the corpus that only a **specimen** could close, and observed that none of the four
//! in `design/specimen` switches at all, so nothing had ever put it to the instrument that answers
//! it.
//!
//! `lock.game` is the fifth, and it does what the question proposes: one flag, `ashore`, naming the
//! controlled character. The measurements below are what that costs, computed off the parse tree
//! rather than counted by hand.
//!
//! **The finding is one line up in the model.** `Effect::Move`'s own doc comment says it: *"`None`
//! is the player, and the asymmetry is the point."* The player is a nameless slot with a room and
//! an inventory; an `actor` is a name with a room and no inventory. Control transfer needs them to
//! be one kind of thing — which is what the node has said since genesis, from UFO, and in the
//! opposite direction: *"modeling the controlled one as a distinct kind means the transfer is a
//! change of kind — which is exactly the thing UFO says a kind cannot undergo."*
//!
//! The corpus made `player` a role. The format made it a kind. Nothing noticed for eleven phases
//! because no game had two.
//!
//! [`actor/player-character`]: ../../../.yidam/corpus/actor/player-character.yml
//! `.yidam/decisions/control-transfer.yml`.

use game_format::{parse, resolve, Effect, Game, Pred};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn read(name: &str) -> String {
    std::fs::read_to_string(root().join("design/specimen").join(name)).expect("readable")
}

fn game(name: &str) -> Game {
    let src = read(name);
    let (mut g, mut problems) = parse(&src, name);
    problems.extend(resolve(&mut g));
    assert!(problems.is_empty(), "{name}: {problems:?}");
    g
}

fn specimens() -> Vec<(String, Game)> {
    let dir = root().join("design/specimen");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .map(|p| {
            p.file_name()
                .expect("a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|n| {
            let g = game(&n);
            (n.trim_end_matches(".game").to_string(), g)
        })
        .collect()
}

/// The one flag this game uses to name the controlled character.
const CONTROL: &str = "ashore";

/// Every binding that writes it — a transfer of control, whatever else it does.
fn transfers(g: &Game) -> Vec<&game_format::Binding> {
    g.bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, Effect::SetFlag(f) | Effect::ClearFlag(f) if f == CONTROL))
        })
        .collect()
}

/// Where each actor can stand: declared by `holds`, plus every room a binding moves them to.
fn reachable_rooms(g: &Game) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = g
        .actor_start
        .iter()
        .map(|(a, r)| (a.clone(), BTreeSet::from([r.clone()])))
        .collect();
    for b in &g.bindings {
        for e in &b.effects {
            if let Effect::Move(Some(actor), room) = e {
                out.entry(actor.clone()).or_default().insert(room.clone());
            }
        }
    }
    out
}

/// **The two records of where the player is are never held against each other, and one game has
/// them both name the same person.**
///
/// This is a census. `lock.game` writes `start` and `holds boatman` in the same room section
/// because the player *is* the boatman, and the format has no way to say so: `start_room` is where
/// the nameless player begins and `actor_start` is where a named actor begins, and resolution
/// relates them not at all.
///
/// The mutation is the proof and it is the shape [`found-in`] established — declare the controlled
/// actor somewhere other than the room the player starts in, which is a flat contradiction, and
/// the game **parses clean, resolves clean, plays and is winnable**. What it is not is the same
/// game: three more reachable states and five more transitions, attributed to nothing.
///
/// A defect that changes the report without changing what the report says is the one this
/// repository keeps finding, and this is the first time it has been found in a fact the format
/// *cannot* hold rather than in one it forgot to.
///
/// [`found-in`]: ../../../.yidam/decisions/found-in.yml
#[test]
fn the_player_and_the_actor_the_player_is_are_two_records_of_one_room() {
    let g = game("lock.game");
    assert_eq!(
        g.start_room.as_deref(),
        Some("chamber"),
        "where play begins"
    );
    assert_eq!(
        g.actor_start.get("boatman").map(String::as_str),
        Some("chamber"),
        "and where the actor the player is begins — written twice, agreeing by care"
    );

    // Moved, not deleted: deleting it is caught, because `boatman in <room>` would have no
    // answer. Contradicting it is not caught by anything.
    let bent = read("lock.game")
        .replace("  start\n  holds boatman\n", "  start\n")
        .replace(
            "room lockside\n  holds mate\n",
            "room lockside\n  holds mate\n  holds boatman\n",
        );
    let (mut m, mut problems) = parse(&bent, "bent");
    problems.extend(resolve(&mut m));
    assert!(
        problems.is_empty(),
        "the contradiction is not a parse error, which is the finding: {problems:?}"
    );
    assert_eq!(m.start_room.as_deref(), Some("chamber"));
    assert_eq!(
        m.actor_start.get("boatman").map(String::as_str),
        Some("lockside")
    );
}

/// **What a transfer of control costs, counted: six sites, and the bound on them is twice the
/// product of the two characters' room sets.**
///
/// A binding carries one guard and one effect list, so a switch cannot say *go to where she is* —
/// it names a room. A site is therefore needed per **ordered pair of positions**: one for each
/// room the outgoing character may be standing in, times each room the incoming one may be, times
/// the two directions. Two rooms each is the smallest map that lets both of them move, and it
/// already wants eight.
///
/// This game writes six. The two it leaves out are the pair that never co-occurs — the boatman
/// reaches the lockside only when the lock is full, and the game ends a move later.
///
/// Two of the six cost nothing: once both characters stand in the same room an `actor` site is
/// live, and the transfer is one binding with no destination in it at all. **The only natural site
/// control transfer has in this format is the case where nobody has to move** — and that is also
/// the only case where the flag carries anything, because everywhere else the room the player is
/// standing in already says who they are.
#[test]
fn the_bound_on_switch_sites_is_twice_the_product_of_two_room_sets() {
    let g = game("lock.game");
    let t = transfers(&g);
    assert_eq!(t.len(), 6, "bindings that write `{CONTROL}`");

    // Four hang on invented scenery, because control transfer is not a verb applied to a target
    // and Layer 0 has nowhere else to put one. Two hang on the character being taken.
    let on_actor: Vec<&str> = t
        .iter()
        .filter_map(|b| match &b.site {
            game_format::Site::InRoom { target, .. }
                if g.declared.get(target).map(|(k, _)| *k) == Some(game_format::Kind::Actor) =>
            {
                Some(target.as_str())
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        on_actor,
        ["boatman", "mate"],
        "the transfers with a site of their own"
    );

    let rooms = reachable_rooms(&g);
    let (b, m) = (&rooms["boatman"], &rooms["mate"]);
    assert_eq!(
        (b.len(), m.len()),
        (2, 2),
        "the smallest map that lets both of them move"
    );

    let pairs = b.len() * m.len();
    let shared = b.intersection(m).count();
    assert_eq!(
        (pairs, 2 * pairs, shared),
        (4, 8, 1),
        "positions the pair can be in, sites that would cover them all, and rooms that hold both"
    );
    assert_eq!(
        t.len() - on_actor.len(),
        4,
        "cross-room sites written, against the six the bound allows"
    );
}

/// **An item cannot be handed over, so the object the game is about is an item for two bindings
/// and a flag for the rest of it.**
///
/// `inventory-item` has three states — unclaimed, carried, spent — and none of them is *in the
/// other character's hands*. `is <item>` gives an item exactly one hotspot, so a thing put down
/// cannot be picked up by somebody else, and `spend` is terminal.
///
/// So the windlass goes up onto the coping as `spend windlass` plus `set windlass-up`, and from
/// there it is a flag. The visible cost is one line of the game: the mate winds the paddle with
/// `walk-to` and not with `use-with windlass`, **because she cannot carry it**, and the binary
/// verb is lost with the item.
#[test]
fn the_object_the_game_is_about_stops_being_an_item_when_it_changes_hands() {
    let g = game("lock.game");
    let (item, flag) = windlass(&g);
    assert_eq!(
        item, 2,
        "bindings in which the windlass is an item: taken, then spent"
    );
    assert_eq!(flag, 4, "and bindings in which it is a flag");

    // The binary verb, lost with the item: nothing in this game uses `use-with` on the paddle.
    let paddle = g
        .bindings
        .iter()
        .find(|b| matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "lockside.paddle-gear")
            && b.effects.iter().any(|e| matches!(e, Effect::SetFlag(f) if f == "paddle-up")))
        .expect("the binding that winds the paddle");
    assert_eq!(
        paddle.verb, "walk-to",
        "winding a paddle, without the windlass in hand"
    );
    assert_eq!(paddle.object, None);
}

/// The two lives of the windlass: bindings in which it is an item, and bindings in which the flag
/// that stands in for it once it has changed hands is read or written.
fn windlass(g: &Game) -> (usize, usize) {
    let item = g
        .bindings
        .iter()
        .filter(|b| {
            b.object.as_deref() == Some("windlass")
                || b.effects.iter().any(
                    |e| matches!(e, Effect::Give(i, _) | Effect::Spend(i, _) if i == "windlass"),
                )
        })
        .count();
    let mut flag = 0;
    for b in &g.bindings {
        let mut asks = BTreeSet::new();
        if let Some(q) = &b.when {
            flags_in(q, &mut asks);
        }
        for r in b.says.iter().chain(&b.refusals) {
            if let Some(q) = &r.when {
                flags_in(q, &mut asks);
            }
        }
        let writes = b
            .effects
            .iter()
            .any(|e| matches!(e, Effect::SetFlag(f) | Effect::ClearFlag(f) if f == "windlass-up"));
        if asks.contains("windlass-up") || writes {
            flag += 1;
        }
    }
    (item, flag)
}

fn flags_in(p: &Pred, out: &mut BTreeSet<String>) {
    match p {
        Pred::Flag(f) => {
            out.insert(f.clone());
        }
        Pred::Not(q) => flags_in(q, out),
        Pred::And(a, b) | Pred::Or(a, b) => {
            flags_in(a, out);
            flags_in(b, out);
        }
        _ => {}
    }
}

/// **The only flag any specimen puts back is the one naming the player.**
///
/// Five games declare fourteen flags between them and write them at thirty-odd sites, and until
/// this one no binding had ever cleared a single one. `world-flag.reversible` has had a value with
/// no authored instance since genesis, and `crates/game-format/tests/coverage.rs` had `clear-flag`
/// on its never-written list for eleven phases.
///
/// The reason is not a gap in the games. Every flag in the other four is a thing that happened to
/// a world, and worlds do not un-happen: a crate is not re-nailed, a toll is not un-paid, a kettle
/// boiled stays boiled. **Which character the player is, is the only fact in any of these games
/// that goes both ways** — and it is not a fact about the world at all.
#[test]
fn the_only_flag_any_specimen_clears_is_the_one_naming_the_controlled_character() {
    let mut cleared: Vec<(String, String)> = Vec::new();
    for (name, g) in specimens() {
        for b in &g.bindings {
            for e in &b.effects {
                if let Effect::ClearFlag(f) = e {
                    cleared.push((name.clone(), f.clone()));
                }
            }
        }
    }
    let which: BTreeSet<&str> = cleared.iter().map(|(_, f)| f.as_str()).collect();
    assert_eq!(
        which.into_iter().collect::<Vec<_>>(),
        [CONTROL],
        "every flag any specimen puts back, over five games"
    );
    assert_eq!(
        cleared.len(),
        3,
        "and the sites that do it — all three of them transfers"
    );
}

/// The documents that quote these figures, by the path they are quoted at.
const QUOTING: [&str; 3] = [
    ".yidam/decisions/control-transfer.yml",
    "design/specimen/README.md",
    "design/bounded-expression-language.md",
];

fn word(n: usize) -> &'static str {
    [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve",
    ][n]
}

/// **Nobody may quote one of these figures from memory.**
///
/// The four checks above are measurements of one game. This one is the reason they are worth
/// having: three documents state them in prose — a record, the specimen findings, and the design
/// the specimen was written against — and prose figures are the defect this repository keeps
/// finding. `.yidam/decisions/collected-verdict.yml` names the three ways they go wrong
/// (unreadable, mistaken, decayed) and the remedy that covers all three, which is to compute them
/// where they are checked and hold the documents to the result.
///
/// So the sentences below are built from the parse tree and the documents must contain them. Edit
/// the specimen and the prose fails until somebody re-reads it; write a number by hand and it
/// fails on the day it is written.
#[test]
fn every_document_quoting_these_figures_quotes_the_computed_one() {
    let root = root();
    let g = game("lock.game");
    // Flattened, because these are wrapped paragraphs and a line break is not a difference of
    // fact. `crates/design-gate/tests/retrieval.rs` reads documents the same way.
    let read = |rel: &str| {
        std::fs::read_to_string(root.join(rel))
            .expect("readable")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    // Six switch sites, in all three, because it is the figure any of them would be tempted to
    // round.
    let sites = format!("{} switch sites", word(transfers(&g).len()));
    for rel in QUOTING {
        assert!(
            read(rel).contains(&sites),
            "{rel} does not say {sites:?}, which is what the game has"
        );
    }

    // The bound the sites are counted against, wherever the argument for it is made.
    let rooms = reachable_rooms(&g);
    let bound = format!(
        "{} sites",
        word(2 * rooms["boatman"].len() * rooms["mate"].len())
    );
    for rel in QUOTING.iter().take(2) {
        assert!(
            read(rel).contains(&bound),
            "{rel} does not say {bound:?}, which is the bound the six are counted against"
        );
    }

    // And the windlass, which is the sentence a reader is most likely to carry away.
    let (item, flag) = windlass(&g);
    let split = format!(
        "an item for {} bindings and a flag for the other {}",
        word(item),
        word(flag)
    );
    for rel in QUOTING.iter().take(2) {
        assert!(read(rel).contains(&split), "{rel} does not say {split:?}");
    }
}
