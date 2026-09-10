//! The admission rule, checked.
//!
//! *"An operand must name something the corpus declares."* That is what keeps the predicate
//! layer finite — there are no variables and no binding forms, so every name in a game has
//! to be one the game itself declared. This checks it, and checks the **kind**: the shell
//! gate this replaced matched a name against one flat set, so `carrying quay` and `move
//! brass-key` both passed.

use crate::*;

pub fn resolve(game: &Game) -> Vec<Problem> {
    let mut out = Vec::new();
    let mut want =
        |name: &str, kind: Kind, line: usize, out: &mut Vec<Problem>| match game.declared.get(name)
        {
            None => out.push(Problem {
                line,
                message: format!("`{name}` is not declared (wanted a {kind})"),
            }),
            Some((k, _)) if *k != kind => out.push(Problem {
                line,
                message: format!("`{name}` is a {k} here, and a {kind} is wanted"),
            }),
            Some(_) => {}
        };

    if game.start_room.is_none() {
        out.push(Problem {
            line: 0,
            message: "no room is marked `start`".into(),
        });
    }
    if !game.bindings.iter().any(|b| b.ending) {
        out.push(Problem {
            line: 0,
            message: "no binding is marked `ending`".into(),
        });
    }

    for f in &game.flags {
        if let Some(scope) = &f.scope {
            want(scope, Kind::Conversation, f.line, &mut out);
        }
    }
    for (hotspot, item) in &game.carried_form {
        let line = game.declared.get(hotspot).map(|(_, l)| *l).unwrap_or(0);
        want(hotspot, Kind::Hotspot, line, &mut out);
        want(item, Kind::Item, line, &mut out);
    }

    for b in &game.bindings {
        let line = b.line;
        match &b.site {
            Site::InRoom { room, target } => {
                want(room, Kind::Room, line, &mut out);
                // a site is a hotspot or an actor, and nothing else
                match game.declared.get(target) {
                    Some((Kind::Hotspot, _)) | Some((Kind::Actor, _)) => {}
                    Some((k, _)) => out.push(Problem {
                        line,
                        message: format!(
                            "`{target}` is a {k}, and a binding needs a hotspot or an actor"
                        ),
                    }),
                    None => out.push(Problem {
                        line,
                        message: format!("`{target}` is not declared"),
                    }),
                }
            }
            Site::InConversation {
                conversation,
                option,
            } => {
                want(conversation, Kind::Conversation, line, &mut out);
                want(option, Kind::Option_, line, &mut out);
            }
        }
        if let Some(o) = &b.object {
            want(o, Kind::Item, line, &mut out);
        }
        for q in b.when.iter().chain(b.guards.iter()) {
            check_pred(q, game, line, &mut out, &mut want);
        }
        for e in &b.effects {
            match e {
                Effect::SetFlag(x) | Effect::ClearFlag(x) => want(x, Kind::Flag, line, &mut out),
                Effect::SetCounter(x, v) => {
                    want(x, Kind::Counter, line, &mut out);
                    if let Some(c) = game.counter(x) {
                        if *v < c.lo || *v > c.hi {
                            out.push(Problem {
                                line,
                                message: format!(
                                    "`set {x} {v}` is outside the declared range {}..{}",
                                    c.lo, c.hi
                                ),
                            });
                        }
                    }
                }
                Effect::Give(x) | Effect::Spend(x) => want(x, Kind::Item, line, &mut out),
                Effect::Move(x) => want(x, Kind::Room, line, &mut out),
                Effect::Enter(x) => want(x, Kind::Conversation, line, &mut out),
                Effect::Leave => {}
            }
        }
    }
    out
}

fn check_pred(
    q: &Pred,
    game: &Game,
    line: usize,
    out: &mut Vec<Problem>,
    want: &mut impl FnMut(&str, Kind, usize, &mut Vec<Problem>),
) {
    match q {
        Pred::Flag(x) => want(x, Kind::Flag, line, out),
        Pred::Carrying(x) => want(x, Kind::Item, line, out),
        Pred::Chosen(x) => want(x, Kind::Option_, line, out),
        Pred::Cmp(x, _, v) => {
            want(x, Kind::Counter, line, out);
            if let Some(c) = game.counter(x) {
                if *v < c.lo || *v > c.hi {
                    out.push(Problem {
                        line,
                        message: format!(
                            "`{x}` is compared against {v}, outside its declared range {}..{}",
                            c.lo, c.hi
                        ),
                    });
                }
            }
        }
        Pred::Not(a) => check_pred(a, game, line, out, want),
        Pred::And(a, b) | Pred::Or(a, b) => {
            check_pred(a, game, line, out, want);
            check_pred(b, game, line, out, want);
        }
    }
}
