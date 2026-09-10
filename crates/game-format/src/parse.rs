//! A line-oriented reader for the `.game` format.
//!
//! Not a general parser and not trying to be. The format is a specimen's surface, its shape
//! is fixed by `design/bounded-expression-language.md`, and every construct it admits is
//! listed below. Anything it does not recognise is reported rather than ignored — a reader
//! that silently skips what it does not understand would let a specimen claim coverage it
//! does not have, which is the one failure this crate exists to prevent.

use crate::*;

const VERBS: [&str; 6] = [
    "look-at", "use-with", "talk-to", "pick-up", "walk-to", "give-to",
];

struct Line {
    no: usize,
    tokens: Vec<String>,
    text: Option<String>,
}

/// Split one source line into bare tokens and an optional trailing quoted string.
/// `??` begins an annotation and runs to end of line, unless it falls inside quotes.
fn lex(no: usize, raw: &str) -> Option<Line> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    // drop an annotation, counting quotes so one inside a string survives.
    // Indices here are CHAR positions, not byte positions — this format carries em dashes
    // in its authored prose, and slicing a &str by char count is a panic waiting for the
    // first one.
    let bytes: Vec<char> = raw.chars().collect();
    let mut cut = bytes.len();
    let mut quotes = 0usize;
    for i in 0..bytes.len() {
        if bytes[i] == '"' {
            quotes += 1;
        }
        if bytes[i] == '?' && i + 1 < bytes.len() && bytes[i + 1] == '?' && quotes % 2 == 0 {
            cut = i;
            break;
        }
    }
    let line: String = bytes[..cut].iter().collect();
    if line.trim().is_empty() {
        return None;
    }

    let (head, text) = match line.find('"') {
        Some(i) => {
            let rest = &line[i + 1..];
            let close = rest.rfind('"');
            (line[..i].to_string(), close.map(|j| rest[..j].to_string()))
        }
        None => (line.clone(), None),
    };
    let tokens: Vec<String> = head
        .split_whitespace()
        .map(|t| t.trim_end_matches(':').to_string())
        .filter(|t| !t.is_empty())
        .collect();
    Some(Line { no, tokens, text })
}

struct P {
    game: Game,
    problems: Vec<Problem>,
    room: Option<String>,
    conv: Option<String>,
    site: Option<Site>,
    binding: Option<Binding>,
}

impl P {
    /// The scope a bare name means, here.
    fn scope(&self) -> Scope {
        match (&self.room, &self.conv) {
            (Some(r), _) => Scope::Room(r.clone()),
            (_, Some(c)) => Scope::Conversation(c.clone()),
            _ => Scope::Global,
        }
    }
}

impl P {
    fn err(&mut self, no: usize, message: impl Into<String>) {
        self.problems.push(Problem {
            line: no,
            message: message.into(),
        });
    }

    /// Declare `name` in `scope`, under its canonical name. A declaration always knows its
    /// own scope, so only *references* need a second pass.
    fn declare_in(&mut self, name: &str, kind: Kind, no: usize, scope: &Scope) -> String {
        let canonical = scope.qualify(name);
        if let Some((prior, at)) = self.game.declared.get(&canonical) {
            let (prior, at) = (*prior, *at);
            self.err(
                no,
                format!("`{canonical}` is already declared as a {prior} on line {at}"),
            );
            return canonical;
        }
        self.game.declared.insert(canonical.clone(), (kind, no));
        canonical
    }

    fn declare(&mut self, name: &str, kind: Kind, no: usize) {
        self.declare_in(name, kind, no, &Scope::Global);
    }

    fn flush(&mut self) {
        if let Some(b) = self.binding.take() {
            self.game.bindings.push(b);
        }
    }

    fn open_binding(&mut self, verb: &str, object: Option<String>, site: Site, no: usize) {
        self.flush();
        let scope = self.scope();
        self.binding = Some(Binding {
            site,
            scope,
            verb: verb.to_string(),
            object,
            when: None,
            guards: Vec::new(),
            effects: Vec::new(),
            ending: false,
            line: no,
        });
    }
}

