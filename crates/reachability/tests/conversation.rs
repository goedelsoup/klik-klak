//! **Is `conversation` a primitive, or a response type?**
//!
//! [`interaction/conversational-binding`] holds one side of what it calls this corpus's most
//! consequential open question, and has held it unargued since genesis: *"If a binding's response
//! may be 'enter this conversation', then `conversation` is a response type rather than a
//! primitive, and the vocabulary is one class smaller than currently drawn. If it is not, the
//! reason has to be stated, and the only candidate reason this corpus has found is that a
//! conversation's available choices depend on the path taken through it, which no binding
//! records."*
//!
//! That candidate reason is refuted by the specimen it was written beside. `lighthouse.game`
//! records the path with ordinary machinery — `flag asked-key in keeper-talk` is a scoped world
//! flag and `option wick-ask: when chosen lamp and not asked-key` reads it — so the corpus already
//! had its answer, in the same file, the way [`binary-dimension`] did.
//!
//! So the reduction is run instead of argued, on the model's own terms: a conversation rewritten
//! as an ordinary room, options as hotspots, `enter` as `move`, `leave` as `move <the room you
//! came from>`, `chosen` as a latched flag. It is exact. What it cannot do is the last clause, and
//! that is where the class survives. `.yidam/decisions/conversation-reduction.yml`.
//!
//! [`interaction/conversational-binding`]: ../../../.yidam/corpus/interaction/conversational-binding.yml
//! [`binary-dimension`]: ../../../.yidam/decisions/binary-dimension.yml

use reachability::analyse;

const LIGHTHOUSE: &str = include_str!("../../../design/specimen/lighthouse.game");

fn load(src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, "conversation.game");
    p.extend(game_format::resolve(&mut g));
    let m: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(m.is_empty(), "{m:#?}");
    g
}

/// The rewrite, done to the text rather than the model, so what is being tested is a game an
/// author could have written rather than a transform only the checker can see.
fn as_a_room(src: &str) -> String {
    src.replace(
        "room    lamp-room   \"The Lamp Room\"",
        "room    lamp-room   \"The Lamp Room\"\nroom    keeper-talk \"Talking to the keeper\"",
    )
    .replace(
        "flag    asked-key   in keeper-talk \"the keeper has been asked for the key\"",
        "flag    asked-key   \"the keeper has been asked for the key\"\n\
         flag    said-lamp   latched \"the lamp has been mentioned\"",
    )
    .replace("conversation keeper-talk with keeper", "room keeper-talk")
    .replace(
        "  option lamp \"The lamp is out.\"",
        "  hotspot lamp \"The lamp is out.\"\n    talk-to:",
    )
    .replace(
        "  option wick-ask \"Could I take the wick up myself?\"",
        "  hotspot wick-ask \"Could I take the wick up myself?\"\n    talk-to:",
    )
    .replace(
        "  option leave \"Never mind.\"",
        "  hotspot leave \"Never mind.\"\n    talk-to:",
    )
    .replace("    when  not chosen lamp", "      when  not said-lamp")
    .replace(
        "    when  chosen lamp and not asked-key",
        "      when  said-lamp and not asked-key",
    )
    .replace(
        "      do    enter keeper-talk",
        "      do    move keeper-talk",
    )
    .replace("    do    leave", "      do    move quay")
    .replace(
        "    say   \"Aye. Three nights now. I've not the hands for that stair any more.\"\n\
         \x20   do    give wick",
        "      say   \"Aye. Three nights now. I've not the hands for that stair any more.\"\n\
         \x20     do    give wick\n      do    set said-lamp",
    )
    .replace(
        "    say   \"If you can get through the door. Key's under the coil of rope.\"\n\
         \x20   do    give brass-key\n    do    set asked-key",
        "      say   \"If you can get through the door. Key's under the coil of rope.\"\n\
         \x20     do    give brass-key\n      do    set asked-key",
    )
    .replace("    say   \"Mm.\"", "      say   \"Mm.\"")
}

/// **The reduction is exact.** Every figure, on the one specimen that holds a conversation.
///
/// `mode` is `room` with a return address; `Effect::Enter` is `Effect::Move`; `Effect::Leave` is
/// `Effect::Move` back; `chosen` is a latched flag the option sets for itself. None of that is a
/// claim about what a conversation *is* — it is a claim that the checker cannot tell the two apart,
/// which is the same standing as `arity.rs`'s result about the second key.
#[test]
fn a_conversation_rewritten_as_a_room_analyses_identically() {
    let rewritten = as_a_room(LIGHTHOUSE);
    assert_ne!(rewritten, LIGHTHOUSE, "the rewrite must apply");
    let survives = rewritten
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .any(|l| l.starts_with("conversation ") || l.trim_start().starts_with("option "));
    assert!(!survives, "no conversation construct survives the rewrite");

    let (a, b) = (analyse(&load(LIGHTHOUSE)), analyse(&load(&rewritten)));
    assert_eq!(
        (a.states, a.transitions, a.winnable, a.strandings.len()),
        (b.states, b.transitions, b.winnable, b.strandings.len())
    );
    assert_eq!((a.states, a.transitions), (91, 173));
}

