//! composes: verb-menu, interaction-resolver, response-presenter, path-solver
//!
//! A graphical client: geometry is separate data, walking and speech are transient holds,
//! and only the existing session commits game actions. See `design/graphical-client.md`.

use crate::{Session, Stop};
use game_format::{Effect, Game, Kind, Site};
use path_solver::{contains, solve, Point, Region, Route};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Target {
    pub name: String,
    pub rect: [i32; 4],
    pub approach: Point,
    pub art: String,
}

#[derive(Default, Debug)]
pub struct RoomScene {
    pub floor: String,
    pub spawn: Point,
    pub targets: Vec<Target>,
    pub figures: Vec<(String, Point)>,
}

#[derive(Default, Debug)]
pub struct Scene {
    pub floors: BTreeMap<String, Vec<Point>>,
    pub rooms: BTreeMap<String, RoomScene>,
}

impl Scene {
    /// Read the drawing side without extending the game's effect vocabulary.
    pub fn parse(source: &str, game: &Game) -> Result<Self, String> {
        let mut scene = Self::default();
        let number = |s: &str| {
            s.parse::<i32>()
                .map_err(|_| format!("invalid coordinate {s}"))
        };
        let point = |x: &str, y: &str| -> Result<Point, String> {
            let (x, y) = (number(x)?, number(y)?);
            if !(0..=960).contains(&x) || !(0..=540).contains(&y) {
                return Err("coordinate outside 960 × 540 room".into());
            }
            Ok((x, y))
        };
        for (i, line) in source.lines().enumerate() {
            let tokens: Vec<_> = line
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect();
            let bad = || format!("scene line {}: invalid or duplicate declaration", i + 1);
            match tokens.as_slice() {
                [] => {}
                ["floor", name, coordinates @ ..]
                    if coordinates.len() >= 6 && coordinates.len() % 2 == 0 =>
                {
                    let ring = coordinates
                        .chunks(2)
                        .map(|p| point(p[0], p[1]))
                        .collect::<Result<Vec<_>, _>>()?;
                    if scene.floors.insert((*name).into(), ring).is_some() {
                        return Err(bad());
                    }
                }
                ["room", name, floor, x, y] => {
                    if scene
                        .rooms
                        .insert(
                            (*name).into(),
                            RoomScene {
                                floor: (*floor).into(),
                                spawn: point(x, y)?,
                                ..Default::default()
                            },
                        )
                        .is_some()
                    {
                        return Err(bad());
                    }
                }
                ["target", room, name, x, y, width, height, ax, ay, art] => {
                    let r = scene.rooms.get_mut(*room).ok_or_else(bad)?;
                    if ![
                        "door", "cabinet", "kettle", "switch", "tin", "paper", "person",
                    ]
                    .contains(art)
                        || r.targets.iter().any(|t| t.name == *name)
                    {
                        return Err(bad());
                    }
                    let (x, y) = point(x, y)?;
                    let (width, height) = (number(width)?, number(height)?);
                    if width <= 0 || height <= 0 || width > 960 - x || height > 540 - y {
                        return Err(bad());
                    }
                    r.targets.push(Target {
                        name: (*name).into(),
                        rect: [x, y, width, height],
                        approach: point(ax, ay)?,
                        art: (*art).into(),
                    });
                }
                ["figure", room, name, x, y] => {
                    let r = scene.rooms.get_mut(*room).ok_or_else(bad)?;
                    if r.figures.iter().any(|(n, _)| n == name) {
                        return Err(bad());
                    }
                    r.figures.push(((*name).into(), point(x, y)?));
                }
                _ => return Err(bad()),
            }
        }
        if scene.rooms.keys().any(|r| !game.rooms.contains(r))
            || game.rooms.iter().any(|r| !scene.rooms.contains_key(r))
        {
            return Err("scene rooms must match the game".into());
        }
        for (name, r) in &scene.rooms {
            let polygon = scene
                .floors
                .get(&r.floor)
                .ok_or("room names an unknown floor")?;
            if !matches!(
                solve(
                    &[Region {
                        name: &r.floor,
                        polygon
                    }],
                    r.spawn,
                    r.spawn
                ),
                Route::Walk(_)
            ) || !contains(polygon, r.spawn)
            {
                return Err(format!("{name}: invalid floor or spawn"));
            }
            for t in &r.targets {
                if !game.bindings.iter().any(|b| matches!(&b.site, Site::InRoom { room, target } if room == name && target == &t.name)) {
                    return Err(format!("{name}: target {} has no game binding",t.name));
                }
                if !contains(polygon, t.approach) {
                    return Err(format!("{}: approach is off floor", t.name));
                }
            }
            for (actor, p) in &r.figures {
                if game.declared.get(actor).map(|d| d.0) != Some(Kind::Actor)
                    || !contains(polygon, *p)
                {
                    return Err(format!("{name}: invalid actor figure {actor}"));
                }
            }
        }
        for binding in &game.bindings {
            if let Site::InRoom { room, target } = &binding.site {
                if !scene.rooms[room].targets.iter().any(|t| &t.name == target) {
                    return Err(format!("missing drawn target: {room}/{target}"));
                }
            }
            for effect in &binding.effects {
                if let Effect::Move(Some(actor), room) = effect {
                    let r = &scene.rooms[room];
                    if !r.figures.iter().any(|(n, _)| n == actor)
                        && !r.targets.iter().any(|t| &t.name == actor)
                    {
                        return Err(format!("no position for {actor} in {room}"));
                    }
                }
            }
        }
        Ok(scene)
    }
}

