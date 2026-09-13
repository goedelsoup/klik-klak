//! A line-oriented reader for the `.game` format.
//!
//! Not a general parser and not trying to be. The format is a specimen's surface, its shape
//! is fixed by `design/bounded-expression-language.md`, and every construct it admits is
//! listed below. Anything it does not recognise is reported rather than ignored — a reader
//! that silently skips what it does not understand would let a specimen claim coverage it
//! does not have, which is the one failure this crate exists to prevent.

use crate::*;

const VERBS: [&str; 5] = ["look-at", "use-with", "talk-to", "pick-up", "walk-to"];

/// The written form of a construct, out of [`crate::FORMS`], which is where this format's
/// declaration half is specified.
///
/// The fallback is the keyword itself and is unreachable: `forms.rs` carries a row for every word
/// this reader dispatches on and `crates/game-format/tests/forms.rs` holds the two lists equal.
/// Returning the keyword rather than panicking is the reader's standing rule — it reports and does
/// not abort, which is what `.yidam/decisions/parser-silence.yml` found eight keywords breaking.
fn fm(keyword: &'static str, quoted: bool) -> &'static str {
    crate::form(keyword, quoted).unwrap_or(keyword)
}

/// The same, for a keyword read off the line rather than written into the arm.
fn fm_dyn(keyword: &str) -> String {
    crate::form(keyword, false).unwrap_or(keyword).to_string()
}

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
        .flat_map(parens)
        .map(|t| t.trim_end_matches(':').to_string())
        .filter(|t| !t.is_empty())
        .collect();
    Some(Line { no, tokens, text })
}

