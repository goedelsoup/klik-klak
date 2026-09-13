//! **The node about spending has four claims, no source, and one of them is false.**
//!
//! [`consuming-binding`] is the only node in this corpus with `[inference]` on everything and
//! `[verified]` on nothing. Two of its four claims are measurements written as opinions — *"against
//! a specimen it named exactly one — a can of oil poured into a hinge"* is a thing the checker
//! prints, not a thing anybody reasoned to. One is an account of how the checker works, and it
//! describes an implementation that no longer exists:
//!
//! > consumption and ordering are the same computation and differ only in which effect the
//! > stranding edge carried
//!
//! They differ by a **counterfactual re-run**. `analyse` takes the spending away and asks whether
//! that rescues the stranded state; if it does the cause is consumption, and if it does not
//! something else closed behind the player. The source says why in a comment older than the node's
//! sentence: *"a binding that both spends and moves carries both kinds of effect, and reading the
//! cause off the list picks one by fiat."*
//!
//! The specimen it names is in the corpus and settles it. **`ferry.game`'s toll spends three coins
//! and is classified `Ordering`**, because taking the spend away rescues nothing — the river is
//! what closed. A syntactic reading gets that one wrong, and it is one of the five strandings
//! these four specimens have.
//!
//! [`consuming-binding`]: ../../../.yidam/corpus/interaction/consuming-binding.yml

mod common;

use game_format::Effect;
use reachability::{analyse, Cause};

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "spend.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

fn spends(b: &game_format::Binding) -> bool {
    b.effects.iter().any(|e| matches!(e, Effect::Spend(..)))
}

/// **Seven bindings spend, six states-worth of stranding, three of them consumption.**
///
/// The node says *"against a specimen it named exactly one."* That was one specimen. Across the
/// specimens there are six strandings and the split is 3 to 3, which is the figure the node should
/// have carried and the reason it is pinned here rather than described there.
///
/// A fifth game added one spending binding and one stranding, and the stranding is an **ordering**
/// one — the committed defect `lock.game` was written to carry. The consumption count has not
/// moved since the third specimen.
#[test]
fn the_corpus_strands_six_times_and_three_are_a_spend() {
    let (mut spending, mut strandings, mut consumption, mut ordering) = (0, 0, 0, 0);
    for (_, src) in common::sources() {
        let g = load(&src);
        spending += g.bindings.iter().filter(|b| spends(b)).count();
        let r = analyse(&g);
        strandings += r.strandings.len();
        for s in &r.strandings {
            match s.cause {
                Cause::Consumption => consumption += 1,
                Cause::Ordering => ordering += 1,
            }
        }
    }
    assert_eq!((spending, strandings, consumption, ordering), (11, 7, 4, 3));
}

/// **A binding that spends is not a consumption stranding, and the difference is a re-run.**
///
/// `ferry.game`'s toll is `use-with coin ferryman`: it spends three coins *and* crosses the river.
/// Reading the cause off the effect list makes it consumption. Asking the counterfactual makes it
/// ordering, because handing the coins back does not bring the player home — what closed is the
/// crossing.
///
/// This is the one case in the corpus where the two answers differ, which is exactly enough: a
/// syntactic classifier is wrong about a fifth of the strandings these specimens have, and the
/// node claiming the two methods are the same computation would have shipped that error.
#[test]
fn the_only_spending_binding_in_ferry_strands_for_another_reason() {
    let g = load(&common::source("ferry"));
    let r = analyse(&g);
    let s = &r.strandings[0];
    assert_eq!((r.strandings.len(), s.verb.as_str()), (1, "use-with"));
    assert_eq!(s.cause, Cause::Ordering);
    assert_eq!(
        s.detail,
        "a gate closed with something still needed behind it"
    );

    let spending: Vec<&game_format::Binding> = g.bindings.iter().filter(|b| spends(b)).collect();
    assert_eq!(spending.len(), 1);
    assert_eq!(
        spending[0].line, s.line,
        "the binding that spends is the binding that strands, and it is still not consumption"
    );
}

/// **Two of the three consumption strandings are not a binary verb applied to a target, which is
/// the whole of what the class says it is.**
///
/// `consuming-binding` declares `shape: binary`, `applies -> use-with` and `requires ->
/// portable-object`. In `nightwatch.game` the spend is authored on an **`on-enter`** — the
/// watchman taking the ledger out of the player's hands — which has no verb, no target and no
/// second object. The checker attributes the stranding to the two `walk-to` bindings that walk
/// into the hall, because an entry binding is forced and the arrival is what the player chose.
///
/// So the corpus's own specimen spends at a site the class it seeded for spending cannot describe.
/// That is not a defect in the specimen: `effect-vocabulary` put `spend` in the effect list, and
/// an effect list belongs to any binding. The class was named after the *shape* that spends most
/// often and then written as though that shape were the only one.
#[test]
fn the_spend_that_strands_nightwatch_is_not_a_binary_binding() {
    let g = load(&common::source("nightwatch"));
    let r = analyse(&g);
    assert_eq!(r.strandings.len(), 2);
    for s in &r.strandings {
        assert_eq!(s.cause, Cause::Consumption);
        assert_eq!(s.verb, "walk-to");
        assert!(
            s.object.is_none(),
            "unary: nothing is being used on anything"
        );
        assert_eq!(s.detail, "spends ledger, which is needed later");
    }

    let spending: Vec<&game_format::Binding> = g.bindings.iter().filter(|b| spends(b)).collect();
    assert_eq!(spending.len(), 1);
    assert!(
        matches!(spending[0].site, game_format::Site::OnEnter { .. }),
        "the one spend in this game is at an arrival trigger"
    );
    assert!(spending[0].object.is_none());
    assert_eq!(
        spending[0].verb, "on-enter",
        "the model gives a verbless site a verb name, which is how it keys one map for two \
         kinds of trigger — `entry-binding` calls this *a binding with no verb and no target*, \
         and the string is there so nothing downstream needs a special case"
    );
}

/// **Every stranding is a success, which is the node's one claim that is genuinely an inference and
/// is worth keeping.**
///
/// *"The only binding in this vocabulary whose success can be the mistake."* A refusal writes
/// nothing and is therefore not a transition, so no stranding edge can be one — that much is true
/// by construction and proves nothing. What can be checked is the other half: the binding the
/// player chose was **shown to them**, in the state it stranded from, with no indication that this
/// use was the wrong one. `Report::unshown` is empty across all four specimens, so every one of the
/// five was on offer.
///
/// The lighthouse case is the whole argument in one line: `use-with oil-can base.hinge` succeeds,
/// says so, and costs four reachable states.
#[test]
fn the_player_was_shown_every_binding_that_stranded_them() {
    for (name, src) in common::sources() {
        let r = analyse(&load(&src));
        assert!(r.unshown.is_empty(), "{name}: {:?}", r.unshown);
    }
    let r = analyse(&load(&common::source("lighthouse")));
    let s = r
        .strandings
        .iter()
        .find(|s| s.cause == Cause::Consumption)
        .expect("the can of oil");
    assert_eq!(
        (s.verb.as_str(), s.object.as_deref(), s.target.as_str()),
        ("use-with", Some("oil-can"), "base.hinge")
    );
    assert_eq!(s.states, 4);
    assert_eq!(
        s.example,
        "in base; carrying crowbar, brass-key, wick; lamp-prep=0"
    );
}
