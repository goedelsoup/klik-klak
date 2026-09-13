//! Whether a binary verb earns a whole dimension.
//!
//! `verb/use-with` seeded the sharpest form of this corpus's central question: *"A vocabulary
//! without it is meaningfully smaller — not by one entry, but by a whole dimension. Whether the
//! puzzles it enables can be expressed by unary verbs over a richer world state is `[open]`, and
//! the answer decides whether 'minimal' means fewer verbs or simpler ones."*
//!
//! It is answerable mechanically, because the reduction is already implemented. `offered` gates a
//! binding's second object on `items[o] >= 1`, and `Pred::Carrying(o)` evaluates to
//! `items[o] >= 1`. The same expression, reached two ways. So moving the object into the guard is
//! not an approximation of the binary form — it is the binary form, spelled differently, and the
//! tests below hold that against every specimen.
//!
//! `.yidam/decisions/binary-dimension.yml` is the record.

mod common;

use game_format::{Binding, Game, Pred};
use reachability::{analyse, Cause};

fn load(source: &str) -> Game {
    let (mut g, mut p) = game_format::parse(source, "arity.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{msgs:?}");
    g
}

/// The reduction, applied to the model rather than to the text: every binding's second object
/// becomes a `carrying` conjunct on its guard. No verb is renamed and no verb is added, because
/// what is being tested is whether *arity* costs the analysis anything — not what the resulting
/// game would be called.
fn unarise(g: &mut Game) -> usize {
    let mut moved = 0;
    for b in &mut g.bindings {
        if let Some(o) = b.object.take() {
            moved += 1;
            let carrying = Pred::Carrying(o);
            b.when = Some(match b.when.take() {
                Some(q) => Pred::And(Box::new(carrying), Box::new(q)),
                None => carrying,
            });
        }
    }
    moved
}

fn binary_count(g: &Game) -> usize {
    g.bindings
        .iter()
        .filter(|b: &&Binding| b.object.is_some())
        .count()
}

/// **The answer, and it is exact rather than approximate.** Every figure the checker reports is
/// unchanged on every specimen: the same reachable set, the same transitions, the same strandings,
/// the same winnability, the same latch violations. Arity costs the state space nothing.
#[test]
fn moving_the_second_object_into_the_guard_changes_no_figure() {
    for (name, src) in common::sources() {
        let before = load(&src);
        let n = binary_count(&before);
        assert!(n > 0, "{name} has no binary binding to reduce");

        let mut after = load(&src);
        assert_eq!(unarise(&mut after), n);
        assert_eq!(binary_count(&after), 0);

        let a = analyse(&before);
        let b = analyse(&after);
        assert_eq!(
            (
                a.states,
                a.transitions,
                a.winnable,
                a.stranded,
                a.strandings.len()
            ),
            (
                b.states,
                b.transitions,
                b.winnable,
                b.stranded,
                b.strandings.len()
            ),
            "{name} analysed differently once its verbs were made unary:\n{}\n---\n{}",
            a.render(),
            b.render()
        );
        assert_eq!(a.observed_space, b.observed_space, "{name}");
        assert_eq!(a.latch_violations, b.latch_violations, "{name}");
    }
}

/// **What it costs is the report, and the specimen's committed defect is where that shows.**
/// `lighthouse.game` strands the player by pouring the oil into the wrong hinge, and the checker
/// names the object it was poured from. Reduced to a unary verb, the same states are still
/// stranded, by the same binding, for the same cause — and the report can no longer say what was
/// spent, because the object is now one conjunct of a guard rather than a field of the binding.
///
/// `verb/use-with` predicted exactly this half of the cost when it was seeded: *"every refusal has
/// to name which of the two objects is wrong."*
#[test]
fn what_it_costs_is_the_diagnostic_and_not_the_analysis() {
    let a = analyse(&load(&common::source("lighthouse")));
    let mut reduced = load(&common::source("lighthouse"));
    unarise(&mut reduced);
    let b = analyse(&reduced);

    assert_eq!(a.strandings.len(), 1);
    assert_eq!(b.strandings.len(), 1);
    let (x, y) = (&a.strandings[0], &b.strandings[0]);
    assert_eq!(x.cause, Cause::Consumption);
    assert_eq!(y.cause, Cause::Consumption);
    assert_eq!(x.target, y.target, "the same binding is still blamed");
    assert_eq!(x.object.as_deref(), Some("oil-can"));
    assert_eq!(
        y.object, None,
        "the reduced form cannot name what was spent, which is the whole of what it loses here"
    );
    assert!(a.render().contains("oil-can"));
    assert!(!b.render().contains(" oil-can on "));
}

