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

    // `holds <actor>` is `actor -occupies-> room` at rest — the value `move <actor> to <room>`
    // writes over.
    //
    // It took an item too, and that was `inventory-item -found-in-> room`, which is a
    // relationship no author should be asked to write: a hotspot belongs to a room, so where
    // an item is found is the room of the hotspot it is the carried form of. Five ways of
    // writing it wrong were measured — omitted, moved to another room, naming an item nothing
    // stands for, contradicting itself across two rooms — and every one of them produced the
    // same states, the same strandings and the same silence as writing it right. So it is an
    // error now rather than a check, because there is no second statement worth holding in
    // step. See `.yidam/decisions/found-in.yml`.
    let holds = std::mem::take(&mut game.holds);
    for (room, name, line) in &holds {
        let mut n = name.clone();
        match look(&declared, &n, &Scope::Room(room.clone())) {
            Some((canonical, Kind::Actor)) => {
                if let Some(prev) = game.actor_start.insert(canonical.clone(), room.clone()) {
                    if prev != *room {
                        out.push(Problem {
                            line: *line,
                            message: format!(
                                "`{canonical}` is held by {prev} and by {room}; an actor \
                                 starts in one place"
                            ),
                        });
                    }
                }
            }
            Some((canonical, Kind::Item)) => out.push(Problem {
                line: *line,
                message: format!(
                    "a room holds actors, and `{canonical}` is an item — where an item starts \
                     is `is {canonical}` on the hotspot it stands in the room as"
                ),
            }),
            Some((canonical, k)) => out.push(Problem {
                line: *line,
                message: format!("`{canonical}` is a {k}, and a room holds actors"),
            }),
            None => fix(
                &declared,
                &mut n,
                Kind::Actor,
                &Scope::Global,
                *line,
                &mut out,
            ),
        }
    }
    game.holds = holds;

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
            Site::OnEnter { room } => {
                fix(&declared, room, Kind::Room, &Scope::Global, line, &mut out)
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
                Effect::Move(who, x) => {
                    if let Some(a) = who {
                        fix(&declared, a, Kind::Actor, &Scope::Global, line, &mut out);
                    }
                    fix(&declared, x, Kind::Room, &Scope::Global, line, &mut out)
                }
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
    // An actor a predicate asks about must have somewhere to start. This is the one check the
    // pair of primitives needs that neither has on its own: `move <actor> to <room>` writes a
    // value, `<actor> in <room>` reads one, and without a `holds` the read has no initial value
    // to compare against and the answer would be whatever the search happened to default to.
    let mut asked: Vec<String> = Vec::new();
    for b in &game.bindings {
        for q in b.when.iter().chain(b.guards.iter()) {
            collect_asked(q, &mut asked);
        }
        // Being moved counts too: `crates/reachability` gates an actor's site on where the
        // actor is, so an actor a binding moves is followed whether or not anything asks.
        for e in &b.effects {
            if let Effect::Move(Some(a), _) = e {
                if !asked.contains(a) {
                    asked.push(a.clone());
                }
            }
        }
    }
    for a in asked {
        if !game.actor_start.contains_key(&a) {
            let line = game.declared.get(&a).map(|(_, l)| *l).unwrap_or(0);
            out.push(Problem {
                line,
                message: format!(
                    "`{a} in <room>` is asked and no room `holds {a}`, so the question has no \
                     answer before anything moves them"
                ),
            });
        }
    }

    // **`carried-form-of` and `acquired-by` are one fact stated twice, and this is what holds
    // them in step.** `is <item>` says which hotspot the item is when it is not carried, and
    // that is what makes "the object leaves the room" derived rather than an effect; the
    // derivation fires only when a binding *on that hotspot* gives *that item*. Let the two
    // names disagree and the hotspot stays in the room forever while the player carries the
    // thing — and nothing said so. Worse: the checker gets *quieter*. Making `nightwatch`'s
    // wire hand over the ledger instead takes it from fourteen states and two strandings to
    // six and none, still reporting the game winnable. A check whose output improves when the
    // game breaks is the one kind of check worth writing before any other.
    for (hotspot, item) in &game.carried_form {
        let given = game.bindings.iter().any(|b| {
            matches!(&b.site, Site::InRoom { target, .. } if target == hotspot)
                && b.effects
                    .iter()
                    .any(|e| matches!(e, Effect::Give(x, _) if x == item))
        });
        if !given {
            let line = game.declared.get(hotspot).map(|(_, l)| *l).unwrap_or(0);
            out.push(Problem {
                line,
                message: format!(
                    "`{hotspot}` stands for `{item}` and no binding on it gives `{item}`, so \
                     the hotspot would stay in the room whatever the player picked up"
                ),
            });
        }
    }

    dead_writes(game, &mut out);
    unreachable_values(game, &mut out);
    ambiguous_cells(game, &mut out);
    out
}