pub fn parse(source: &str, name: &str) -> (Game, Vec<Problem>) {
    let mut p = P {
        game: Game {
            source: name.to_string(),
            ..Default::default()
        },
        problems: Vec::new(),
        room: None,
        conv: None,
        site: None,
        binding: None,
    };

    for (i, raw) in source.lines().enumerate() {
        let Some(line) = lex(i + 1, raw) else {
            continue;
        };
        let no = line.no;
        let t: Vec<&str> = line.tokens.iter().map(|s| s.as_str()).collect();

        // a bare quoted line is an unguarded response
        if t.is_empty() {
            if line.text.is_some() && p.binding.is_none() {
                p.err(no, "response text outside a binding");
            }
            continue;
        }

        match t[0] {
            // ── declarations and sections ───────────────────────────────────
            "room" if line.text.is_some() => {
                if t.len() != 2 {
                    p.err(no, "a room declaration is `room <name> \"<label>\"`");
                } else {
                    p.declare(t[1], Kind::Room, no);
                    p.game.rooms.push(t[1].to_string());
                }
            }
            "room" => {
                p.flush();
                p.room = Some(t[1].to_string());
                p.conv = None;
                p.site = None;
            }
            "actor" if line.text.is_some() => p.declare(t[1], Kind::Actor, no),
            "actor" => {
                p.flush();
                match &p.room {
                    Some(r) => {
                        p.site = Some(Site::InRoom {
                            room: r.clone(),
                            target: t[1].to_string(),
                        })
                    }
                    None => p.err(no, "an actor site outside a room section"),
                }
            }
            "item" => {
                p.declare(t[1], Kind::Item, no);
                p.game.items.push(t[1].to_string());
                // `item coin 0..9` is `quantity: many`; a bare `item crowbar` is one.
                let cap = match t.get(2).and_then(|r| r.split_once("..")) {
                    Some((_, hi)) => hi.parse().unwrap_or(1),
                    None => 1,
                };
                if cap < 1 {
                    p.err(no, "an item that cannot be carried at all is not an item");
                }
                p.game.item_cap.insert(t[1].to_string(), cap.max(1));
            }
            "flag" => {
                let latched = t.contains(&"latched");
                let owner = t
                    .iter()
                    .position(|x| *x == "in")
                    .and_then(|i| t.get(i + 1))
                    .map(|s| s.to_string());
                let scope = match &owner {
                    Some(c) => Scope::Conversation(c.clone()),
                    None => Scope::Global,
                };
                let name = p.declare_in(t[1], Kind::Flag, no, &scope);
                p.game.flags.push(Flag {
                    name,
                    latched,
                    scope: owner,
                    line: no,
                });
            }
            "counter" => {
                p.declare(t[1], Kind::Counter, no);
                let (lo, hi) = match t.get(2).and_then(|r| r.split_once("..")) {
                    Some((a, b)) => (a.parse().unwrap_or(0), b.parse().unwrap_or(0)),
                    None => {
                        p.err(no, "a counter declaration needs a range, as `0..3`");
                        (0, 0)
                    }
                };
                p.game.counters.push(Counter {
                    name: t[1].to_string(),
                    lo,
                    hi,
                    line: no,
                });
            }
            "conversation" => {
                p.flush();
                p.declare(t[1], Kind::Conversation, no);
                p.conv = Some(t[1].to_string());
                p.room = None;
                p.site = None;
            }
            "hotspot" => {
                p.flush();
                match p.room.clone() {
                    Some(r) => {
                        let scope = Scope::Room(r.clone());
                        let target = p.declare_in(t[1], Kind::Hotspot, no, &scope);
                        p.site = Some(Site::InRoom { room: r, target });
                    }
                    None => p.err(no, "a hotspot outside a room section"),
                }
            }
            "option" => match p.conv.clone() {
                Some(c) => {
                    let scope = Scope::Conversation(c.clone());
                    let option = p.declare_in(t[1], Kind::Option_, no, &scope);
                    let site = Site::InConversation {
                        conversation: c,
                        option,
                    };
                    p.open_binding("option", None, site, no);
                }
                None => p.err(no, "an option outside a conversation"),
            },

            // ── room- and site-level facts ──────────────────────────────────
            "start" => match p.room.clone() {
                Some(r) => p.game.start_room = Some(r),
                None => p.err(no, "`start` outside a room section"),
            },
            // `holds <name>` is a room-level fact about what starts there. For an item it has
            // always been decoration — `is <item>` on a hotspot is what actually places one —
            // and for an actor it is the only way to say where they begin. Recorded raw and
            // sorted out in `resolve`, because a room may be written before the actor is
            // declared.
            "holds" => match (p.room.clone(), t.get(1)) {
                (Some(r), Some(x)) => p.game.holds.push((r, x.to_string(), no)),
                (None, _) => p.err(no, "`holds` outside a room section"),
                (_, None) => p.err(no, "`holds` needs a name"),
            },
            "on-enter" => match p.room.clone() {
                Some(r) => {
                    let site = Site::OnEnter { room: r };
                    p.site = None; // an entry binding belongs to the room, not to a target
                    p.open_binding("on-enter", None, site, no);
                }
                None => p.err(no, "`on-enter` outside a room section"),
            },
            "is" => match (&p.site, t.get(1)) {
                (Some(Site::InRoom { target, .. }), Some(item)) => {
                    p.game.carried_form.insert(target.clone(), item.to_string());
                }
                _ => p.err(no, "`is <item>` needs a hotspot and an item"),
            },

            // ── bindings ────────────────────────────────────────────────────
            v if VERBS.contains(&v) => {
                let object = t.get(1).map(|s| s.to_string());
                match p.site.clone() {
                    Some(s) => p.open_binding(v, object, s, no),
                    None => p.err(no, format!("`{v}` outside a hotspot or actor")),
                }
            }

            // ── binding body ────────────────────────────────────────────────
            "when" | "else" | "otherwise" | "say" | "do" | "ending" => {
                if p.binding.is_none() {
                    p.err(no, format!("`{}` outside a binding", t[0]));
                    continue;
                }
                let head = if t[0] == "else" && t.get(1) == Some(&"when") {
                    2
                } else {
                    1
                };
                match t[0] {
                    "ending" => p.binding.as_mut().unwrap().ending = true,
                    "say" | "otherwise" => {}
                    "do" => match effect(&t[1..], &p.game) {
                        Ok(e) => p.binding.as_mut().unwrap().effects.push(e),
                        Err(m) => p.err(no, m),
                    },
                    "when" | "else" => {
                        if t.len() <= head {
                            continue; // a bare `else "text"` is a refusal and carries no predicate
                        }
                        match pred(&t[head..]) {
                            Ok(q) => {
                                let b = p.binding.as_mut().unwrap();
                                // a predicate with text beside it selects a line; one without
                                // it gates the binding
                                if line.text.is_some() || t[0] == "else" {
                                    b.guards.push(q);
                                } else if b.when.is_some() {
                                    p.err(no, "this binding already has a `when`");
                                } else {
                                    b.when = Some(q);
                                }
                            }
                            Err(m) => p.err(no, m),
                        }
                    }
                    _ => unreachable!(),
                }
            }

            other => p.err(no, format!("`{other}` is not a construct this format has")),
        }
    }
    p.flush();
    let P { game, problems, .. } = p;
    (game, problems)
}