/// **What the rewrite cannot do, and it is the whole of what survives.**
///
/// `do leave` does not name a room, and the rewrite had to write `do move quay` — correct only
/// because this exchange is entered from one place. Enter it from two and the room form needs one
/// exit per entrance, and **the copies are not merely duplication: they are wrong.** Nothing keys
/// an exit to the door you came in by, so from room `a` the player may leave the exchange into
/// room `b`. The room form has *fewer* states than the conversation form because it has lost the
/// entrance, and it is a different game.
///
/// A conversation's mode sits **beside** the room rather than replacing it, which is what carries
/// the return. To write that as rooms an author has to record the entrance in world state by hand,
/// once per door — the shape `.yidam/decisions/arrival-trigger.yml` admitted `on-enter` for.
///
/// Prior art keeps the mode the same way: `startYack(actor1, actor2, filename)` takes no room and
/// moves nobody ([delores-source](../../../.yidam/catalog/delores-source.md)).
#[test]
fn a_room_cannot_hold_the_return_and_a_mode_can() {
    const DECL: &str =
        "room a \"A\"\nroom b \"B\"\nactor sam \"Sam\"\nflag done latched \"done\"\n";

    let conversation = format!(
        "{DECL}conversation chat with sam\n  option hello \"Hello.\"\n    do    set done\n\
         \x20 option bye \"Bye.\"\n    do    leave\n\n\
         room a\n  start\n  holds sam\n  hotspot sam-a \"Sam\"\n    talk-to:\n      do    enter chat\n\
         \x20 hotspot door \"a door\"\n    walk-to:\n      do    move b\n\n\
         room b\n  hotspot sam-b \"Sam\"\n    talk-to:\n      do    enter chat\n\
         \x20 hotspot out \"out\"\n    walk-to:\n      when  done\n      ending\n      else  \"Not yet.\"\n"
    );
    let rooms = format!(
        "{DECL}room chat \"Talking to Sam\"\n\nroom chat\n\
         \x20 hotspot hello \"Hello.\"\n    talk-to:\n      do    set done\n\
         \x20 hotspot bye-a \"Bye.\"\n    talk-to:\n      do    move a\n\
         \x20 hotspot bye-b \"Bye.\"\n    talk-to:\n      do    move b\n\n\
         room a\n  start\n  holds sam\n  hotspot sam-a \"Sam\"\n    talk-to:\n      do    move chat\n\
         \x20 hotspot door \"a door\"\n    walk-to:\n      do    move b\n\n\
         room b\n  hotspot sam-b \"Sam\"\n    talk-to:\n      do    move chat\n\
         \x20 hotspot out \"out\"\n    walk-to:\n      when  done\n      ending\n      else  \"Not yet.\"\n"
    );

    let (c, r) = (load(&conversation), load(&rooms));
    let exits =
        |g: &game_format::Game| {
            g.bindings
            .iter()
            .filter(|b| {
                matches!(&b.site, game_format::Site::InConversation { .. })
                    || matches!(&b.site, game_format::Site::InRoom { room, .. } if room == "chat")
            })
            .filter(|b| {
                b.effects.iter().any(|e| {
                    matches!(e, game_format::Effect::Leave | game_format::Effect::Move(None, _))
                })
            })
            .count()
        };
    assert_eq!(exits(&c), 1, "one way out, whichever door you came in by");
    assert_eq!(exits(&r), 2, "one per entrance, each naming its own room");

    // and the copies are wrong rather than merely repeated: the room form loses the entrance, so
    // the exchange becomes a passage between the two rooms
    let (a, b) = (analyse(&c), analyse(&r));
    assert!(a.winnable && b.winnable);
    assert!(
        b.states < a.states,
        "the room form has lost the entrance, so it has fewer states: {} against {}",
        b.states,
        a.states
    );
    assert_eq!((a.states, b.states), (8, 6));
}

/// **The reason the class was founded is not the reason it survives**, which is worth pinning
/// because the description in `conversation.ont.yml` says otherwise.
///
/// That class calls itself irreducible *"because of that carried availability and that modality —
/// not because of branching"*. Availability is not carried: `option wick-ask` re-reads `chosen
/// lamp and not asked-key` at display time, and the rewrite above turns both into ordinary flags
/// with no figure moving. What is left is the return.
#[test]
fn availability_is_read_and_not_carried() {
    let g = load(LIGHTHOUSE);
    let guarded = g
        .bindings
        .iter()
        .filter(|b| matches!(&b.site, game_format::Site::InConversation { .. }))
        .filter(|b| b.when.is_some())
        .count();
    assert_eq!(
        guarded, 2,
        "two of the three options are gated by a predicate"
    );

    // nothing in the language writes an option's availability; there is no `disable` effect
    let writes_availability = g.bindings.iter().any(|b| {
        b.effects.iter().any(|e| {
            !matches!(
                e,
                game_format::Effect::SetFlag(_)
                    | game_format::Effect::ClearFlag(_)
                    | game_format::Effect::SetCounter(_, _)
                    | game_format::Effect::Give(_, _)
                    | game_format::Effect::Spend(_, _)
                    | game_format::Effect::Move(_, _)
                    | game_format::Effect::Enter(_)
                    | game_format::Effect::Leave
            )
        })
    });
    assert!(
        !writes_availability,
        "the effect vocabulary is closed and holds no `disable`"
    );
}
