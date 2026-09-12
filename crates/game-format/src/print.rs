//! Write a [`Game`] back out as a `.game` file.
//!
//! **This exists to be read back in.** The reader now accounts for every token on every line
//! (`.yidam/decisions/parser-silence.yml`); whether the *tree* holds everything those tokens
//! said is a different question, and it has never been asked. Three fields were added to `Game`
//! in the last seven phases and every one arrived because a consumer noticed something missing —
//! labels and response text when a menu and a presenter wanted them, `spoken_with` when a menu
//! asked who a conversation was held with. Each was found from the outside, by accident.
//!
//! `parse(print(parse(src))) == parse(src)` asks it directly, and it is the whole point of the
//! module: a distinction the file makes that this cannot reproduce is a distinction the tree
//! does not hold.
//!
//! It is not a formatter. It writes no comments, no blank lines and no alignment, because none of
//! those is in the tree — and that is the answer rather than a limitation.

use crate::*;
use std::fmt::Write as _;

/// The name as an author wrote it, inside the section that qualifies it.
fn bare<'a>(name: &'a str, scope: &Scope) -> &'a str {
    match scope.qualifier() {
        Some(q) => name
            .strip_prefix(q)
            .and_then(|r| r.strip_prefix('.'))
            .unwrap_or(name),
        None => name,
    }
}

fn quoted(s: &str) -> String {
    format!("\"{s}\"")
}

fn cmp(c: Cmp) -> &'static str {
    match c {
        Cmp::Eq => "=",
        Cmp::Ne => "!=",
        Cmp::Lt => "<",
        Cmp::Le => "<=",
        Cmp::Gt => ">",
        Cmp::Ge => ">=",
    }
}

/// A predicate, written so the reader's precedence puts it back the way it was.
///
/// `or` binds loosest, then `and`, then `not`, and the language has no parentheses — so the only
/// shapes writable are a disjunction of conjunctions of literals.
///
/// **[`Pred::normal`] is what makes this total.** `(a or b) and c` is a tree this type admits and
/// the reader cannot produce; it is written `a and c or b and c`, which is the same formula in
/// the form the grammar spells. Every propositional formula has one, so there is no predicate
/// this cannot write. The assertion below is an invariant of `normal` rather than a limit of the
/// language, and `.yidam/decisions/normal-form.yml` is the difference between those two things.
fn pred(p: &Pred) -> String {
    let p = &p.normal();
    fn at(p: &Pred, level: u8) -> String {
        let (s, mine) = match p {
            Pred::Flag(f) => (f.clone(), 3),
            Pred::Carrying(i) => (format!("carrying {i}"), 3),
            Pred::Chosen(o) => (format!("chosen {o}"), 3),
            Pred::Cmp(n, c, v) => (format!("{n} {} {v}", cmp(*c)), 3),
            Pred::Count(i, c, v) => (format!("count {i} {} {v}", cmp(*c)), 3),
            Pred::At(a, r) => (format!("{a} in {r}"), 3),
            Pred::Not(q) => (format!("not {}", at(q, 3)), 2),
            Pred::And(a, b) => (format!("{} and {}", at(a, 1), at(b, 1)), 1),
            Pred::Or(a, b) => (format!("{} or {}", at(a, 0), at(b, 0)), 0),
        };
        assert!(
            mine >= level,
            "`Pred::normal` returned a shape the grammar cannot spell, which is a defect in it \
             and not in the language: {s}"
        );
        s
    }
    at(p, 0)
}

fn effect(e: &Effect) -> String {
    match e {
        Effect::SetFlag(f) => format!("set {f}"),
        Effect::ClearFlag(f) => format!("clear {f}"),
        Effect::SetCounter(c, v) => format!("set {c} {v}"),
        Effect::Give(i, 1) => format!("give {i}"),
        Effect::Give(i, n) => format!("give {i} {n}"),
        Effect::Spend(i, 1) => format!("spend {i}"),
        Effect::Spend(i, n) => format!("spend {i} {n}"),
        Effect::Move(None, r) => format!("move {r}"),
        Effect::Move(Some(a), r) => format!("move {a} to {r}"),
        Effect::Enter(c) => format!("enter {c}"),
        Effect::Leave => "leave".to_string(),
    }
}

/// The body of one binding: guard, lines, effects, ending, refusals — in the order the reader
/// accepts them and `flush` closes them.
fn body(out: &mut String, b: &Binding) {
    if let Some(q) = &b.when {
        let _ = writeln!(out, "      when  {}", pred(q));
    }
    for r in &b.says {
        match &r.when {
            Some(q) => {
                let _ = writeln!(out, "      when {} {}", pred(q), quoted(&r.text));
            }
            None => {
                let _ = writeln!(out, "      say   {}", quoted(&r.text));
            }
        }
    }
    for e in &b.effects {
        let _ = writeln!(out, "      do    {}", effect(e));
    }
    if b.ending {
        let _ = writeln!(out, "      ending");
    }
    if b.refuses && b.refusals.is_empty() {
        let _ = writeln!(out, "      else");
    }
    for r in &b.refusals {
        match &r.when {
            Some(q) => {
                let _ = writeln!(out, "      else when {} {}", pred(q), quoted(&r.text));
            }
            None => {
                let _ = writeln!(out, "      else  {}", quoted(&r.text));
            }
        }
    }
}

/// Every declaration of one kind, in the order it was written.
///
/// `declared` is keyed by canonical name and so iterates alphabetically; the line it carries is
/// what puts an author's order back. Order is information here — `rooms`, `items` and `bindings`
/// are all `Vec`s the reader fills in source order, and a round trip that reordered them would be
/// comparing a different game.
fn written(game: &Game, kind: Kind) -> Vec<&String> {
    let mut out: Vec<(usize, &String)> = game
        .declared
        .iter()
        .filter(|(_, (k, _))| *k == kind)
        .map(|(n, (_, line))| (*line, n))
        .collect();
    out.sort();
    out.into_iter().map(|(_, n)| n).collect()
}