/// The census the record quotes, held against the specimens.
///
/// Delores ships 203 bindings of which 26 take a second object — **12.8%** — across 16 distinct
/// binary verbs, with 18 declared binary in an engine list validated at boot and no generic
/// "use with" among them (`.yidam/catalog/delores-source.md`). These specimens were written
/// without reference to that game and land in the same place.
///
/// **Twelve of a hundred and twenty is 10.0%, and it was eleven of ninety-one — 12.1%.** A fifth
/// game moved the share by two points and left it inside the band, which is the useful thing about
/// stating a claim as a range: an agreement between two independent bodies of work survives one
/// more author, and would not have survived being pinned to three decimal places.
#[test]
fn the_binary_share_is_about_an_eighth_in_both_bodies_of_work() {
    let total: usize = common::sources()
        .iter()
        .map(|(_, s)| load(s).bindings.len())
        .sum();
    let binary: usize = common::sources()
        .iter()
        .map(|(_, s)| binary_count(&load(s)))
        .sum();
    assert_eq!(binary, 16);
    let share = binary as f64 / total as f64;
    assert!(
        (0.10..0.16).contains(&share),
        "binary share {share:.3} ({binary} of {total}); Delores is 0.128 and the record quotes both"
    );
}

/// The reduction as an author would have to write it, rather than as the model can apply it.
///
/// `unarise` above moves the second object into the guard inside the `Game`, which is the right
/// instrument for asking what arity costs the *analysis*. It cannot answer what arity costs the
/// person, because the person writes text. This does the same move on the source: the object comes
/// out of the block header and goes back in as a `carrying` conjunct on the guard, inserting one
/// where there is none.
fn unarise_source(src: &str) -> String {
    let mut out = String::new();
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        let object = t
            .strip_suffix(':')
            .and_then(|h| h.strip_prefix("use-with "))
            .or_else(|| t.strip_suffix(':').and_then(|h| h.strip_prefix("give-to ")))
            .map(|o| o.to_string());
        let Some(object) = object else {
            out += lines[i];
            out.push('\n');
            i += 1;
            continue;
        };
        let indent = lines[i].len() - lines[i].trim_start().len();
        let verb = t.split_whitespace().next().unwrap();
        out += &format!("{}{verb}:\n", " ".repeat(indent));

        // the block's own lines, with the first bare `when` gaining a conjunct
        let mut body = vec![];
        let mut j = i + 1;
        while j < lines.len() {
            let l = lines[j];
            if !l.trim().is_empty() && l.len() - l.trim_start().len() <= indent {
                break;
            }
            body.push(l.to_string());
            j += 1;
        }
        let first_when = body.iter().position(|l| l.trim().starts_with("when "));
        match first_when {
            Some(k) => {
                let l = &body[k];
                let pad = l.len() - l.trim_start().len();
                let rest = l.trim().strip_prefix("when ").unwrap().trim();
                body[k] = format!("{}when  carrying {object} and {rest}", " ".repeat(pad));
            }
            None => body.insert(
                0,
                format!("{}when  carrying {object}", " ".repeat(indent + 2)),
            ),
        }
        for l in body {
            out += &l;
            out.push('\n');
        }
        i = j;
    }
    out
}

