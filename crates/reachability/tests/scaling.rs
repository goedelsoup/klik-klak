//! Does the collapse hold at scale? Two generated families, and the answer is no.
//!
//! Every number this project has published about the collapse came from a game small enough to
//! enumerate by hand — 103, 20 and 14 states — and `design/specimen/README.md` has said for six
//! phases that three small games do not establish it holds for a large one. These are the large
//! ones. They are generated rather than authored, which costs realism and buys a *curve*: a
//! single game gives one point, and tractability is a question about growth.
//!
//! **The finding is that the collapse is a property of the game's dependency structure and not
//! of the language.** A corridor collapses totally. A hub does not collapse at all, and a hub is
//! an ordinary adventure-game shape — find the three map pieces, collect the four keys.

use reachability::analyse;

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "generated.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(
        m.is_empty(),
        "generated game does not resolve:\n{}",
        m.join("\n")
    );
    g
}

/// A corridor. Room *i* holds the key to the door into room *i+1*: every flag depends on the
/// one before it, which is the shape `world-flag/latched-flag` had in mind.
fn chain(n: usize) -> String {
    let mut s = String::new();
    for i in 0..=n {
        s += &format!("room    r{i} \"Room {i}\"\n");
    }
    for i in 0..n {
        s += &format!("item    k{i} \"Key {i}\"\n");
    }
    for i in 0..n {
        s += &format!("flag    d{i} latched \"Door {i}\"\n");
    }
    for i in 0..n {
        s += &format!("\nroom r{i}\n");
        if i == 0 {
            s += "  start\n";
        }
        s += &format!(
            "  hotspot key{i} \"a key\"\n    is k{i}\n    pick-up:\n      do    give k{i}\n"
        );
        s += &format!("  hotspot door{i} \"a door\"\n    use-with k{i}:\n      when  not d{i}\n      do    set d{i}\n      else  \"It is open.\"\n");
        s += &format!(
            "    walk-to:\n      when  d{i}\n      do    move r{}\n      else  \"Locked.\"\n",
            i + 1
        );
    }
    s += &format!("\nroom r{n}\n  hotspot out \"the way out\"\n    walk-to:\n      ending\n");
    s
}

/// A hub. *n* tokens in one room, no order between them, all wanted at the gate. Nothing here
/// is exotic: it is "collect the four keys", written down.
fn hub(n: usize) -> String {
    let mut s = String::from("room    hub \"The Hub\"\n");
    for i in 0..n {
        s += &format!("item    k{i} \"Token {i}\"\n");
    }
    s += "\nroom hub\n  start\n";
    for i in 0..n {
        s += &format!(
            "  hotspot tok{i} \"a token\"\n    is k{i}\n    pick-up:\n      do    give k{i}\n"
        );
    }
    let all = (0..n)
        .map(|i| format!("carrying k{i}"))
        .collect::<Vec<_>>()
        .join(" and ");
    s += &format!(
        "  hotspot gate \"the gate\"\n    walk-to:\n      when  {all}\n      ending\n      else  \"Not yet.\"\n"
    );
    s
}

/// **The collapse, where it holds.** Ordered dependencies mean the reachable set is *linear* in
/// the number of rooms while the space is exponential. This is the case the argument was built
/// on and it is real.
#[test]
fn a_corridor_collapses_and_its_reachable_set_is_linear() {
    for n in [4usize, 8, 16, 32] {
        let r = analyse(&load(&chain(n)));
        assert!(r.winnable);
        assert_eq!(r.states, 3 * n + 1, "corridor of {n} is linear");
    }
}

/// **The collapse, where it fails — completely.** No order between the tokens means every subset
/// of them is reachable, so the reachable set is the *whole* declared space. Not a small
/// fraction of it: all of it, at every size.
#[test]
fn a_hub_does_not_collapse_at_all() {
    for n in [2usize, 4, 6, 8, 10, 12] {
        let r = analyse(&load(&hub(n)));
        assert!(r.winnable);
        assert_eq!(r.states, 1 << n, "a hub of {n} tokens has 2^{n} states");
        assert_eq!(
            r.observed_space,
            1u128 << n,
            "and that is the entire space — the collapse is 100%, which is to say none"
        );
    }
}

/// The contrast, stated as the thing that matters. At sixteen declarations the corridor has
/// forty-nine reachable states and the hub has sixty-five thousand — the same language, the
/// same checker, and a difference of three orders of magnitude that comes entirely from whether
/// the author happened to order the puzzles.
#[test]
fn the_language_admits_both_and_guarantees_neither() {
    let corridor = analyse(&load(&chain(16)));
    let hub16 = analyse(&load(&hub(16)));
    assert_eq!(corridor.states, 49);
    assert_eq!(hub16.states, 65536);
    assert!(hub16.states > corridor.states * 1000);
}

/// **Why the old number could not have found this.** `observed_space` used to multiply in one
/// bit per acquisition site for `taken`, which is determined by the item count — bookkeeping
/// this checker keeps, not world state anybody declared. A hub of *n* tokens therefore reported
/// 2^n states against 4^n combinations: a collapse to 2^-n that was pure redundancy, and which
/// looked exactly like the corridor's.
///
/// The metric that could not tell a linear family from an exponential one was reporting the
/// representation, not the game.
#[test]
fn bookkeeping_is_not_counted_as_world_state() {
    let r = analyse(&load(&hub(6)));
    // 6 items observed at 0..1, and nothing else varies
    assert_eq!(r.observed_space, 64);
    // the old computation would have doubled once per takeable hotspot
    assert_ne!(r.observed_space, 4096);
}

/// A space too large to count is reported as such rather than as `u128::MAX`, which the report
/// used to divide by and print a percentage from.
#[test]
fn a_space_too_large_to_count_says_so() {
    let r = analyse(&load(&chain(64)));
    assert!(r.space_saturated);
    assert!(r.render().contains("no fraction is reported"));
    assert!(!r.render().contains('%'));
}