/// The `.game` source for a game, readable by [`crate::parse`].
pub fn print(game: &Game) -> String {
    let mut out = String::new();
    let label = |name: &str| {
        game.labels
            .get(name)
            .map(|l| format!(" {}", quoted(l)))
            .unwrap_or_default()
    };

    // ── declarations, in the order they were written ──
    let mut top: Vec<(usize, String)> = Vec::new();
    for name in written(game, Kind::Actor) {
        let line = game.declared[name].1;
        top.push((line, format!("actor {name}{}", label(name))));
    }
    for name in written(game, Kind::Item) {
        let line = game.declared[name].1;
        let cap = game.item_cap.get(name).copied().unwrap_or(1);
        let range = if cap > 1 {
            format!(" 0..{cap}")
        } else {
            String::new()
        };
        top.push((line, format!("item {name}{range}{}", label(name))));
    }
    for f in &game.flags {
        let latched = if f.latched { " latched" } else { "" };
        let scope = f
            .scope
            .as_ref()
            .map(|c| format!(" in {c}"))
            .unwrap_or_default();
        let owner = match &f.scope {
            Some(c) => Scope::Conversation(c.clone()),
            None => Scope::Global,
        };
        top.push((
            f.line,
            format!(
                "flag {}{latched}{scope}{}",
                bare(&f.name, &owner),
                label(&f.name)
            ),
        ));
    }
    for c in &game.counters {
        top.push((
            c.line,
            format!("counter {} {}..{}{}", c.name, c.lo, c.hi, label(&c.name)),
        ));
    }
    for name in written(game, Kind::Room) {
        // only a room carrying a label was declared before its section; one without was not
        // declared at all, and writing `room <name> ""` would invent a label the author did not
        // give. The section below is the whole of such a room's declaration.
        if game.labels.contains_key(name) {
            top.push((game.declared[name].1, format!("room {name}{}", label(name))));
        }
    }
    top.sort();
    for (_, line) in &top {
        let _ = writeln!(out, "{line}");
    }

    // ── sections, in the order they were opened ──
    let mut sections: Vec<(usize, String)> = Vec::new();
    for conv in written(game, Kind::Conversation) {
        sections.push((game.declared[conv].1, conv.clone()));
    }
    for room in &game.rooms {
        let line = game
            .bindings
            .iter()
            .filter(|b| match &b.site {
                Site::InRoom { room: r, .. } | Site::OnEnter { room: r } => r == room,
                _ => false,
            })
            .map(|b| b.line)
            .min()
            .unwrap_or(usize::MAX);
        sections.push((line, room.clone()));
    }
    sections.sort();

    for (_, name) in &sections {
        if game.declared.get(name).map(|(k, _)| *k) == Some(Kind::Conversation) {
            section_conversation(&mut out, game, name, &label);
        } else {
            section_room(&mut out, game, name, &label);
        }
    }
    out
}

fn section_conversation(out: &mut String, game: &Game, conv: &str, label: &dyn Fn(&str) -> String) {
    let with = game
        .spoken_with
        .get(conv)
        .map(|a| format!(" with {a}"))
        .unwrap_or_default();
    let _ = writeln!(out, "\nconversation {conv}{with}{}", label(conv));
    let scope = Scope::Conversation(conv.to_string());
    for b in &game.bindings {
        let Site::InConversation {
            conversation,
            option,
        } = &b.site
        else {
            continue;
        };
        if conversation != conv {
            continue;
        }
        let _ = writeln!(out, "  option {}{}", bare(option, &scope), label(option));
        body(out, b);
    }
}

fn section_room(out: &mut String, game: &Game, room: &str, label: &dyn Fn(&str) -> String) {
    let scope = Scope::Room(room.to_string());
    let _ = writeln!(out, "\nroom {room}");
    if game.start_room.as_deref() == Some(room) {
        let _ = writeln!(out, "  start");
    }
    for (r, who, _) in &game.holds {
        if r == room {
            let _ = writeln!(out, "  holds {who}");
        }
    }
    // every site in this room, in the order its first binding was written — which is the order
    // its hotspot or actor heading was, because a heading opens the site the bindings land in
    let mut sites: Vec<&String> = Vec::new();
    for b in &game.bindings {
        match &b.site {
            Site::OnEnter { room: r } if r == room => {
                let _ = writeln!(out, "\n  on-enter");
                body(out, b);
            }
            Site::InRoom { room: r, target } if r == room && !sites.contains(&target) => {
                sites.push(target)
            }
            _ => {}
        }
    }
    for target in sites {
        let kind = game.declared.get(target).map(|(k, _)| *k);
        let head = if kind == Some(Kind::Actor) {
            "actor"
        } else {
            "hotspot"
        };
        // an actor's site names a global declaration and carries no label of its own;
        // a hotspot's site *is* the declaration
        let shown = if head == "actor" {
            String::new()
        } else {
            label(target)
        };
        let _ = writeln!(out, "\n  {head} {}{shown}", bare(target, &scope));
        if let Some(item) = game.carried_form.get(target) {
            let _ = writeln!(out, "    is {item}");
        }
        for b in &game.bindings {
            let Site::InRoom { room: r, target: t } = &b.site else {
                continue;
            };
            if r != room || t != target {
                continue;
            }
            let object = b
                .object
                .as_ref()
                .map(|o| format!(" {o}"))
                .unwrap_or_default();
            let _ = writeln!(out, "    {}{object}:", b.verb);
            body(out, b);
        }
    }
}