/// Frame — every non-space character outside a string literal — in the response blocks of the
/// binary verbs, before and after. The unit is the one `crates/game-format/tests/duplication.rs`
/// uses.
fn binary_frame(src: &str) -> usize {
    let lines: Vec<&str> = src.lines().collect();
    let (mut total, mut i) = (0usize, 0usize);
    while i < lines.len() {
        let t = lines[i].trim();
        let is_binary = t.starts_with("use-with") || t.starts_with("give-to");
        if !is_binary || !t.ends_with(':') {
            i += 1;
            continue;
        }
        let indent = lines[i].len() - lines[i].trim_start().len();
        let mut j = i;
        loop {
            let l = lines[j];
            if !l.trim().is_empty() {
                let mut quoted = false;
                for c in l.trim().chars() {
                    match c {
                        '"' => quoted = !quoted,
                        _ if quoted || c == ' ' => {}
                        _ => total += 1,
                    }
                }
            }
            j += 1;
            if j >= lines.len() {
                break;
            }
            let l = lines[j];
            if !l.trim().is_empty() && l.len() - l.trim_start().len() <= indent {
                break;
            }
        }
        i = j;
    }
    total
}

/// **What arity costs the author is negative, and the sign is the finding.**
///
/// The binary key costs six and a half characters — a block header is `use-with wick:` instead of
/// `use-with:`. Writing the same eleven bindings without it costs **a hundred and twenty-four
/// more**, because every one of them has to say `carrying <item>` in a guard, and that is a thing
/// the key was saying for free.
///
/// The two totals fell by eight when `availability` deleted two unreachable refusals, both of them
/// on binary bindings and both there because the key had already refused. They fell by one more
/// when `authored-text` took a needless escape out of the ferryman's refusal — a specimen edit
/// nothing could have justified until the response text was kept. **The difference between them
/// has not moved once**, which is the figure this check is for.
///
/// `binary-dimension` reached the same conclusion structurally — the reduction *"moves the item is
/// carried out of the first group and into the third"* — and filed the authoring cost as
/// unmeasured. It is uniform per site: **every binary site in every specimen pays eleven or twelve
/// characters**, because the sentence added is the same sentence.
///
/// **This check said that and pinned the other thing.** The prose named the invariant and the
/// assertion held the aggregate percentage — fifteen, then fifteen again when a fifth specimen
/// arrived — and read the repeat as confirmation that the cost does not depend on the game. A
/// sixth moved it to eighteen without a single site's cost changing.
///
/// **The spread it was concealing was there the whole time.** Per game the surcharge runs from
/// **11% in `lighthouse.game` to 34% in `patrol.game`**, and both of those were among the five.
/// `relay.game` pays 31% and changed nothing except which games are being averaged. An aggregate
/// that lands on the same number twice is not thereby measuring something stable; these two
/// landed on fifteen because of how much frame the big game has, and the big game is one of six.
///
/// So the invariant is pinned and the percentage is reported as the range it is. The old
/// assertion would have gone on passing for exactly as long as nobody changed the mix.
/// `.yidam/decisions/or-earns-it.yml`.
#[test]
fn writing_the_reduction_costs_more_than_the_key_it_removes() {
    let (mut before, mut after) = (0usize, 0usize);
    for (_, src) in common::sources() {
        before += binary_frame(&src);
        after += binary_frame(&unarise_source(&src));
    }
    assert_eq!((before, after), (1019, 1203));

    // **The invariant.** Every binary site in every specimen pays the same sentence, and the
    // sentence is eleven or twelve characters. This is what the note above always claimed.
    let mut per_site = Vec::new();
    let mut percent = Vec::new();
    for (_, src) in common::sources() {
        let (b, a) = (binary_frame(&src), binary_frame(&unarise_source(&src)));
        let sites = binary_count(&load(&src));
        per_site.push((a - b) / sites);
        percent.push((a - b) * 100 / b);
    }
    assert_eq!(
        per_site,
        [11, 11, 12, 11, 11, 12],
        "characters added at each binary site, per specimen"
    );

    // And the percentage is not an invariant and never was: it is the per-site cost divided by how
    // much frame the game already had, which is a fact about the game.
    percent.sort_unstable();
    assert_eq!(
        (percent[0], percent[percent.len() - 1]),
        (11, 34),
        "the surcharge as a share, lowest and highest across the six"
    );
}