fn effect(t: &[&str], game: &Game) -> Result<Effect, String> {
    match t {
        ["leave"] => Ok(Effect::Leave),
        ["give", x] => Ok(Effect::Give(x.to_string(), 1)),
        ["spend", x] => Ok(Effect::Spend(x.to_string(), 1)),
        ["give", x, n] => n
            .parse()
            .map(|v| Effect::Give(x.to_string(), v))
            .map_err(|_| format!("`{n}` is not a number")),
        ["spend", x, n] => n
            .parse()
            .map(|v| Effect::Spend(x.to_string(), v))
            .map_err(|_| format!("`{n}` is not a number")),
        ["move", x] => Ok(Effect::Move(None, x.to_string())),
        ["move", a, "to", r] => Ok(Effect::Move(Some(a.to_string()), r.to_string())),
        ["move", _, r] => Err(format!(
            "moving somebody else reads `move <actor> to {r}` — the `to` is what tells the two \
             forms apart"
        )),
        ["enter", x] => Ok(Effect::Enter(x.to_string())),
        ["clear", x] => Ok(Effect::ClearFlag(x.to_string())),
        ["set", x] => {
            if game.counter(x).is_some() {
                Err(format!("`set {x}` on a counter needs a value"))
            } else {
                Ok(Effect::SetFlag(x.to_string()))
            }
        }
        ["set", x, n] => n
            .parse()
            .map(|v| Effect::SetCounter(x.to_string(), v))
            .map_err(|_| format!("`{n}` is not a number")),
        [] => Err("`do` with no effect".into()),
        _ => Err(format!(
            "`{}` is not an effect this format has",
            t.join(" ")
        )),
    }
}