/// Browser animation is a hold, not a game timer. No action or save is admitted until it ends.
#[derive(Debug)]
pub enum Hold {
    Idle,
    Walking {
        choice: Option<String>,
        route: Vec<Point>,
    },
    Speaking {
        lines: Vec<String>,
        index: usize,
    },
}

pub struct Screen<'a> {
    pub session: Session<'a>,
    pub scene: Scene,
    pub position: Point,
    pub hold: Hold,
}

impl<'a> Screen<'a> {
    pub fn new(game: &'a Game, scene: Scene) -> Result<Self, String> {
        let session = Session::start_visible(game).map_err(|e| format!("{e:?}"))?;
        let position = scene.rooms[&session.world.rooms[session.state.room]].spawn;
        let hold = if session.opening.is_empty() {
            Hold::Idle
        } else {
            Hold::Speaking {
                lines: session.opening.clone(),
                index: 0,
            }
        };
        Ok(Self {
            session,
            scene,
            position,
            hold,
        })
    }

    fn idle(&self) -> Result<(), String> {
        if !matches!(self.hold, Hold::Idle) {
            return Err("finish walking or dismiss the response first".into());
        }
        Ok(())
    }

    fn walk(&mut self, goal: Point, choice: Option<String>) -> Result<(), String> {
        let room = &self.scene.rooms[&self.session.world.rooms[self.session.state.room]];
        let floor = [Region {
            name: &room.floor,
            polygon: &self.scene.floors[&room.floor],
        }];
        match solve(&floor, self.position, goal) {
            Route::Walk(route) => {
                self.hold = Hold::Walking { choice, route };
                Ok(())
            }
            Route::OffFloor { .. } => Err("That point is outside the walkable floor.".into()),
            Route::Unreachable { .. } => Err("There is no route there.".into()),
            Route::InvalidFloor { .. } => Err("The room has invalid floor geometry.".into()),
        }
    }

