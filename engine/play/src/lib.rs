//! composes: verb-menu, interaction-resolver, response-presenter
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
//! **A terminal holds a player, and what it cannot do is time the hold.** Every line goes through
//! `response_presenter::present`, which returns only when the sink does, and there are two sinks
//! here. [`Collect`] returns at once and holds nobody; [`Await`] returns when the player asks for
//! the next thing, which is a real hold and needs no clock. `.yidam/decisions/dismissed-hold.yml`.
//!
//! **The second one is what shows the cost.** Under a dismissed hold the author's longest sentence
//! and their shortest cost the player one keypress each, so *"a line's length is how long the game
//! is held"* stops being true — the lever `subsystem/response-presenter` calls the only one this
//! vocabulary offers is a lever only over a hold that **elapses**, and an elapsing hold needs
//! seconds per character, which no source in the catalog states.
//!
//! **And in a terminal the two cannot both be had, because the dismissal and the choice are one
//! channel.** There is one stream; a hold that elapses cannot be cut short by a player who has
//! finished reading, because the thing that would cut it short is the next line of input and the
//! loop is not reading yet. A frame does not supply the clock — `std::thread::sleep` was always
//! here — it supplies a *second* channel, which is what lets a timed hold be interrupted and is
//! therefore what makes an invented rate survivable.

use game_format::{parse, resolve, Game, State, World};
use interaction_resolver::{resolve as respond, settle_tracking, Outcome};
use response_presenter::{present_all, Perceive};
use std::io::{BufRead, Write};
use verb_menu::{shown, Entry};

/// What this client's perceiving is: collecting the lines, and returning at once.
///
/// The component's contract is that it returns when this does. This returns immediately, which is
/// the whole of what a terminal can promise. `.yidam/decisions/held.yml`.
#[derive(Default)]
struct Collect(Vec<String>);

impl Perceive for Collect {
    fn deliver(&mut self, line: &str) {
        self.0.push(line.to_string());
    }
}

/// **The other perceiving a terminal can do: deliver the line, and return when the player says
/// go on.**
///
/// This is a hold. Nothing in the game advances while it lasts, which is the precondition
/// `.yidam/decisions/held.yml` argued keeps a binding one edge — and it is arrived at with no
/// clock and no invented rate, because the interval is the player's rather than the engine's.
///
/// **What it gives up is the author's lever.** `weight` still reports the line's length and this
/// sink cannot act on it: a seventy-four character line and a four character one are one keypress
/// each. `.yidam/decisions/dismissed-hold.yml`.
///
/// It reads from the same stream the choice is read from, because a terminal has one. That is the
/// whole of what a frame would change.
pub struct Await<R, W> {
    input: R,
    out: W,
    /// how many times the player was held and asked to go on
    pub dismissals: usize,
    /// set once the stream is finished, after which it holds nobody and says so
    pub closed: bool,
}

impl<R: BufRead, W: Write> Await<R, W> {
    pub fn new(input: R, out: W) -> Self {
        Await {
            input,
            out,
            dismissals: 0,
            closed: false,
        }
    }

    /// One line from the stream, or `None` when it is finished.
    ///
    /// The choice and the dismissal come through here both, which is the constraint rather than a
    /// convenience: a terminal cannot be listening for one while it waits for the other.
    pub fn line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.input.read_line(&mut line) {
            Ok(0) | Err(_) => {
                self.closed = true;
                None
            }
            Ok(_) => Some(line.trim().to_string()),
        }
    }

    /// Give it back what it was built on.
    pub fn into_out(self) -> W {
        self.out
    }
}