/// A cell of the interaction table: where a binding sits, the verb, and the second object.
type Cell = (String, String, String);

/// Two bindings written into one cell of the interaction table, where both can hold at once.
///
/// [`data-format/interaction-table`] describes the format as *keyed by verb and target, with the
/// second object as an optional third key*, and until this check the key was a description rather
/// than a rule: a hotspot could carry `use-with key:` twice and nothing said so. When both guards
/// can hold in the same state only one of the two can resolve, the author cannot tell which, and
/// the answer is the order they were typed — which is not something this format promises.
///
/// **Two bindings in one cell are not in themselves a defect**, and refusing them outright would
/// refuse a shape transcribed from a shipped game: `crates/reachability/tests/delegation.rs`
/// carries a `pick-up` guarded on `not device-seen` beside one guarded on `device-seen`, which is
/// a response continuing into a description, and exactly one of the two ever applies. So what is
/// refused is the pair that cannot be told apart.
///
/// Disjointness is decided by looking for a contradicting pair of atoms — a flag asserted and
/// denied, an actor in two rooms, two comparisons on one counter whose satisfying intervals do not
/// meet. It proves disjointness rather than overlap, so a pair it cannot separate is reported; the
/// author separates them by writing the guard that says which is which, and that guard is the
/// thing a second key would have supplied for free.
///
/// [`data-format/interaction-table`]: ../../../.yidam/corpus/data-format/interaction-table.yml
fn ambiguous_cells(game: &Game, out: &mut Vec<Problem>) {
    let mut seen: BTreeMap<Cell, Vec<(usize, Option<Pred>)>> = BTreeMap::new();
    for b in &game.bindings {
        let site = match &b.site {
            Site::InRoom { room, target } => format!("{room}/{target}"),
            Site::InConversation {
                conversation,
                option,
            } => format!("{conversation}/{option}"),
            Site::OnEnter { room } => room.clone(),
        };
        let key = (site, b.verb.clone(), b.object.clone().unwrap_or_default());
        let entry = seen.entry(key.clone()).or_default();
        if let Some((first, guard)) = entry
            .iter()
            .find(|(_, g)| !disjoint(g.as_ref(), b.when.as_ref()))
        {
            out.push(Problem {
                line: b.line,
                message: format!(
                    "`{}{}` is already written on `{}` at line {first} and nothing tells the two \
                     apart — {}, so only one of them can ever resolve and which one \
                     is the order they were typed in",
                    b.verb,
                    b.object
                        .as_ref()
                        .map(|o| format!(" {o}"))
                        .unwrap_or_default(),
                    key.0,
                    match (guard, &b.when) {
                        (None, None) => "neither is guarded".to_string(),
                        (None, Some(_)) | (Some(_), None) => "one of them is unguarded".to_string(),
                        _ => "both guards can hold in the same state".to_string(),
                    },
                ),
            });
        }
        entry.push((b.line, b.when.clone()));
    }
}

