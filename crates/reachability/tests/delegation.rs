//! Delegation, tried in this vocabulary rather than argued about.
//!
//! `data-format/interaction-table` records "calls another verb slot on the same object" as one
//! binding in eleven of a shipped game and "the cheapest thing on this list, and there is no way
//! to say it here". The count held when it was recounted; the description did not. Six of the
//! eighteen are on the same object. The rest reach for another object, and two cross a room.
//!
//! `.yidam/decisions/delegation.yml` has the taxonomy. What is below is the corpus's own first
//! move — *try to express the thing with what is already here and record what failed* — carried
//! out on real bindings rather than on imagined ones, so that "it can already be said" is a
//! claim a reader can run.

use reachability::analyse;

/// Six of the seven shapes, transcribed from `Scripts/` at the commit `delores-source` records.
/// Names and text are rewritten; what is preserved is the shape of each binding and what it
/// reads and writes.
const TX: &str = r####"
room    street  "Main Street"
room    bath    "The Bathroom"

item    screwdriver "a screwdriver"
item    device      "a dangerous dovice"

flag    faucet-fixed latched "the faucet has been tightened"
flag    seen-circus  latched "the circus has been remarked on once"
flag    device-seen  latched "the dovice has been described"
flag    bank-open    latched "the bank has opened"

room street
  start

  hotspot toolbox "a toolbox somebody left out"
    is screwdriver
    pick-up:
      say   "You help yourself to a screwdriver."
      do    give screwdriver

  hotspot cone-one "a traffic cone"
    look-at:
      "A traffic cone. Road safety is important."
  hotspot cone-two "a traffic cone"
    look-at:
      "A traffic cone. Road safety is important."
  hotspot cone-three "a traffic cone"
    look-at:
      "A traffic cone. Road safety is important."

  hotspot circus "the circus"
    look-at:
      when  not seen-circus
      say   "I love the circus! I need to see Ransome's show before I go back."
      do    set seen-circus
      else  "I love the circus!"
  hotspot circus-top "the circus"
    look-at:
      when  not seen-circus
      say   "I love the circus! I need to see Ransome's show before I go back."
      do    set seen-circus
      else  "I love the circus!"

  hotspot bank-door "the bank door"
    look-at:
      when  bank-open   "That is an impressive door."
      otherwise         "CLOSED. Back at noon. — the sign on the door"

  hotspot strange-device "Philipp's Dangerous Dovice"
    is device
    look-at:
      say   "It's Uncle Chuck's Philipp's Dangerous Dovice."
      do    set device-seen
    pick-up:
      when  not device-seen
      say   "It's Uncle Chuck's Philipp's Dangerous Dovice."
      do    give device
      do    set device-seen
    pick-up:
      when  device-seen
      do    give device

  hotspot bath-door "a door marked BATHROOM"
    walk-to:
      do    move bath

room bath
  hotspot street-door "the door out"
    walk-to:
      do    move street

  hotspot sink "a gross sink"
    look-at:
      "It's a gross sink!"
    use-with screwdriver:
      when  not faucet-fixed
      say   "You tighten the collar until it stops dripping."
      do    set faucet-fixed
      else  "Nothing wrong with it now."
  hotspot faucet "the faucet"
    use-with screwdriver:
      when  not faucet-fixed
      say   "You tighten the collar until it stops dripping."
      do    set faucet-fixed
      else  "Nothing wrong with it now."

  hotspot radio "a radio on the cistern"
    use-with screwdriver:
      when  faucet-fixed and carrying device
      say   "The jolt goes through you, the dovice, and the whole street."
      ending
      else  "Not yet."
"####;

fn load(source: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(source, "delegation.game");
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|p| p.to_string()).collect();
    assert!(msgs.is_empty(), "{msgs:?}");
    g
}

#[test]
fn six_of_the_seven_shapes_go_in_as_written() {
    let r = analyse(&load(TX));
    assert!(r.winnable, "{}", r.render());
    assert_eq!(r.states, 36);
}

