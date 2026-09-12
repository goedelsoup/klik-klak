//! composes: verb-menu, interaction-resolver
//!
//! **The menu, a choice, the response, and again.**
//!
//! This is not a component and declares none. `composes:` is the gate's second spelling and says
//! what this file is: a **client** of the components the corpus specifies.
//! `.yidam/decisions/playable.yml`.
//!
//! The loop is not in this corpus's subject and is not missing from it. This is a corpus about
//! authoring vocabularies, and no vocabulary here has a form of *every frame* — AGS exposes it as
//! a script slot, `repeatedly_execute`, and what goes in the slot is arbitrary script. A
//! declarative form of it would be unbounded computation on every frame, which is precisely what
//! `design/bounded-expression-language.md` excludes to keep the reachable set enumerable.
//!
//! **What this does that it should not.** It prints a line and returns.
//! `subsystem/response-presenter` says *"turn a response into something perceived, and do not
//! return until it is over"*, and a `println!` is the first half without the second. There is no
//! presenter, so the client does that job inline and does it wrongly, in the exact way that node
//! was seeded to name.

use game_format::{parse, resolve, Game, State, World};
use interaction_resolver::{resolve as respond, settle, Outcome};
use verb_menu::{shown, Entry};

/// A turn: what the player was shown, what they picked, and what the game said.
#[derive(Debug, Clone)]
pub struct Turn {
    /// every entry offered, in the order it was shown
    pub offered: Vec<String>,
    /// the one that was picked, by the same spelling
    pub picked: String,
    /// what the game said, or `None`
    pub said: Option<String>,
    /// whether the binding fired or declined
    pub fired: bool,
    /// what the player can *see* changed when nothing was said: the room they are now in, or the
    /// exchange they are now inside.
    ///
    /// **The first thing this slice found.** `walk-to door` says nothing and moves you to another
    /// room, and calling that a silent miss would be wrong — in a point-and-click the movement
    /// *is* the response. `subsystem/interaction-resolver`'s *"never nothing"* is a promise about
    /// the outcome and not about the line, and nothing could tell the two apart until something
    /// printed a transcript. `.yidam/decisions/playable.yml`.
    pub moved: Option<String>,
}

impl Turn {
    /// **Nothing was said and nothing the player can see changed.**
    ///
    /// This is the failure `subsystem/interaction-resolver` exists to prevent — *"a silent miss is
    /// the failure players read as brokenness"* — and it is narrower than *no line*. It is also
    /// narrower than the checker's `observable`, which asks whether a transition moves the search:
    /// a flag set with no line is observable to a search and invisible to a player, and this is
    /// the first thing in the repository that can tell those apart.
    pub fn silent(&self) -> bool {
        self.said.is_none() && self.moved.is_none()
    }
}

/// Why a session stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    /// a binding marked `ending` fired
    Ending,
    /// the script ran out, or the player asked to stop
    OutOfInput,
    /// a choice named nothing on the menu
    NoSuchChoice(String),
    /// entering a room started a chain of `on-enter` bindings that never settles
    EntryCycle(String),
}

/// How an entry is written on the menu and in a script.
///
/// `look-at crate` and `use-with crowbar on crate`. The target is the canonical name rather than
/// the label, because a script is written by whoever wrote the game and a label is for a player.
pub fn spell(e: &Entry<'_>) -> String {
    let target = e.target.rsplit('.').next().unwrap_or(e.target);
    match e.object {
        Some(o) => format!("{} {} on {}", e.verb, o, target),
        None => format!("{} {}", e.verb, target),
    }
}

/// A game being played: the world, and the state the client owns.
///
/// **The state is here and in no component.** All four subsystems declare none across frames, and
/// one that held it would be one the other two had to ask — which is what would stop the menu being
/// *"a pure function of the target and the state"*. `.yidam/decisions/playable.yml`.
pub struct Session<'a> {
    pub world: World<'a>,
    pub state: State,
    pub log: Vec<Turn>,
}

pub fn load(source: &str, name: &str) -> Result<Game, Vec<String>> {
    let (mut game, mut problems) = parse(source, name);
    problems.extend(resolve(&mut game));
    if problems.is_empty() {
        Ok(game)
    } else {
        Err(problems.into_iter().map(|p| p.to_string()).collect())
    }
}

impl<'a> Session<'a> {
    /// Start a game: build the world, and settle the arrival the player never chose.
    pub fn start(game: &'a Game) -> Result<Self, Stop> {
        let world = World::new(game);
        let state = settle(&world, world.initial())
            .map_err(|room| Stop::EntryCycle(world.rooms[room].clone()))?;
        Ok(Session {
            world,
            state,
            log: Vec::new(),
        })
    }

    /// What the player is looking at.
    pub fn menu(&self) -> Vec<String> {
        shown(&self.world, &self.state).iter().map(spell).collect()
    }

    /// Where they are, as the author wrote it.
    pub fn here(&self) -> &str {
        let room = &self.world.rooms[self.state.room];
        self.world.game.label(room).unwrap_or(room)
    }

    /// Take one turn. The whole of the loop is these fifteen lines, and every one of them is a
    /// call into a component except the two that hold the state.
    pub fn take(&mut self, choice: &str) -> Result<Option<Stop>, Stop> {
        let entries = shown(&self.world, &self.state);
        let offered: Vec<String> = entries.iter().map(spell).collect();
        let Some(i) = offered.iter().position(|s| s == choice) else {
            return Err(Stop::NoSuchChoice(choice.to_string()));
        };
        let binding = entries[i].binding;

        let before = (self.state.room, self.state.mode);
        let (said, fired) = match respond(&self.world, binding, &self.state) {
            Outcome::Fired { says, next } => {
                self.state = settle(&self.world, next)
                    .map_err(|room| Stop::EntryCycle(self.world.rooms[room].clone()))?;
                (says, true)
            }
            Outcome::Refused { says } => (says, false),
        };
        let moved = ((self.state.room, self.state.mode) != before).then(|| match self.state.mode {
            Some(m) => self.world.convs[m].clone(),
            None => self.here().to_string(),
        });
        self.log.push(Turn {
            offered,
            picked: choice.to_string(),
            said: said.map(str::to_string),
            fired,
            moved,
        });
        Ok((fired && binding.ending).then_some(Stop::Ending))
    }

    /// Play a written script to its end, and say why it stopped.
    pub fn play(&mut self, script: &[&str]) -> Stop {
        for choice in script {
            match self.take(choice) {
                Ok(Some(stop)) => return stop,
                Ok(None) => {}
                Err(stop) => return stop,
            }
        }
        Stop::OutOfInput
    }

    /// The session as a player would have seen it.
    pub fn transcript(&self) -> String {
        let mut out = String::new();
        for t in &self.log {
            out.push_str(&format!("> {}\n", t.picked));
            if let Some(line) = &t.said {
                out.push_str(&format!("  {line}\n"));
            }
            if let Some(where_) = &t.moved {
                out.push_str(&format!("  → {where_}\n"));
            }
            // The silent miss, printed rather than swallowed. A player meets it as nothing
            // happening, which is the failure `subsystem/interaction-resolver` exists to prevent
            // and the one thing a transcript can make visible.
            if t.silent() {
                out.push_str("  (nothing)\n");
            }
        }
        out
    }
}