    /// Commands are accepted at explicit boundaries; client-side disabled controls are not the guard.
    pub fn command(&mut self, command: &str, value: &str) -> Result<Option<String>, String> {
        match command {
            "frame" => {}
            "choose" => {
                self.idle()?;
                if self.session.ended {
                    return Err("the game has ended".into());
                }
                let entries = verb_menu::shown(&self.session.world, &self.session.state);
                let entry = entries
                    .iter()
                    .find(|e| crate::spell(e) == value)
                    .ok_or("choice is not offered")?;
                if self.session.state.mode.is_some() {
                    self.perform(value)?;
                } else {
                    let room =
                        &self.scene.rooms[&self.session.world.rooms[self.session.state.room]];
                    let target = room
                        .targets
                        .iter()
                        .find(|t| t.name == entry.target)
                        .ok_or("target has no geometry")?;
                    self.walk(target.approach, Some(value.into()))?;
                }
            }
            "walk" => {
                self.idle()?;
                if self.session.ended || self.session.state.mode.is_some() {
                    return Err("walking is unavailable".into());
                }
                let parts: Vec<_> = value.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err("expected a point".into());
                }
                let point = (
                    parts[0].parse().map_err(|_| "invalid x")?,
                    parts[1].parse().map_err(|_| "invalid y")?,
                );
                self.walk(point, None)?;
            }
            "arrive" => {
                if !matches!(self.hold, Hold::Walking { .. }) {
                    return Err("no walk is pending".into());
                }
                let Hold::Walking { choice, route } = std::mem::replace(&mut self.hold, Hold::Idle)
                else {
                    unreachable!()
                };
                self.position = *route.last().ok_or("empty route")?;
                if let Some(choice) = choice {
                    self.perform(&choice)?;
                }
            }
            "dismiss" => {
                let Hold::Speaking { lines, index } = &mut self.hold else {
                    return Err("no response is pending".into());
                };
                *index += 1;
                if *index == lines.len() {
                    self.hold = Hold::Idle;
                }
            }
            "save" => {
                self.idle()?;
                return self.session.save().map(Some);
            }
            "load" => {
                self.idle()?;
                self.session.restore(value)?;
                self.position =
                    self.scene.rooms[&self.session.world.rooms[self.session.state.room]].spawn;
            }
            _ => return Err("unknown command".into()),
        }
        Ok(None)
    }

    fn perform(&mut self, choice: &str) -> Result<(), String> {
        let room = self.session.state.room;
        let stop = self.session.take(choice).map_err(|e| format!("{e:?}"))?;
        if room != self.session.state.room {
            self.position =
                self.scene.rooms[&self.session.world.rooms[self.session.state.room]].spawn;
        }
        if let Some(turn) = self.session.log.last() {
            let lines: Vec<_> = turn.said.iter().chain(&turn.forced).cloned().collect();
            if !lines.is_empty() {
                self.hold = Hold::Speaking { lines, index: 0 };
            }
        }
        if stop == Some(Stop::Ending) && matches!(self.hold, Hold::Idle) {
            self.hold = Hold::Speaking {
                lines: vec!["The end.".into()],
                index: 0,
            };
        }
        Ok(())
    }

    pub fn frame(&self, message: &str, saved: Option<&str>) -> String {
        let w = &self.session.world;
        let s = &self.session.state;
        let room = &self.scene.rooms[&w.rooms[s.room]];
        let entries = verb_menu::shown(w, s);
        let menu = entries
            .iter()
            .map(|e| {
                format!(
                    "{{\"key\":{},\"target\":{},\"text\":{},\"object\":{}}}",
                    json(&crate::spell(e)),
                    json(e.target),
                    json(&e.phrase()),
                    e.object.map(json).unwrap_or("null".into())
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let targets = room
            .targets
            .iter()
            .filter(|t| {
                if let Some(i) = w.takeable.iter().position(|n| n == &t.name) {
                    if s.taken[i] {
                        return false;
                    }
                }
                if let Some(i) = w.actors.iter().position(|n| n == &t.name) {
                    if s.actors[i] != s.room {
                        return false;
                    }
                }
                true
            })
            .map(|t| {
                format!(
                    "{{\"name\":{},\"label\":{},\"rect\":{:?},\"art\":{},\"active\":{}}}",
                    json(&t.name),
                    json(w.game.label(&t.name).unwrap_or(&t.name)),
                    t.rect,
                    json(&t.art),
                    entries.iter().any(|e| e.target == t.name)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let figures = room
            .figures
            .iter()
            .filter(|(n, _)| {
                w.actors
                    .iter()
                    .position(|a| a == n)
                    .map(|i| s.actors[i] == s.room)
                    .unwrap_or_else(|| w.game.actor_start.get(n) == Some(&w.rooms[s.room]))
            })
            .map(|(n, p)| {
                format!(
                    "{{\"label\":{},\"point\":{}}}",
                    json(w.game.label(n).unwrap_or(n)),
                    point_json(*p)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let inventory = w
            .items
            .iter()
            .zip(&s.items)
            .filter(|(_, n)| **n > 0)
            .map(|(n, count)| {
                format!(
                    "{{\"name\":{},\"label\":{},\"count\":{count}}}",
                    json(n),
                    json(w.game.label(n).unwrap_or(n))
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let (hold, line, route) = match &self.hold {
            Hold::Idle => ("idle", String::new(), "[]".into()),
            Hold::Walking { route, .. } => ("walking", String::new(), points_json(route)),
            Hold::Speaking { lines, index } => ("speaking", lines[*index].clone(), "[]".into()),
        };
        format!("{{\"room\":{},\"title\":{},\"position\":{},\"floor\":{},\"targets\":[{targets}],\"figures\":[{figures}],\"menu\":[{menu}],\"inventory\":[{inventory}],\"hold\":{},\"line\":{},\"route\":{route},\"message\":{},\"saved\":{},\"ended\":{},\"conversation\":{}}}",json(&w.rooms[s.room]),json(&self.session.here()),point_json(self.position),points_json(&self.scene.floors[&room.floor]),json(hold),json(&line),json(message),saved.map(json).unwrap_or("null".into()),self.session.ended,s.mode.is_some())
    }
}

fn point_json(p: Point) -> String {
    format!("[{},{}]", p.0, p.1)
}
fn points_json(points: &[Point]) -> String {
    format!(
        "[{}]",
        points
            .iter()
            .map(|p| point_json(*p))
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// JSON strings are data, including authored quotes, control characters and non-ASCII text.
pub fn json(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
