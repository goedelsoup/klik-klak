//! The `.game` authoring format: a model, a parser, and the admission rule.
//!
//! A `.game` file is a specimen — a worked example authored in the language proposed by
//! `design/bounded-expression-language.md`. Nothing here runs one. This crate reads the
//! authored form so that other crates can compute over it, which is the difference between
//! an instrument and an engine and is this repository's characteristic hazard.
//!
//! The one rule the format rests on is the predicate layer's **admission rule**: every
//! operand must name something the game declares. [`Game::resolve`] is that rule, and it is
//! type-aware where the shell gate it replaced was not — `carrying` must name an item,
//! `move` must name a room, `chosen` must name an option. A name that resolves to the wrong
//! kind used to pass.

use std::collections::BTreeMap;
use std::fmt;

mod parse;
mod print;
mod resolve;
mod world;

pub use parse::parse;
pub use print::print;
pub use resolve::resolve;
pub use world::{slot, State, World};

// ── model ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Room,
    Actor,
    Item,
    Flag,
    Counter,
    Conversation,
    Hotspot,
    Option_,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Kind::Room => "room",
            Kind::Actor => "actor",
            Kind::Item => "item",
            Kind::Flag => "flag",
            Kind::Counter => "counter",
            Kind::Conversation => "conversation",
            Kind::Hotspot => "hotspot",
            Kind::Option_ => "option",
        };
        f.write_str(s)
    }
}

/// Where a name lives.
///
/// Not invented for the format: it is the ontology's containment read as a namespace.
/// `room -contains-> hotspot` makes a hotspot's name the room's to give, and a conversation
/// owns its options — which is what AGS does, numbering options *for the current topic*.
/// Everything else is the world's and is global.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    Global,
    Room(String),
    Conversation(String),
}

