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
mod resolve;

pub use parse::parse;
pub use resolve::resolve;

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
    Give(String),
    /// `interaction -consumes-> inventory-item` — the one effect this corpus had to add
    Spend(String),
    /// `actor -occupies-> room`
    Move(String),
    /// `interaction -opens-> conversation`
    Enter(String),
    /// `conversation -returns-control-to-> actor`
    Leave,
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
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub site: Site,
    pub verb: String,
    /// the second object of a binary verb
    pub object: Option<String>,
    pub when: Option<Pred>,
    /// predicates appearing in a guarded response list — checked, never evaluated here
    pub guards: Vec<Pred>,
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
    pub flags: Vec<Flag>,
    pub counters: Vec<Counter>,
    pub items: Vec<String>,
    pub rooms: Vec<String>,
    pub bindings: Vec<Binding>,
    /// hotspot -> the item it is the uncarried form of (`inventory-item -carried-form-of->`)
    pub carried_form: BTreeMap<String, String>,
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
    pub fn flag(&self, name: &str) -> Option<&Flag> {
        self.flags.iter().find(|f| f.name == name)
    }
    pub fn counter(&self, name: &str) -> Option<&Counter> {
        self.counters.iter().find(|c| c.name == name)
    }
}