/// And it is a *writable* surcharge on all four specimens — every figure holds, and the cell check
/// stays quiet.
#[test]
fn the_reduction_is_writable_and_moves_no_figure() {
    for (name, src) in common::sources() {
        let rewritten = unarise_source(&src);
        assert_ne!(rewritten, src, "{name}: the rewrite must apply");
        let (mut g, mut p) = game_format::parse(&rewritten, "reduced.game");
        p.extend(game_format::resolve(&mut g));
        let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
        assert!(msgs.is_empty(), "{name} reduced: {msgs:?}");

        let (a, b) = (analyse(&load(&src)), analyse(&g));
        assert_eq!(
            (a.states, a.transitions, a.winnable, a.strandings.len()),
            (b.states, b.transitions, b.winnable, b.strandings.len()),
            "{name}"
        );
    }
}

/// **One site would have collided, and the thing that saves it is the primitive two earlier phases
/// admitted.**
///
/// `lamp-room.lamp` carries three `use-with` bindings — wick, oil, match. Reduced, all three are
/// keyed `(use-with, lamp)`, and `crates/game-format/tests/cells.rs` refuses a pair nothing tells
/// apart. They survive because each is already guarded on a different value of `lamp-prep`, which
/// is the ordered sequence `world-flag/counter-flag` was admitted for.
///
/// Strip the counter guards and leave only the carrying conjunct the reduction adds — which is all
/// a site with no ordering would have — and the cell is ambiguous. So the reduction is safe here
/// by a coincidence of this specimen rather than by construction.
#[test]
fn the_one_site_that_collides_is_saved_by_its_counter() {
    let reduced = unarise_source(&common::source("lighthouse"));
    let g = load(&reduced);
    let n = g
        .bindings
        .iter()
        .filter(|b| matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "lamp-room.lamp"))
        .filter(|b| b.verb == "use-with" && b.object.is_none())
        .count();
    assert_eq!(n, 3, "three bindings in one cell");

    let flattened = reduced
        .replace(" and lamp-prep = 0", "")
        .replace(" and lamp-prep = 1", "")
        .replace(" and lamp-prep = 2", "");
    assert_ne!(flattened, reduced);
    let (mut g, mut p) = game_format::parse(&flattened, "flat.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(
        msgs.iter()
            .any(|m| m.contains("nothing tells the two apart")),
        "without the ordering the cell is ambiguous: {msgs:?}"
    );
}

/// **The table is two-dimensional and this corpus had been counting it flat.**
///
/// `.yidam/decisions/default-binding.yml` computed the specimens' authored density as *"42 sites
/// against six verbs: 252 cells, 87 authored, 34.5%"*. Two of the six verbs take a second object,
/// so their cells are sites × items, not sites. Counted that way there were **396**, and the
/// density **22.0%**.
///
/// **A fifth specimen makes it 56 sites, 480 cells and 24.1%**, and the density held within two
/// points across a game written years after the others. The
/// authored count is computed here rather than quoted, because quoting it is how the flat figure
/// survived three phases — and how this file went on reporting four games' worth of sites after a
/// fifth was written (`.yidam/decisions/unread-specimen.yml`).
///
/// The correction runs the same way on the other side of the comparison and further: 164 objects
/// against 57 verbs is 9,348 only if none of the 57 is binary, and eighteen are.
#[test]
fn the_binary_verbs_make_the_table_two_dimensional() {
    let (mut sites, mut cells) = (0usize, 0usize);
    for (_, src) in common::sources() {
        let g = load(&src);
        let n: std::collections::BTreeSet<String> = g
            .bindings
            .iter()
            .filter_map(|b| match &b.site {
                game_format::Site::InRoom { room, target } => Some(format!("{room}/{target}")),
                _ => None,
            })
            .collect();
        sites += n.len();
        cells += n.len() * 4 + n.len() * g.items.len() * 2;
    }
    assert_eq!(sites, 72, "sites, where `default-binding` counted 42");
    assert_eq!(
        cells, 672,
        "and the cell count it should have used, where it used 252"
    );

    // computed, not quoted: every binding an author sited on a hotspot in a room
    let authored: usize = common::sources()
        .iter()
        .map(|(_, src)| {
            load(src)
                .bindings
                .iter()
                .filter(|b| matches!(&b.site, game_format::Site::InRoom { .. }))
                .count()
        })
        .sum();
    assert_eq!(authored, 144, "where `default-binding` counted 87");
    assert_eq!(
        authored * 1000 / cells,
        214,
        "24.1%, against the 34.5% the flat count produced"
    );
}