/// The atoms of a guard, with `or` refused rather than approximated.
///
/// A disjunction is two guards wearing one name, and separating a pair of them wants the cross
/// product. Nothing in the four specimens writes one in a cell that collides, so this returns
/// `None` and the caller treats the pair as inseparable — which is the safe direction: it asks the
/// author for a guard rather than accepting one it did not understand.
fn atoms(p: &Pred, out: &mut Vec<(bool, Pred)>, sign: bool) -> bool {
    match p {
        Pred::And(a, b) => atoms(a, out, sign) && atoms(b, out, sign),
        Pred::Not(x) => atoms(x, out, !sign),
        Pred::Or(_, _) => false,
        atom => {
            out.push((sign, atom.clone()));
            true
        }
    }
}

/// The set of values a comparison admits, as an inclusive interval over `i64`.
fn interval(c: Cmp, n: i64) -> (i64, i64) {
    match c {
        Cmp::Eq => (n, n),
        Cmp::Ne => (i64::MIN, i64::MAX),
        Cmp::Lt => (i64::MIN, n - 1),
        Cmp::Le => (i64::MIN, n),
        Cmp::Gt => (n + 1, i64::MAX),
        Cmp::Ge => (n, i64::MAX),
    }
}

/// Can these two guards be shown never to hold together?
fn disjoint(a: Option<&Pred>, b: Option<&Pred>) -> bool {
    let (Some(a), Some(b)) = (a, b) else {
        return false;
    };
    let (mut xs, mut ys) = (vec![], vec![]);
    if !atoms(a, &mut xs, true) || !atoms(b, &mut ys, true) {
        return false;
    }
    for (sx, x) in &xs {
        for (sy, y) in &ys {
            let contradicts = match (x, y) {
                (Pred::Flag(p), Pred::Flag(q))
                | (Pred::Carrying(p), Pred::Carrying(q))
                | (Pred::Chosen(p), Pred::Chosen(q)) => p == q && sx != sy,
                (Pred::At(p, r), Pred::At(q, s)) => p == q && sx == sy && *sx && r != s,
                (Pred::Cmp(p, c, m), Pred::Cmp(q, d, n))
                | (Pred::Count(p, c, m), Pred::Count(q, d, n)) => {
                    let (lo1, hi1) = interval(*c, *m);
                    let (lo2, hi2) = interval(*d, *n);
                    p == q && *sx && *sy && (lo1.max(lo2) > hi1.min(hi2))
                }
                _ => false,
            };
            if contradicts {
                return true;
            }
        }
    }
    false
}

/// A comparison no value the counter can actually hold will ever satisfy.
///
/// The declared range is already checked above — `lamp-prep = 7` against `0..3` is refused. What
/// that check cannot see is that **a counter's range is declared and its steps are not**. A
/// staged puzzle is written as a run of bindings, each guarded on the value the last one wrote,
/// and the agreement between the write and the read is maintained by the author and by nothing
/// else. `puzzle/state-gate` carried the general form of this as the strongest live case for
/// adding a primitive.
///
/// This is the narrower thing the specimens actually want, and it needs no new syntax: the steps
/// are already implicit in the bindings, and the set of values a counter can hold is its
/// starting value plus everything any `set` writes. A guard selecting outside that set is text
/// no player can reach — and the mutation that produces it is the likeliest one, because writing
/// the wrong next value makes the *report improve*. Changing lighthouse's wick step to write 2
/// instead of 1 removes every stranding the specimen has and leaves the ending reachable.
fn unreachable_values(game: &Game, out: &mut Vec<Problem>) {
    for c in &game.counters {
        let mut holdable: Vec<i64> = vec![c.lo];
        for b in &game.bindings {
            for e in &b.effects {
                if let Effect::SetCounter(x, n) = e {
                    if *x == c.name && !holdable.contains(n) {
                        holdable.push(*n);
                    }
                }
            }
        }
        let mut report = |line: usize, cmp: Cmp, v: i64| {
            if holdable.iter().any(|h| cmp.holds(*h, v)) {
                return;
            }
            let mut seen: Vec<String> = holdable.iter().map(|h| h.to_string()).collect();
            seen.sort();
            out.push(Problem {
                line,
                message: format!(
                    "no value `{}` can hold satisfies `{} {} {v}` — it starts at {} and is only ever set to {}, so this is a branch the game cannot reach",
                    c.name,
                    c.name,
                    cmp.spelling(),
                    c.lo,
                    seen.join(", ")
                ),
            });
        };
        for b in &game.bindings {
            let mut walk = |p: &Pred| {
                let mut stack = vec![p];
                while let Some(q) = stack.pop() {
                    match q {
                        Pred::Cmp(x, cmp, v) if *x == c.name => report(b.line, *cmp, *v),
                        Pred::Not(a) => stack.push(a),
                        Pred::And(a, b) | Pred::Or(a, b) => {
                            stack.push(a);
                            stack.push(b);
                        }
                        _ => {}
                    }
                }
            };
            if let Some(w) = &b.when {
                walk(w);
            }
            for g in &b.guards {
                walk(g);
            }
        }
    }
}