/// Split a whitespace-delimited token around parentheses, which are tokens in their own right.
///
/// The grammar has carried `atom := '(' pred ')'` since it was written and `atom()` has always
/// accepted it — **as three tokens**. Nothing had ever written a group, because nothing had
/// written a disjunction, so nobody had found that `(x` lexes as a name and comes back as
/// *"`(x` is not declared, wanted a flag"*: an error naming the operand when the fault is the
/// spacing. `.yidam/decisions/or-earns-it.yml`.
///
/// Nothing else in the language can hold a parenthesis. A label is quoted and is taken off this
/// line before it is split; every other token is a declared name, and no name here has ever
/// contained one.
fn parens(t: &str) -> Vec<String> {
    if !t.contains(['(', ')']) {
        return vec![t.to_string()];
    }
    let (mut out, mut cur) = (Vec::new(), String::new());
    for c in t.chars() {
        if c == '(' || c == ')' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            out.push(c.to_string());
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

struct P {
    game: Game,
    problems: Vec<Problem>,
    room: Option<String>,
    conv: Option<String>,
    site: Option<Site>,
    binding: Option<Binding>,
    /// the line of a bare `else "<text>"` on the binding being read, if one has been seen
    refusal: Option<usize>,
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

    /// The operand at `i`, or a problem naming the form this construct reads.
    ///
    /// **This replaces every `t[1]`, and the crash is why.** `t[1]` on a line with one token is
    /// an index out of bounds, and eight keywords aborted the process on a bare form until
    /// `.yidam/decisions/parser-silence.yml`. A reader whose stated purpose is to report rather
    /// than ignore must do neither.
    fn at<'a>(&mut self, no: usize, t: &[&'a str], i: usize, form: &str) -> Option<&'a str> {
        match t.get(i) {
            Some(x) => Some(x),
            None => {
                self.err(no, format!("reads `{form}`"));
                None
            }
        }
    }

    /// Report whatever follows the `used` tokens this construct accounted for.
    ///
    /// The other half of the same defect: `t[1]` where the line has three tokens drops the third
    /// without looking, and 229 of 349 specimen lines took a word that way. The form is carried
    /// into the message because this format has no grammar document a person reads, so the reader
    /// rejecting a line is the only place the line's shape is written down.
    fn rest(&mut self, no: usize, t: &[&str], used: usize, form: &str) {
        if t.len() > used {
            self.err(
                no,
                format!("`{}` is not part of `{form}`", t[used..].join(" ")),
            );
        }
    }

    /// Declare `name` in `scope`, under its canonical name. A declaration always knows its
    /// own scope, so only *references* need a second pass.
    ///
    /// The label is taken here rather than at the call sites, so a declaration cannot quietly
    /// fail to record one. `.yidam/decisions/authored-text.yml`.
    fn declare_in(
        &mut self,
        name: &str,
        kind: Kind,
        no: usize,
        scope: &Scope,
        label: Option<&str>,
    ) -> String {
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
        if let Some(l) = label {
            self.game.labels.insert(canonical.clone(), l.to_string());
        }
        canonical
    }

    fn declare(&mut self, name: &str, kind: Kind, no: usize, label: Option<&str>) -> String {
        self.declare_in(name, kind, no, &Scope::Global, label)
    }

    /// Add a line to the binding being read.
    fn respond(&mut self, refusing: bool, when: Option<Pred>, text: String, no: usize) {
        let r = Response {
            when,
            text,
            line: no,
        };
        if let Some(b) = self.binding.as_mut() {
            if refusing {
                b.refusals.push(r);
            } else {
                b.says.push(r);
            }
        }
    }

    fn flush(&mut self) {
        let refusal = self.refusal.take();
        if let Some(b) = self.binding.take() {
            // **A refusal is what the game says when the player tries something the
            // precondition forbids** — `interaction/entry-binding` put it that way while
            // arguing that an arrival trigger cannot have one, because nobody tried anything.
            //
            // The same sentence convicts a binding that has a refusal and nothing to refuse.
            // With no `when` and no guarded clause there is no state in which this binding is
            // offered and declines, so the line is text no player can reach — the third of
            // this format's checks for authored text nothing can produce, after `dead_writes`
            // and `unreachable_values`.
            //
            // Both instances in the committed corpus were on binary bindings, where the
            // second key had already done the refusing: you cannot use the lantern on the
            // gate without the lantern. `.yidam/decisions/availability.yml`.
            if let Some(line) = refusal {
                if b.when.is_none() && b.guards().next().is_none() {
                    self.problems.push(Problem {
                        line,
                        message: format!(
                            "`{}` has a refusal and nothing to refuse — no `when` and no \
                             guarded clause, so there is no state in which it is offered and \
                             declines, and this line is text no player can reach",
                            b.verb
                        ),
                    });
                }
            }
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
            refuses: false,
            says: Vec::new(),
            refusals: Vec::new(),
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
        refusal: None,
    };

    for (i, raw) in source.lines().enumerate() {
        let Some(line) = lex(i + 1, raw) else {
            continue;
        };
        let no = line.no;
        let t: Vec<&str> = line.tokens.iter().map(|s| s.as_str()).collect();

        // a bare quoted line is an unguarded response
        if t.is_empty() {
            match (line.text.clone(), p.binding.is_some()) {
                (Some(text), true) => p.respond(false, None, text, no),
                (Some(_), false) => p.err(no, "response text outside a binding"),
                (None, _) => {}
            }
            continue;
        }

        // **A construct accounts for its whole line, and the quoted half is part of it.**
        // `.yidam/decisions/parser-silence.yml` established the rule over the bare tokens and
        // `tests/ignored.rs` states in its own comment that its mutation goes "before the first
        // quote", so this half had never been checked by anything. Six constructs took a quoted
        // string and dropped it without a word. `.yidam/decisions/line-format.yml`.
        if line.text.is_some() && crate::stray_quote(t[0]) {
            p.err(
                no,
                format!(
                    "`{}` reads no quoted text — the form is `{}`",
                    t[0],
                    fm_dyn(t[0])
                ),
            );
            continue;
        }

        match t[0] {
            // ── declarations and sections ───────────────────────────────────
            "room" if line.text.is_some() => {
                if t.len() != 2 {
                    p.err(no, format!("a room declaration is `{}`", fm("room", true)));
                } else {
                    p.declare(t[1], Kind::Room, no, line.text.as_deref());
                    p.game.rooms.push(t[1].to_string());
                }
            }
            "room" => {
                p.flush();
                let Some(name) = p.at(no, &t, 1, fm("room", false)) else {
                    continue;
                };
                p.rest(no, &t, 2, fm("room", false));
                p.room = Some(name.to_string());
                p.conv = None;
                p.site = None;
            }
            "actor" if line.text.is_some() => {
                let form = fm("actor", true);
                let Some(name) = p.at(no, &t, 1, form) else {
                    continue;
                };
                p.rest(no, &t, 2, form);
                p.declare(name, Kind::Actor, no, line.text.as_deref());
            }
            "actor" => {
                p.flush();
                let Some(name) = p.at(no, &t, 1, fm("actor", false)) else {
                    continue;
                };
                p.rest(no, &t, 2, fm("actor", false));
                let name = name.to_string();
                match &p.room {
                    Some(r) => {
                        p.site = Some(Site::InRoom {
                            room: r.clone(),
                            target: name,
                        })
                    }
                    None => p.err(no, "an actor site outside a room section"),
                }
            }
            "item" => {
                let form = fm("item", true);
                let Some(name) = p.at(no, &t, 1, form) else {
                    continue;
                };
                let name = name.to_string();
                // `item coin 0..9` is `quantity: many`; a bare `item crowbar` is one.
                let range = t.get(2).and_then(|r| r.split_once(".."));
                if t.len() > 2 && range.is_none() {
                    p.err(no, format!("`{}` is not a range, as `0..4`", t[2]));
                }
                p.rest(no, &t, 2 + usize::from(range.is_some()), form);
                let cap = match range {
                    Some((_, hi)) => hi.parse().unwrap_or(1),
                    None => 1,
                };
                if cap < 1 {
                    p.err(no, "an item that cannot be carried at all is not an item");
                }
                p.declare(&name, Kind::Item, no, line.text.as_deref());
                p.game.items.push(name.clone());
                p.game.item_cap.insert(name, cap.max(1));
            }
            // A flag's modifiers are read in the order the table gives them, rather
            // than searched for. `t.contains(&"latched")` found the word anywhere on the line,
            // so it read a conversation called `latched` as the modifier, and every token it did
            // not recognise fell through. Order is the cheapest grammar there is.
            "flag" => {
                let form = fm("flag", true);
                let Some(flag) = p.at(no, &t, 1, form) else {
                    continue;
                };
                let mut i = 2;
                let latched = t.get(i) == Some(&"latched");
                i += usize::from(latched);
                let owner = match t.get(i) {
                    Some(&"in") => {
                        let owner = p.at(no, &t, i + 1, form).map(str::to_string);
                        i += 2;
                        owner
                    }
                    _ => None,
                };
                p.rest(no, &t, i, form);
                let scope = match &owner {
                    Some(c) => Scope::Conversation(c.clone()),
                    None => Scope::Global,
                };
                let name = p.declare_in(flag, Kind::Flag, no, &scope, line.text.as_deref());
                p.game.flags.push(Flag {
                    name,
                    latched,
                    scope: owner,
                    line: no,
                });
            }
            "counter" => {
                let form = fm("counter", true);
                let Some(name) = p.at(no, &t, 1, form) else {
                    continue;
                };
                let name = name.to_string();
                p.rest(no, &t, 3, form);
                let (lo, hi) = match t.get(2).and_then(|r| r.split_once("..")) {
                    Some((a, b)) => (a.parse().unwrap_or(0), b.parse().unwrap_or(0)),
                    None => {
                        p.err(no, "a counter declaration needs a range, as `0..3`");
                        (0, 0)
                    }
                };
                p.declare(&name, Kind::Counter, no, line.text.as_deref());
                p.game.counters.push(Counter {
                    name,
                    lo,
                    hi,
                    line: no,
                });
            }
            // The `with` half was parsed and dropped from the
            // day this arm was written: only `t[1]` was read, so `conversation keeper-talk beside
            // keeper` parsed identically and nothing could say who the player was talking to.
            // `.yidam/decisions/verb-names.yml`.
            "conversation" => {
                p.flush();
                let Some(name) = p.at(no, &t, 1, fm("conversation", true)) else {
                    continue;
                };
                let name = name.to_string();
                p.rest(no, &t, 4, fm("conversation", true));
                p.declare(&name, Kind::Conversation, no, line.text.as_deref());
                match (t.get(2), t.get(3)) {
                    (Some(&"with"), Some(a)) => {
                        p.game.spoken_with.insert(name.clone(), a.to_string());
                    }
                    _ => p.err(
                        no,
                        format!(
                            "`conversation {name}` is missing its `with` — the form is \
                             `{}`, and the `with` is what names who it is held with",
                            fm("conversation", true)
                        ),
                    ),
                }
                p.conv = Some(name);
                p.room = None;
                p.site = None;
            }
            "hotspot" => {
                p.flush();
                let form = fm("hotspot", true);
                let Some(name) = p.at(no, &t, 1, form) else {
                    continue;
                };
                let name = name.to_string();
                p.rest(no, &t, 2, form);
                match p.room.clone() {
                    Some(r) => {
                        let scope = Scope::Room(r.clone());
                        let target =
                            p.declare_in(&name, Kind::Hotspot, no, &scope, line.text.as_deref());
                        p.site = Some(Site::InRoom { room: r, target });
                    }
                    None => p.err(no, "a hotspot outside a room section"),
                }
            }
            "option" => {
                let form = fm("option", true);
                let Some(name) = p.at(no, &t, 1, form) else {
                    continue;
                };
                let name = name.to_string();
                p.rest(no, &t, 2, form);
                match p.conv.clone() {
                    Some(c) => {
                        let scope = Scope::Conversation(c.clone());
                        let option =
                            p.declare_in(&name, Kind::Option_, no, &scope, line.text.as_deref());
                        let site = Site::InConversation {
                            conversation: c,
                            option,
                        };
                        p.open_binding("option", None, site, no);
                    }
                    None => p.err(no, "an option outside a conversation"),
                }
            }

            // ── room- and site-level facts ──────────────────────────────────
            "start" => {
                p.rest(no, &t, 1, fm("start", false));
                match p.room.clone() {
                    Some(r) => p.game.start_room = Some(r),
                    None => p.err(no, "`start` outside a room section"),
                }
            }
            // `holds` says where an actor begins, and it is the only way to say it.
            // It once took an item too; `resolve` is where that stopped, and why. Recorded
            // raw and sorted out there, because a room may be written before the actor is.
            "holds" => {
                p.rest(no, &t, 2, fm("holds", false));
                match (p.room.clone(), t.get(1)) {
                    (Some(r), Some(x)) => p.game.holds.push((r, x.to_string(), no)),
                    (None, _) => p.err(no, "`holds` outside a room section"),
                    (_, None) => p.err(no, "`holds` needs a name"),
                }
            }
            // `exit` — the first primitive this corpus has admitted on the
            // second question in `AGENTS.md` rather than the first. Each of the eighteen bare
            // exits in a shipped game is sayable here as a hotspot with a `walk-to` that moves;
            // *an exit* is not, and `chain(n)` writes that shape out once per room in Rust
            // because of it.
            //
            // It expands into exactly the binding an author would have written, before anything
            // reads the game, so the checker sees no archetype at all — and a `when`/`else`
            // written under it lands on the expansion the way it lands on any other binding.
            // That is the whole of what makes an archetype safe here and is not free elsewhere;
            // see `.yidam/decisions/exit-archetype.yml`.
            "exit" => {
                p.rest(no, &t, 4, fm("exit", false));
                match (p.room.clone(), t.get(1), t.get(2), t.get(3)) {
                    (Some(r), Some(name), Some(&"to"), Some(dest)) => {
                        p.flush();
                        let scope = Scope::Room(r.clone());
                        let target = p.declare_in(name, Kind::Hotspot, no, &scope, None);
                        let site = Site::InRoom {
                            room: r,
                            target: target.clone(),
                        };
                        p.site = Some(site.clone());
                        p.open_binding("walk-to", None, site, no);
                        if let Some(b) = p.binding.as_mut() {
                            b.effects.push(Effect::Move(None, dest.to_string()));
                        }
                    }
                    (None, ..) => p.err(no, "`exit` outside a room section"),
                    (_, None, ..) => p.err(no, "`exit` needs a name"),
                    (_, Some(name), ..) => p.err(
                        no,
                        format!(
                            "`exit {name}` is missing its `to` — the form is `{}`, and the \
                         `to` is what names where it goes",
                            fm("exit", false)
                        ),
                    ),
                }
            }
            "on-enter" => {
                p.rest(no, &t, 1, fm("on-enter", false));
                match p.room.clone() {
                    Some(r) => {
                        let site = Site::OnEnter { room: r };
                        p.site = None; // an entry binding belongs to the room, not to a target
                        p.open_binding("on-enter", None, site, no);
                    }
                    None => p.err(no, "`on-enter` outside a room section"),
                }
            }
            "is" => {
                p.rest(no, &t, 2, fm("is", false));
                match (&p.site, t.get(1)) {
                    (Some(Site::InRoom { target, .. }), Some(item)) => {
                        p.game.carried_form.insert(target.clone(), item.to_string());
                    }
                    _ => p.err(
                        no,
                        format!("`{}` needs a hotspot and an item", fm("is", false)),
                    ),
                }
            }

            // ── bindings ────────────────────────────────────────────────────
            // **Only `use-with` takes a second object, and this is where that is enforced.**
            // It was `t.get(1)` for every verb, so `look-at crowbar:` parsed as a binding with
            // an object on it — which `verb_menu::live` shows only while the player carries the
            // crowbar. A description that comes and goes with the inventory, written by a typo.
            // `resolve` never caught it because the name it was handed was declared.
            // `.yidam/decisions/parser-silence.yml`.
            v if VERBS.contains(&v) => {
                let object = if v == "use-with" {
                    // **Optional, and `arity-cost`'s measurement is why.** The reduction that
                    // record measured — take the second key away and put `carrying <item>` in
                    // the guard — is written in this format and parsed by this reader, and a
                    // bare `use-with` is the reduced form. `crates/reachability/tests/arity.rs`
                    // writes 124 characters more of it than of the real thing, which is the
                    // finding. Requiring the object here would make the counterfactual
                    // unwritable and the measurement unrepeatable.
                    p.rest(no, &t, 2, fm("use-with", false));
                    t.get(1).map(|s| s.to_string())
                } else {
                    p.rest(no, &t, 1, v);
                    None
                };
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
                if matches!(t[0], "ending" | "say" | "otherwise") {
                    p.rest(no, &t, 1, t[0]);
                }
                match t[0] {
                    "ending" => p.binding.as_mut().unwrap().ending = true,
                    // **A `say` and an `otherwise` are one construct**: a line with no predicate.
                    // What makes the second a fallback is that it is written after the guarded
                    // ones, not that it is spelled differently — so both land in the same list
                    // and the list is read first-match-wins.
                    //
                    // This arm was `{}` for six phases and dropped every word a game says.
                    // `.yidam/decisions/authored-text.yml` separates the two claims that made
                    // that look like a position: the state model ignoring presentation, which it
                    // still does, and the reader throwing it away, which it no longer does.
                    "say" | "otherwise" => match line.text.clone() {
                        Some(text) => p.respond(false, None, text, no),
                        None => p.err(no, format!("`{}` with no line", t[0])),
                    },
                    "do" => match effect(&t[1..], &p.game) {
                        Ok(e) => p.binding.as_mut().unwrap().effects.push(e),
                        Err(m) => p.err(no, m),
                    },
                    "when" | "else" => {
                        // Any `else`, bare or guarded, is the binding saying *shown and
                        // declines* rather than *withheld*. The bare form is tracked separately
                        // below because only it can be a refusal with nothing to refuse; for the
                        // menu the distinction that matters is whether an else exists at all,
                        // and `lighthouse.game`'s oil-can refuses in two worded branches and
                        // never bare.
                        let refusing = t[0] == "else";
                        if refusing {
                            p.binding.as_mut().unwrap().refuses = true;
                        }
                        if t.len() <= head {
                            // a bare `else "text"` is a refusal and carries no predicate
                            if refusing {
                                p.refusal = Some(no);
                                if let Some(text) = line.text.clone() {
                                    p.respond(true, None, text, no);
                                }
                            } else if line.text.is_some() {
                                // `when "<line>"` is a line that always applies, which is what
                                // `otherwise` spells. It was dropped in silence until the lines
                                // were kept, and silence is what this reader exists not to do.
                                p.err(
                                    no,
                                    "a `when` carrying a line needs a predicate — `otherwise` is \
                                     the spelling for one that always applies",
                                );
                            }
                            continue;
                        }
                        match pred(&t[head..]) {
                            // a predicate with text beside it selects a line; one without it
                            // gates the binding
                            Ok(q) => match (line.text.clone(), refusing) {
                                (Some(text), _) => p.respond(refusing, Some(q), text, no),
                                (None, false) => {
                                    let b = p.binding.as_mut().unwrap();
                                    if b.when.is_some() {
                                        p.err(no, "this binding already has a `when`");
                                    } else {
                                        b.when = Some(q);
                                    }
                                }
                                // A refusal writes no state, so a guarded one with no line is a
                                // predicate that does nothing whatever: the binding already
                                // declines by having an `else` at all. Reachable only since the
                                // lines were kept, because there was nothing to be missing.
                                (None, true) => p.err(
                                    no,
                                    "an `else when` needs a line — a guarded refusal with \
                                     nothing to say is a predicate with no effect",
                                ),
                            },
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