// ── predicates: recursive descent over the grammar in the design ────────────

fn pred(t: &[&str]) -> Result<Pred, String> {
    let (q, rest) = disj(t)?;
    if rest.is_empty() {
        Ok(q)
    } else {
        Err(format!("trailing `{}` in a predicate", rest.join(" ")))
    }
}

fn disj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String> {
    let (mut left, mut rest) = conj(t)?;
    while rest.first() == Some(&"or") {
        let (right, r) = conj(&rest[1..])?;
        left = Pred::Or(Box::new(left), Box::new(right));
        rest = r;
    }
    Ok((left, rest))
}

fn conj<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String> {
    let (mut left, mut rest) = unary(t)?;
    while rest.first() == Some(&"and") {
        let (right, r) = unary(&rest[1..])?;
        left = Pred::And(Box::new(left), Box::new(right));
        rest = r;
    }
    Ok((left, rest))
}

fn unary<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String> {
    if t.first() == Some(&"not") {
        let (q, rest) = unary(&t[1..])?;
        return Ok((Pred::Not(Box::new(q)), rest));
    }
    atom(t)
}

fn cmp_of(s: &str) -> Option<Cmp> {
    Some(match s {
        "=" => Cmp::Eq,
        "!=" => Cmp::Ne,
        "<" => Cmp::Lt,
        "<=" => Cmp::Le,
        ">" => Cmp::Gt,
        ">=" => Cmp::Ge,
        _ => return None,
    })
}

fn atom<'a>(t: &'a [&'a str]) -> Result<(Pred, &'a [&'a str]), String> {
    match t.first() {
        None => Err("a predicate ended early".into()),
        Some(&"(") => {
            let (q, rest) = disj(&t[1..])?;
            match rest.first() {
                Some(&")") => Ok((q, &rest[1..])),
                _ => Err("unclosed `(` in a predicate".into()),
            }
        }
        Some(&"carrying") => match t.get(1) {
            Some(x) => Ok((Pred::Carrying(x.to_string()), &t[2..])),
            None => Err("`carrying` needs an item".into()),
        },
        Some(&"chosen") => match t.get(1) {
            Some(x) => Ok((Pred::Chosen(x.to_string()), &t[2..])),
            None => Err("`chosen` needs an option".into()),
        },
        Some(&"count") => {
            let name = t.get(1).ok_or("`count` needs an item")?;
            let c = t
                .get(2)
                .and_then(|x| cmp_of(x))
                .ok_or("`count <item>` must be compared, as `count coin >= 3`")?;
            let v: i64 = t
                .get(3)
                .ok_or("a comparison needs a value")?
                .parse()
                .map_err(|_| format!("`{}` is not a number", t[3]))?;
            Ok((Pred::Count(name.to_string(), c, v), &t[4..]))
        }
        // `<actor> in <room>`, infix because that is how it reads and because no flag name
        // can be followed by `in`. The same shape as the comparison below it.
        Some(name) if t.get(1) == Some(&"in") => match t.get(2) {
            Some(r) => Ok((Pred::At(name.to_string(), r.to_string()), &t[3..])),
            None => Err(format!("`{name} in` needs a room")),
        },
        Some(name) => {
            if let Some(c) = t.get(1).and_then(|s| cmp_of(s)) {
                let v: i64 = t
                    .get(2)
                    .ok_or("a comparison needs a value")?
                    .parse()
                    .map_err(|_| format!("`{}` is not a number", t[2]))?;
                Ok((Pred::Cmp(name.to_string(), c, v), &t[3..]))
            } else {
                Ok((Pred::Flag(name.to_string()), &t[1..]))
            }
        }
    }
}
