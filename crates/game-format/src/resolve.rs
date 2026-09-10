//! The admission rule, checked — and every reference rewritten to its canonical name.
//!
//! *"An operand must name something the corpus declares."* That is what keeps the predicate
//! layer finite: there are no variables and no binding forms, so every name in a game has to
//! be one the game itself declared. This checks it, and checks two things a flat gate could
//! not.
//!
//! **Kind.** The shell gate this replaced matched a name against one flat set, so `carrying
//! quay` and `move brass-key` both passed.
//!
//! **Scope.** A name is declared in the scope of the thing that contains it — a hotspot in
//! its room, an option in its exchange — and a bare reference means the local one, falling
//! back to the world. That is the ontology's containment read as a namespace rather than a
//! rule invented for the format. After this pass every name in the model is canonical, so
//! everything downstream can go on treating names as unique strings.

use crate::*;

type Declared = BTreeMap<String, (Kind, usize)>;
type Bounds = BTreeMap<String, (i64, i64)>;

fn look(declared: &Declared, name: &str, from: &Scope) -> Option<(String, Kind)> {
    if name.contains('.') {
        return declared.get(name).map(|(k, _)| (name.to_string(), *k));
    }
    let local = from.qualify(name);
    if let Some((k, _)) = declared.get(&local) {
        return Some((local, *k));
    }
    declared.get(name).map(|(k, _)| (name.to_string(), *k))
}

/// Resolve `name` to its canonical form, or record why it cannot be.
fn fix(
    declared: &Declared,
    name: &mut String,
    kind: Kind,
    from: &Scope,
    line: usize,
    out: &mut Vec<Problem>,
) {
    match look(declared, name, from) {
        None => out.push(Problem {
            line,
            message: match from.qualifier() {
                Some(q) => {
                    format!("`{name}` is not declared, in {q} or in the world (wanted a {kind})")
                }
                None => format!("`{name}` is not declared (wanted a {kind})"),
            },
        }),
        Some((canonical, k)) if k != kind => out.push(Problem {
            line,
            message: format!("`{canonical}` is a {k} here, and a {kind} is wanted"),
        }),
        Some((canonical, _)) => *name = canonical,
    }
}

pub fn resolve(game: &mut Game) -> Vec<Problem> {
    let declared: Declared = game.declared.clone();
    let bounds: Bounds = game
        .counters
        .iter()
        .map(|c| (c.name.clone(), (c.lo, c.hi)))
        .collect();
    let mut out = Vec::new();

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

    for f in &mut game.flags {
        if let Some(scope) = &mut f.scope {
            fix(
                &declared,
                scope,
                Kind::Conversation,
                &Scope::Global,
                f.line,
                &mut out,
            );
        }
    }

    // a hotspot's key is already canonical; the item it stands for is a world name
    let carried: Vec<(String, String)> = game
        .carried_form
        .iter()
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    game.carried_form.clear();
    for (hotspot, item) in carried {
        let line = declared.get(&hotspot).map(|(_, l)| *l).unwrap_or(0);
        let mut item = item;
        fix(
            &declared,
            &mut item,
            Kind::Item,
            &Scope::Global,
            line,
            &mut out,
        );
        match declared.get(&hotspot) {
            Some((Kind::Hotspot, _)) => {}
            _ => out.push(Problem {
                line,
                message: format!("`{hotspot}` is not a hotspot and cannot stand for an item"),
            }),
        }
        game.carried_form.insert(hotspot, item);
    }

    for b in &mut game.bindings {
        let line = b.line;
        let scope = b.scope.clone();
        match &mut b.site {
            Site::InRoom { room, target } => {
                fix(&declared, room, Kind::Room, &Scope::Global, line, &mut out);
                match look(&declared, target, &scope) {
                    Some((canonical, Kind::Hotspot)) | Some((canonical, Kind::Actor)) => {
                        *target = canonical
                    }
                    Some((canonical, k)) => out.push(Problem {
                        line,
                        message: format!(
                            "`{canonical}` is a {k}, and a binding needs a hotspot or an actor"
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
                fix(
                    &declared,
                    conversation,
                    Kind::Conversation,
                    &Scope::Global,
                    line,
                    &mut out,
                );
                fix(&declared, option, Kind::Option_, &scope, line, &mut out);
            }
        }
        if let Some(o) = &mut b.object {
            fix(&declared, o, Kind::Item, &scope, line, &mut out);
        }
        let mut preds: Vec<&mut Pred> = b.when.iter_mut().collect();
        preds.extend(b.guards.iter_mut());
        for q in preds {
            walk_pred(q, &declared, &scope, line, &mut out, &bounds);
        }
        for e in &mut b.effects {
            match e {
                Effect::SetFlag(x) | Effect::ClearFlag(x) => {
                    fix(&declared, x, Kind::Flag, &scope, line, &mut out)
                }
                Effect::SetCounter(x, v) => {
                    fix(&declared, x, Kind::Counter, &scope, line, &mut out);
                    if let Some((lo, hi)) = bounds.get(x.as_str()).copied() {
                        if *v < lo || *v > hi {
                            out.push(Problem {
                                line,
                                message: format!(
                                    "`set {x} {v}` is outside the declared range {lo}..{hi}"
                                ),
                            });
                        }
                    }
                }
                Effect::Give(x, n) | Effect::Spend(x, n) => {
                    fix(&declared, x, Kind::Item, &scope, line, &mut out);
                    if *n < 1 {
                        out.push(Problem {
                            line,
                            message: format!("a quantity of {n} is not a quantity"),
                        });
                    }
                }
                Effect::Move(x) => fix(&declared, x, Kind::Room, &Scope::Global, line, &mut out),
                Effect::Enter(x) => fix(
                    &declared,
                    x,
                    Kind::Conversation,
                    &Scope::Global,
                    line,
                    &mut out,
                ),
                Effect::Leave => {}
            }
        }
    }
    out
}

fn walk_pred(
    q: &mut Pred,
    declared: &Declared,
    scope: &Scope,
    line: usize,
    out: &mut Vec<Problem>,
    bounds: &Bounds,
) {
    match q {
        Pred::Flag(x) => fix(declared, x, Kind::Flag, scope, line, out),
        Pred::Carrying(x) => fix(declared, x, Kind::Item, scope, line, out),
        Pred::Chosen(x) => fix(declared, x, Kind::Option_, scope, line, out),
        Pred::Count(x, _, n) => {
            fix(declared, x, Kind::Item, scope, line, out);
            if *n < 0 {
                out.push(Problem {
                    line,
                    message: format!("`count {x}` cannot be compared against {n}"),
                });
            }
        }
        Pred::Cmp(x, _, v) => {
            fix(declared, x, Kind::Counter, scope, line, out);
            if let Some((lo, hi)) = bounds.get(x.as_str()).copied() {
                if *v < lo || *v > hi {
                    out.push(Problem {
                        line,
                        message: format!(
                            "`{x}` is compared against {v}, outside its declared range {lo}..{hi}"
                        ),
                    });
                }
            }
        }
        Pred::Not(a) => walk_pred(a, declared, scope, line, out, bounds),
        Pred::And(a, b) | Pred::Or(a, b) => {
            walk_pred(a, declared, scope, line, out, bounds);
            walk_pred(b, declared, scope, line, out, bounds);
        }
    }
}