impl<R: BufRead, W: Write> Perceive for Await<R, W> {
    fn deliver(&mut self, line: &str) {
        let _ = writeln!(self.out, "  {line}");
        if self.closed {
            return;
        }
        let _ = write!(self.out, "  -- ");
        let _ = self.out.flush();
        if self.line().is_some() {
            self.dismissals += 1;
        }
    }
}

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
    /// where the binding that answered was written.
    ///
    /// The one field that lets a transcript be matched against a figure the checker reports:
    /// `crates/reachability` names a stranding by line. `.yidam/decisions/walked-in.yml`.
    pub line: usize,
    /// what the **forced** bindings said, in the order they fired.
    ///
    /// An `on-enter` binding is not chosen and its line is not a response to anything the player
    /// picked. It was dropped for as long as this loop existed —
    /// `interaction_resolver::settle` returns a state — and the longest line in four specimens is
    /// one of these. `.yidam/decisions/walked-in.yml`.
    pub forced: Vec<String>,
    /// how much the presenter held the player for, in characters.
    ///
    /// **Pacing, measured.** `subsystem/response-presenter` says duration is an accident — *"a
    /// line's length is how long the game is held… an author who wants a beat writes a longer
    /// sentence"* — which was an `[inference]` until something counted it.
    /// `.yidam/decisions/held.yml`.
    pub held: usize,
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
        self.said.is_none() && self.forced.is_empty() && self.moved.is_none()
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
        let (state, _) = settle_tracking(&world, world.initial())
            .map_err(|room| Stop::EntryCycle(world.rooms[room].clone()))?;
        Ok(Session {
            world,
            state,
            log: Vec::new(),
        })
    }

    /// The **keys** of what the player is looking at: what a script writes and a transcript
    /// records.
    pub fn menu(&self) -> Vec<String> {
        shown(&self.world, &self.state).iter().map(spell).collect()
    }

    /// The same entries as a player **reads** them, from `verb_menu`'s templates.
    ///
    /// Two spellings, kept apart on purpose: a transcript that moved every time somebody reworded
    /// a template would be pinning prose rather than behaviour, and a script written in phrases
    /// would break on an author renaming a hotspot. `.yidam/decisions/verb-names.yml`.
    pub fn reads(&self) -> Vec<String> {
        shown(&self.world, &self.state)
            .iter()
            .map(|e| e.phrase())
            .collect()
    }

    /// Where they are, as the author wrote it.
    ///
    /// Inside an exchange that is **who** rather than where, which is a distinction nothing here
    /// could draw until the format stopped discarding the `with <actor>` half of a conversation
    /// declaration. It said *"The Quay"* while the player was talking to the keeper.
    /// `.yidam/decisions/verb-names.yml`.
    pub fn here(&self) -> String {
        if let Some(m) = self.state.mode {
            let conv = &self.world.convs[m];
            if let Some(actor) = self.world.game.spoken_with.get(conv) {
                let name = self.world.game.label(actor).unwrap_or(actor);
                return format!("Talking to {name}");
            }
            return conv.clone();
        }
        let room = &self.world.rooms[self.state.room];
        self.world.game.label(room).unwrap_or(room).to_string()
    }

    /// Take one turn. The whole of the loop is these fifteen lines, and every one of them is a
    /// call into a component except the two that hold the state.
    pub fn take(&mut self, choice: &str) -> Result<Option<Stop>, Stop> {
        self.take_with(choice, &mut Collect::default())
    }

    /// The same turn, with the client's own perceiving.
    ///
    /// **The parameter is the whole of this phase's engine change.** The loop built its own
    /// `Collect` and no caller could supply another, so every client that ever ran got the sink
    /// that returns at once — and `.yidam/decisions/held.yml` recorded that as a fact about
    /// terminals. `.yidam/decisions/dismissed-hold.yml`.
    pub fn take_with(
        &mut self,
        choice: &str,
        sink: &mut impl Perceive,
    ) -> Result<Option<Stop>, Stop> {
        let entries = shown(&self.world, &self.state);
        let offered: Vec<String> = entries.iter().map(spell).collect();
        let Some(i) = offered.iter().position(|s| s == choice) else {
            return Err(Stop::NoSuchChoice(choice.to_string()));
        };
        let binding = entries[i].binding;

        let before = (self.state.room, self.state.mode);
        let mut forced: Vec<String> = Vec::new();
        let (said, fired) = match respond(&self.world, binding, &self.state) {
            Outcome::Fired { says, next } => {
                let (settled, ran) = settle_tracking(&self.world, next)
                    .map_err(|room| Stop::EntryCycle(self.world.rooms[room].clone()))?;
                self.state = settled;
                forced = ran
                    .iter()
                    .filter_map(|f| f.says.map(|r| r.text.as_str()))
                    .map(str::to_string)
                    .collect();
                (says, true)
            }
            Outcome::Refused { says } => (says, false),
        };
        // Everything the player is told, in the order they are told it: the chosen binding's line
        // first, then whatever the arrival triggers said. The presenter returns when the sink does
        // and reports what it held them for.
        let held = present_all(
            sink,
            said.into_iter().chain(forced.iter().map(String::as_str)),
        );

        let moved = ((self.state.room, self.state.mode) != before).then(|| self.here());
        self.log.push(Turn {
            offered,
            picked: choice.to_string(),
            said: said.map(str::to_string),
            fired,
            line: binding.line,
            forced,
            held,
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
            for line in &t.forced {
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