/// **A whole and a part** — Delores's `sink` forwards four verbs to `sinkFaucet`, which is a
/// larger clickable region delegating to a smaller one inside it. Here that is two hotspots with
/// the same binding, and the binding is written twice.
#[test]
fn a_whole_and_a_part_is_two_hotspots_with_one_binding_written_twice() {
    let g = load(TX);
    let repairs: Vec<&str> = g
        .bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, game_format::Effect::SetFlag(f) if f == "faucet-fixed"))
        })
        .filter_map(|b| match &b.site {
            game_format::Site::InRoom { target, .. } => Some(target.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(repairs, vec!["bath.sink", "bath.faucet"]);
}

/// **A response continuing into a description** — `verb_pickup` ending with `verb_lookat()` when
/// the object has not been looked at yet. One guard per binding means this is two bindings on one
/// site, and the acquisition is what gets written twice. That is duplication of an *effect*, not
/// of text, which is the shape `.yidam/decisions/arrival-trigger.yml` found drifting.
#[test]
fn a_response_that_continues_into_a_description_costs_the_effect_twice() {
    let g = load(TX);
    let gives = g
        .bindings
        .iter()
        .filter(|b| matches!(&b.site, game_format::Site::InRoom { target, .. } if target == "street.strange-device"))
        .filter(|b| b.effects.iter().any(|e| matches!(e, game_format::Effect::Give(i, _) if i == "device")))
        .count();
    assert_eq!(gives, 2, "the give is written once per guard");
}

/// **A named flag makes the shared once-line safe, and an implicit one would not.** Delores says
/// the circus line with `ONCE(...)`, which is per-call-site: duplicate that binding onto the
/// second region of the same circus and the bonus line plays twice. Here the once-ness has a
/// name, both copies read and write it, and duplicating the binding cannot duplicate the line.
#[test]
fn the_once_line_is_shared_because_the_flag_is_named() {
    let g = load(TX);
    let n = g
        .bindings
        .iter()
        .filter(|b| {
            b.effects
                .iter()
                .any(|e| matches!(e, game_format::Effect::SetFlag(f) if f == "seen-circus"))
        })
        .count();
    assert_eq!(n, 2, "both regions set the same named flag");
    assert!(g.flags.iter().any(|f| f.name == "seen-circus" && f.latched));
}

/// **What the duplication costs, and it is not what a checker can see.**
///
/// Four drifts between two copies of one binding — the second circus forgetting the shared flag,
/// one cone's text going its own way, and either half of the sink forgetting the repair. Every
/// one produces the same 36 states, the same winnability and the same silence as the correct
/// transcription, because a reachability checker asks what is *reachable* and a redundant route
/// going bad does not change that. The player who clicks the sink rather than the faucet plays a
/// different game, and nothing here can say so. That is the argument for a vocabulary that does
/// not need the copy — and it is an argument about naming, not about calling.
#[test]
fn drift_between_two_copies_is_invisible_to_this_checker() {
    let base = analyse(&load(TX));
    let drifts = [
        (
            "the second circus drops the shared flag",
            TX.replacen(
                "      do    set seen-circus\n      else  \"I love the circus!\"\n",
                "      else  \"I love the circus!\"\n",
                2,
            )
            .replacen(
                "      else  \"I love the circus!\"\n",
                "      do    set seen-circus\n      else  \"I love the circus!\"\n",
                1,
            ),
        ),
        (
            "one cone's text goes its own way",
            TX.replacen(
                "  hotspot cone-three \"a traffic cone\"\n    look-at:\n      \"A traffic cone. Road safety is important.\"\n",
                "  hotspot cone-three \"a traffic cone\"\n    look-at:\n      \"A traffic cone.\"\n",
                1,
            ),
        ),
        (
            "the sink forgets the repair",
            TX.replacen(
                "      do    set faucet-fixed\n      else  \"Nothing wrong with it now.\"\n",
                "      else  \"Nothing wrong with it now.\"\n",
                1,
            ),
        ),
        (
            "the faucet forgets it instead",
            TX.replacen(
                "      do    set faucet-fixed\n      else  \"Nothing wrong with it now.\"\n",
                "      else  \"Nothing wrong with it now.\"\n",
                2,
            )
            .replacen(
                "      else  \"Nothing wrong with it now.\"\n",
                "      do    set faucet-fixed\n      else  \"Nothing wrong with it now.\"\n",
                1,
            ),
        ),
    ];
    for (what, source) in &drifts {
        assert_ne!(source, TX, "the drift `{what}` did not apply");
        let r = analyse(&load(source));
        assert_eq!(
            (r.states, r.winnable, r.strandings.len()),
            (base.states, base.winnable, base.strandings.len()),
            "`{what}` was expected to be indistinguishable and was not"
        );
    }
}

/// **The seventh shape, and the absence is committed rather than asserted.**
///
/// `Diner.radio`'s `verb_empty` ends by running `Inventory.screwdriver.verb_jab_with()` — a whole
/// electrocution sequence, reused because a verb slot is the only place in that engine to keep a
/// named piece of behaviour. Nothing here can say it. Both plausible spellings are errors, and
/// this test is what would fail if one were quietly added without a decision behind it.
#[test]
fn there_is_no_way_to_run_another_binding() {
    for attempt in [
        "      do    run look-at\n",
        "      do    look-at strange-device\n",
    ] {
        let source = TX.replace("      do    set device-seen\n", attempt);
        assert_ne!(source, TX);
        let (mut g, mut p) = game_format::parse(&source, "delegation.game");
        p.extend(game_format::resolve(&mut g));
        assert!(!p.is_empty(), "`{}` was accepted", attempt.trim());
    }
}