fn collect_asked(q: &Pred, into: &mut Vec<String>) {
    match q {
        Pred::At(a, _) => {
            if !into.contains(a) {
                into.push(a.clone())
            }
        }
        Pred::Not(x) => collect_asked(x, into),
        Pred::And(x, y) | Pred::Or(x, y) => {
            collect_asked(x, into);
            collect_asked(y, into);
        }
        _ => {}
    }
}

/// What part of the state an effect writes in full. Two effects naming the same slot in one
/// list means the earlier one cannot be observed: the list is a fold applied between two
/// player-visible states, so nothing reads the value in between.
///
/// `Give` and `Spend` are deliberately absent. They accumulate rather than overwrite, so
/// `give coin 3` followed by `spend coin 1` is a net change and not a dead write.
fn slot_written(e: &Effect) -> Option<String> {
    match e {
        Effect::SetFlag(x) | Effect::ClearFlag(x) => Some(format!("flag {x}")),
        Effect::SetCounter(x, _) => Some(format!("counter {x}")),
        Effect::Move(None, _) => Some("the player's room".into()),
        Effect::Move(Some(a), _) => Some(format!("{a}'s room")),
        Effect::Enter(_) | Effect::Leave => Some("the current mode".into()),
        Effect::Give(..) | Effect::Spend(..) => None,
    }
}

fn render(e: &Effect) -> String {
    match e {
        Effect::SetFlag(x) => format!("set {x}"),
        Effect::ClearFlag(x) => format!("clear {x}"),
        Effect::SetCounter(x, v) => format!("set {x} {v}"),
        Effect::Give(x, q) => format!("give {x} {q}"),
        Effect::Spend(x, q) => format!("spend {x} {q}"),
        Effect::Move(None, x) => format!("move {x}"),
        Effect::Move(Some(a), x) => format!("move {a} to {x}"),
        Effect::Enter(x) => format!("enter {x}"),
        Effect::Leave => "leave".into(),
    }
}

/// A write no one can read. This check exists *because* of the decision recorded in
/// `.yidam/decisions/effect-list-semantics.yml`: an effect list is one atomic transition, so
/// there is no moment between two effects at which the first value is visible. Under any
/// other reading — a list that yields, or one an author can step through — an overwritten
/// write might be something the player sees, and this would be a false positive.
fn dead_writes(game: &Game, out: &mut Vec<Problem>) {
    for b in &game.bindings {
        for (i, e) in b.effects.iter().enumerate() {
            let Some(slot) = slot_written(e) else {
                continue;
            };
            if let Some(later) = b.effects[i + 1..]
                .iter()
                .find(|f| slot_written(f).as_deref() == Some(slot.as_str()))
            {
                out.push(Problem {
                    line: b.line,
                    message: format!(
                        "`{}` is overwritten by `{}` in the same list — both write {slot}, \
                         and nothing reads it in between",
                        render(e),
                        render(later)
                    ),
                });
            }
        }
    }
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
        Pred::At(a, r) => {
            fix(declared, a, Kind::Actor, &Scope::Global, line, out);
            fix(declared, r, Kind::Room, &Scope::Global, line, out);
        }
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
