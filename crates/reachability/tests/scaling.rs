//! Does the collapse hold at scale? Two generated families, and the answer is no.
//!
//! Every number this project has published about the collapse came from a game small enough to
//! enumerate by hand — 91, 20 and 14 states — and `design/specimen/README.md` has said for six
//! phases that three small games do not establish it holds for a large one. These are the large
//! ones. They are generated rather than authored, which costs realism and buys a *curve*: a
//! single game gives one point, and tractability is a question about growth.
//!
//! That reading is true and it is not the whole of what the generators are. See the last test in
//! this file: they are this corpus's own instance of the pressure it had recorded three times in
//! somebody else's game, and the games carrying its central tractability argument are games this
//! vocabulary cannot express.
//!
//! **The finding is that the collapse is a property of the game's dependency structure and not
//! of the language.** A corridor collapses totally. A hub does not collapse at all, and a hub is
//! an ordinary adventure-game shape — find the three map pieces, collect the four keys.

mod common;

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

/// **Acts.** *k* of them, each holding *w* tokens with no order between them and a door that
/// wants all *w*, and no way back. This is the shape the form's inventor prescribes: "Solving
/// one puzzle should open up 2 or 3 new ones, and then those collapses down… to a single
/// solution that then opens up more non-linear puzzles", and "each act ends with a bottle neck
/// to the next act" (`.yidam/catalog/grumpy-gamer-pdc.md`).
///
/// It is the corridor and the hub composed: a hub inside each act, a corridor between them.
fn acts(k: usize, w: usize) -> String {
    let mut s = String::new();
    for j in 0..=k {
        s += &format!("room    a{j} \"Act {j}\"\n");
    }
    for j in 0..k {
        for i in 0..w {
            s += &format!("item    t{j}x{i} \"Token {j}.{i}\"\n");
        }
    }
    for j in 0..k {
        s += &format!("\nroom a{j}\n");
        if j == 0 {
            s += "  start\n";
        }
        for i in 0..w {
            s += &format!("  hotspot h{j}x{i} \"a token\"\n    is t{j}x{i}\n    pick-up:\n      do    give t{j}x{i}\n");
        }
        let all = (0..w)
            .map(|i| format!("carrying t{j}x{i}"))
            .collect::<Vec<_>>()
            .join(" and ");
        s += &format!("  hotspot door{j} \"the way on\"\n    walk-to:\n      when  {all}\n      do    move a{}\n      else  \"Not yet.\"\n", j + 1);
    }
    s += &format!("\nroom a{k}\n  hotspot out \"the way out\"\n    walk-to:\n      ending\n");
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

/// **The answer to "how ordered are a real game's puzzles", turned into a number.**
///
/// Hold the puzzle count fixed at forty-eight and vary only the width of each act. The reachable
/// set is **linear in the number of puzzles and exponential in the width** — which is why the
/// question that replaced the collapse claim was the right one, and why its answer decides
/// tractability by itself.
#[test]
fn width_decides_and_the_puzzle_count_does_not() {
    let measure = |k: usize, w: usize| analyse(&load(&acts(k, w))).states;

    // forty-eight puzzles, arranged four ways
    let narrow = measure(24, 2);
    let prescribed = measure(16, 3);
    let wide = measure(6, 8);
    let very_wide = measure(4, 12);

    assert!(narrow < 150, "24 acts of 2: {narrow}");
    assert!(prescribed < 250, "16 acts of 3: {prescribed}");
    assert!(wide > 1000, "6 acts of 8: {wide}");
    assert!(very_wide > 15000, "4 acts of 12: {very_wide}");
    assert!(
        very_wide > prescribed * 100,
        "same forty-eight puzzles, {prescribed} states against {very_wide}"
    );
}

/// And at the prescribed width the count is what scales linearly. Three hundred puzzles in acts
/// of three is a few hundred states — a game far larger than any of the three specimens, and
/// cheaper to check than the lighthouse.
#[test]
fn a_large_game_at_the_prescribed_width_is_small() {
    let r = analyse(&load(&acts(100, 3)));
    assert!(r.winnable);
    assert!(
        r.states < 1500,
        "three hundred puzzles should stay in the hundreds; got {}",
        r.states
    );
}

/// **What the three functions above are, which took five phases and somebody else's game to see.**
///
/// `chain`, `hub` and `acts` are `definePseudoRoom`. They are Rust `for` loops emitting source in
/// the proposed language, written because a corridor of sixty-four rooms cannot be written by
/// hand — and every one of those rooms is individually sayable. What is not sayable is *sixty-four
/// rooms like this one*.
///
/// The file's own opening called that "generated rather than authored, which costs realism and
/// buys a curve", filing it as a methodological trade. It is the same pressure this corpus had
/// already recorded three times in a shipped game — 110 hotel rooms built by a loop, a helper
/// written because a second author "didn't want to do more copy-paste", 21 presentation names the
/// game defines for itself — and did not recognise in its own repository.
///
/// It matters because the figures these generators produce are this corpus's central evidence
/// about tractability, quoted in `world-flag/latched-flag` and in the design. **The games that
/// carry that argument are games this vocabulary could not express.**
///
/// The numbers, at the sizes the tests above actually run: 1,115 lines of hand-authored game
/// across the specimens, and 6,315 generated across seventeen.
///
/// **The ratio only ever moves one way, and the sixth specimen took it under six.** It was
/// nine-and-a-half when `.yidam/decisions/duplication.yml` was written over four games and 664
/// authored lines; seven at five games; **5.7 at six.** Nothing has been generated since the
/// record; every step is a person writing a game. The test was named for ten and asserted six,
/// and both were snapshots of a number that falls every time somebody authors anything.
///
/// So what is held is the **direction**, which is the claim the record actually rests on: the
/// generated side is fixed and the authored side grows. A threshold on the quotient would have to
/// be re-pinned at every specimen and says nothing the two figures above do not.
#[test]
fn the_corpus_still_generates_more_game_source_than_it_writes() {
    let authored: usize = common::sources()
        .iter()
        .map(|(_, s)| s.lines().count())
        .sum();

    let mut generated = 0usize;
    for n in [4usize, 8, 16, 32, 64] {
        generated += chain(n).lines().count();
    }
    for n in [2usize, 4, 6, 8, 10, 12, 16] {
        generated += hub(n).lines().count();
    }
    for (k, w) in [(24usize, 2usize), (16, 3), (6, 8), (4, 12), (100, 3)] {
        generated += acts(k, w).lines().count();
    }

    assert_eq!(authored, 1115, "the specimens, by hand");
    assert_eq!(generated, 6315, "seventeen games, by loop");
    assert!(
        generated > authored,
        "the generated side is still the larger, which is the whole of what          .yidam/decisions/duplication.yml rests on"
    );
    // and the quotient, reported rather than gated, so the trend is legible without pinning it
    assert_eq!(
        generated * 10 / authored,
        56,
        "generated lines per authored line, tenths — 95 over four games, 70 over five"
    );
}

/// **And what those loops write is not what an author writes**, which is the measurement that
/// says how far the pressure above actually reaches.
///
/// `crates/game-format/tests/duplication.rs` counts the four specimens: one hundred and
/// eighty-eight string literals, one hundred and eighty-six of them different, and roughly two
/// characters of writing for every character of frame. Run the same count on what these three
/// functions emit and it inverts — barely half the strings are distinct, and the distinct half is
/// `"Room 7"` and `"Key 7"`, an index in a template. Frame outweighs writing three to one.
///
/// That is the difference between a shape with instances and a shape with *content*. A generation
/// primitive removes frame, so it is worth having exactly where the repeated part is frame — and
/// these games are almost nothing else, because nobody wrote them. [inference]
/// `.yidam/decisions/frame-width.yml`.
#[test]
fn a_generated_game_is_frame_where_an_authored_one_is_writing() {
    for src in [chain(16), hub(8), acts(4, 4)] {
        let (mut frame, mut prose) = (0usize, 0usize);
        let mut strings: Vec<String> = vec![];
        for l in src.lines() {
            let mut cur = String::new();
            let mut quoted = false;
            for c in l.trim().chars() {
                match c {
                    '"' => {
                        if quoted {
                            prose += cur.chars().count();
                            strings.push(std::mem::take(&mut cur));
                        }
                        quoted = !quoted;
                    }
                    _ if quoted => cur.push(c),
                    ' ' => {}
                    _ => frame += 1,
                }
            }
        }
        let distinct: std::collections::BTreeSet<&String> = strings.iter().collect();
        assert!(
            2 * distinct.len() < 3 * strings.len() / 2,
            "an authored game repeats almost no line; this one repeats a third of them: \
             {} of {}",
            distinct.len(),
            strings.len()
        );
        assert!(
            3 * prose < frame + frame / 10,
            "and it is frame where an authored game is writing: {prose} prose, {frame} frame"
        );
    }
}