impl Scope {
    pub fn qualifier(&self) -> Option<&str> {
        match self {
            Scope::Global => None,
            Scope::Room(r) => Some(r),
            Scope::Conversation(c) => Some(c),
        }
    }
    /// The canonical name of something declared in this scope.
    pub fn qualify(&self, name: &str) -> String {
        match self.qualifier() {
            Some(q) => format!("{q}.{name}"),
            None => name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Cmp {
    /// How the comparison is written in a `.game` file.
    pub fn spelling(self) -> &'static str {
        match self {
            Cmp::Eq => "=",
            Cmp::Ne => "!=",
            Cmp::Lt => "<",
            Cmp::Le => "<=",
            Cmp::Gt => ">",
            Cmp::Ge => ">=",
        }
    }
    pub fn holds(self, a: i64, b: i64) -> bool {
        match self {
            Cmp::Eq => a == b,
            Cmp::Ne => a != b,
            Cmp::Lt => a < b,
            Cmp::Le => a <= b,
            Cmp::Gt => a > b,
            Cmp::Ge => a >= b,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pred {
    Flag(String),
    Carrying(String),
    Chosen(String),
    Cmp(String, Cmp, i64),
    /// `count <item> <cmp> <n>` — how many of an item is carried.
    ///
    /// The one quantifier this language has. The design excluded `any`/`all` on the grounds
    /// that they need a collection and the vocabulary has none; an item bearing a quantity
    /// *is* that collection, and this is the only question worth asking about it.
    Count(String, Cmp, i64),
    /// `<actor> in <room>` — where a non-player actor is.
    ///
    /// An effect nothing can read is not state, so `move <actor> to <room>` entails this and
    /// the two are added together. It is also what bounds the cost: `crates/reachability`
    /// tracks only the actors some predicate names.
    At(String, String),
    Not(Box<Pred>),
    And(Box<Pred>, Box<Pred>),
    Or(Box<Pred>, Box<Pred>),
}

/// The closed effect list. Each variant is a relationship the corpus ontology declares;
/// `.yidam/decisions/effect-vocabulary.yml` records which, and which candidates were
/// weighed and found already expressible.
#[derive(Debug, Clone)]
pub enum Effect {
    /// `interaction -sets-> world-flag`
    SetFlag(String),
    /// the same relationship, writing the other value
    ClearFlag(String),
    /// `interaction -sets-> world-flag`, for a counter
    SetCounter(String, i64),
    /// `inventory-item -acquired-by-> interaction`, read from the binding's side
    Give(String, i64),
    /// `interaction -consumes-> inventory-item` — the one effect this corpus had to add
    Spend(String, i64),
    /// `actor -occupies-> room`
    ///
    /// `None` is the player, and the asymmetry is the point: `occupies` is declared on `actor`
    /// and authored by `actor/non-player-character`, so the relationship was always about any
    /// actor. The design table read it as "move the player" for six phases, which is a
    /// relationship read at half its width.
    Move(Option<String>, String),
    /// `interaction -opens-> conversation`
    Enter(String),
    /// `conversation -returns-control-to-> actor`
    Leave,
}

/// One line the game says, and the predicate that selects it.
///
/// `when <predicate> "<line>"` is the construct
/// [`interaction-table`](../../../.yidam/corpus/data-format/interaction-table.yml) calls the
/// **guarded response list** — *"which keys and reads like writing on the same line"*. This is
/// that line, and for six phases the reader kept the predicate and dropped the text beside it.
/// `.yidam/decisions/authored-text.yml`.
#[derive(Debug, Clone)]
pub struct Response {
    /// The predicate that selects this line, or `None` for one that always applies — a `say`, a
    /// bare quoted line, an `otherwise`, or a bare `else`.
    ///
    /// A list is **first match wins**, which is how the author reads it: `otherwise` is a
    /// fallback because it is written last, not because it is spelled differently.
    pub when: Option<Pred>,
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Site {
    /// a hotspot or an actor standing in a room
    InRoom { room: String, target: String },
    /// a choice offered inside an exchange
    InConversation {
        conversation: String,
        option: String,
    },
    /// **arriving in a room**, which is not a verb applied to a target and is the one site an
    /// interaction table keyed by verb and target cannot index.
    ///
    /// Two classes asked for this independently: `verb/walk-to.yml`, reading engine
    /// documentation, and `cutscene/scripted-sequence.yml`, assessed against a specimen. Both
    /// AGS eras document it — "Player enters room (after fadein)" in 2.72's interaction event
    /// list, "Enters room after fade-in" in the current manual.
    ///
    /// A binding at this site is **forced**: it is not offered to the player, it fires. That
    /// is what makes the state it fires from transient, and it is the whole reason this is a
    /// different kind of site rather than a verb nobody types.
    OnEnter { room: String },
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub site: Site,
    /// where this binding was written, and therefore what a bare name in it means
    pub scope: Scope,
    pub verb: String,
    /// the second object of a binary verb
    pub object: Option<String>,
    pub when: Option<Pred>,
    /// Whether a bare `else` sits under the guard — the one bit that separates a binding the
    /// player is *shown and declined* from one they are never shown at all.
    ///
    /// Nothing in the reachability search reads it, because a refusal writes no state and is
    /// therefore not a transition. `.yidam/decisions/availability.yml` established that the two
    /// forms are the format's way of saying *withheld* against *refused*, and the model went on
    /// discarding the distinction for four phases, because the only reader it had was a search
    /// that cannot see either. The reader that needs it is the interface.
    ///
    /// Nearly derivable from `refusals` now and kept because of the gap: an `else` with no line
    /// at all still declines, and has no `Response` to be found in.
    pub refuses: bool,
    /// What this binding says when it fires, first match wins.
    ///
    /// A `say`, a bare quoted line and an `otherwise` are all a `Response` with no predicate;
    /// `when <pred> "<line>"` is one with. Nothing in `crates/reachability` reads this and
    /// nothing should: a line writes no state, so it is not an edge, and that is the half of
    /// `.yidam/decisions/effect-list-semantics.yml` that was always a position.
    pub says: Vec<Response>,
    /// What it says when its `when` does not hold — the `else` forms, bare or guarded.
    pub refusals: Vec<Response>,
    pub effects: Vec<Effect>,
    pub ending: bool,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub name: String,
    /// `world-flag.reversible: no` — nothing can put it back
    pub latched: bool,
    /// `world-flag.scope`, when it is a conversation rather than the world
    pub scope: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Counter {
    pub name: String,
    pub lo: i64,
    pub hi: i64,
    pub line: usize,
}

#[derive(Debug, Default)]
pub struct Game {
    pub declared: BTreeMap<String, (Kind, usize)>,
    /// What a declaration is called, by the same canonical name `declared` is keyed by.
    ///
    /// A separate map rather than a third field on `declared`, because kind and line are what the
    /// **admission rule** needs — every operand must name something declared, of the right kind —
    /// and a label is what a presenter needs. One struct would say they are the same kind of fact.
    /// Two put the seam where the decision is, and make it measurable: `crates/reachability` reads
    /// `declared` and never this, and the evidence is that no number it publishes moved when this
    /// map was filled.
    ///
    /// **The string is three different things and [`Kind`] is what says which.** On a room, actor,
    /// item or hotspot it is a name a player reads; on an option it is a line the *player says*;
    /// on a flag or a counter it is a gloss for whoever reads the file, which no player can be
    /// shown. `.yidam/decisions/authored-text.yml`.
    pub labels: BTreeMap<String, String>,
    pub flags: Vec<Flag>,
    pub counters: Vec<Counter>,
    pub items: Vec<String>,
    /// The most of an item that can be carried at once.
    ///
    /// Default 1, which is `inventory-item.quantity: one` — a crowbar. An item declaring a
    /// range is `quantity: many`, and the ceiling is not decoration: **quantity is the one
    /// thing in this language that can make the state space infinite.** Loops and recursion
    /// were excluded to guarantee that *evaluation* terminates; this is *analysis*
    /// terminating, which is a different property, and an unbounded supply breaks it. A
    /// ceiling restores it by declaration, the way a counter's range already did.
    pub item_cap: BTreeMap<String, i64>,
    pub rooms: Vec<String>,
    pub bindings: Vec<Binding>,
    /// hotspot -> the item it is the uncarried form of (`inventory-item -carried-form-of->`)
    pub carried_form: BTreeMap<String, String>,
    /// every `holds <name>` as written, with the room it was written in and its line.
    ///
    /// Kept raw because a room section may be written before the thing it holds is declared,
    /// and resolution is where kinds are known. `resolve` turns an actor into a starting
    /// position and refuses everything else — including the item this once took, which was
    /// `inventory-item -found-in-> room` and was never information: a hotspot belongs to a
    /// room, so the room an item is found in is the room of its carried form.
    pub holds: Vec<(String, String, usize)>,
    /// actor -> the room it starts in, from `holds`. `actor -occupies-> room`, at rest.
    pub actor_start: BTreeMap<String, String>,
    /// conversation -> the actor it is held with, from `conversation <name> with <actor>`.
    ///
    /// **The `with` was written, unchecked and unread for the whole life of this format.** The
    /// parser took `t[1]` and never looked at `t[2]` or `t[3]`, so `conversation keeper-talk
    /// beside keeper` parsed identically and a game could not say who the player was talking to.
    /// Found by rendering a menu — `.yidam/decisions/verb-names.yml`.
    pub spoken_with: BTreeMap<String, String>,
    pub start_room: Option<String>,
    pub source: String,
}

#[derive(Debug)]
pub struct Problem {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl Game {
    /// Resolve a name as written, from the scope it was written in: the local scope first,
    /// then the world. A name carrying a `.` is already explicit and is taken as given.
    pub fn lookup(&self, name: &str, from: &Scope) -> Option<(String, Kind, usize)> {
        if name.contains('.') {
            return self
                .declared
                .get(name)
                .map(|(k, l)| (name.to_string(), *k, *l));
        }
        let local = from.qualify(name);
        if let Some((k, l)) = self.declared.get(&local) {
            return Some((local, *k, *l));
        }
        self.declared
            .get(name)
            .map(|(k, l)| (name.to_string(), *k, *l))
    }

    /// What something is called, if it was given a label.
    pub fn label(&self, name: &str) -> Option<&str> {
        self.labels.get(name).map(String::as_str)
    }

    pub fn flag(&self, name: &str) -> Option<&Flag> {
        self.flags.iter().find(|f| f.name == name)
    }
    pub fn counter(&self, name: &str) -> Option<&Counter> {
        self.counters.iter().find(|c| c.name == name)
    }
}

impl Binding {
    /// Every predicate that selects a line, in written order: the firing list, then the refusals.
    ///
    /// These were a `guards: Vec<Pred>` of their own until the lines they select were kept. A
    /// predicate and the line it selects are one construct, and holding the predicate in two
    /// places would be two copies of one fact — which is the defect this repository keeps finding
    /// in its own prose. Readers that only want the predicates ask here.
    pub fn guards(&self) -> impl Iterator<Item = &Pred> {
        self.says
            .iter()
            .chain(&self.refusals)
            .filter_map(|r| r.when.as_ref())
    }

    /// The same, for the pass that rewrites names into their canonical form.
    pub fn guards_mut(&mut self) -> impl Iterator<Item = &mut Pred> {
        self.says
            .iter_mut()
            .chain(&mut self.refusals)
            .filter_map(|r| r.when.as_mut())
    }
}
