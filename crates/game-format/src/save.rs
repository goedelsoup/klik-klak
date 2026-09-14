//! Named snapshots of settled world state. No arrival event is implied by restoration.
//!
//! Version one accepts the same canonical game (declaration order may change), and refuses
//! changed content rather than guessing a migration. The compatibility checksum is not a
//! security signature; names, vector widths and value ranges are independently validated.

use crate::{print, State, World};
use std::collections::BTreeMap;

fn identity(w: &World) -> u64 {
    let source = print(w.game);
    let (top, body) = source.split_once("\n\n").unwrap_or((&source, ""));
    let mut declarations: Vec<_> = top.lines().collect();
    declarations.sort_unstable();
    let canonical = format!("{}\n\n{body}", declarations.join("\n"));
    canonical.bytes().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

fn validate(w: &World, s: &State) -> Result<(), String> {
    if s.room >= w.rooms.len() || s.mode.is_some_and(|m| m >= w.convs.len()) {
        return Err("save names an invalid room or conversation".into());
    }
    for (actual, expected) in [
        (s.items.len(), w.items.len()),
        (s.taken.len(), w.takeable.len()),
        (s.flags.len(), w.flags.len()),
        (s.counters.len(), w.counters.len()),
        (s.chosen.len(), w.options.len()),
        (s.actors.len(), w.actors.len()),
    ] {
        if actual != expected {
            return Err("save state has the wrong vector width".into());
        }
    }
    if s.items
        .iter()
        .zip(&w.caps)
        .any(|(v, cap)| *v < 0 || v > cap)
        || s.actors.iter().any(|r| *r >= w.rooms.len())
    {
        return Err("save has an invalid item count or actor room".into());
    }
    for (name, value) in w.counters.iter().zip(&s.counters) {
        let c = w
            .game
            .counters
            .iter()
            .find(|c| &c.name == name)
            .expect("declared");
        if *value < c.lo || *value > c.hi {
            return Err(format!("counter {name} is out of bounds"));
        }
    }
    Ok(())
}

type Rows = BTreeMap<(String, String), String>;

fn rows(w: &World, s: &State) -> Rows {
    let mut out = Rows::new();
    out.insert(("room".into(), "player".into()), w.rooms[s.room].clone());
    out.insert(
        ("mode".into(), "player".into()),
        s.mode
            .map(|m| format!("+{}", w.convs[m]))
            .unwrap_or("-".into()),
    );
    fn add<T: ToString>(out: &mut Rows, kind: &str, names: &[String], values: &[T]) {
        for (name, value) in names.iter().zip(values) {
            out.insert((kind.into(), name.clone()), value.to_string());
        }
    }
    add(&mut out, "item", &w.items, &s.items);
    add(&mut out, "taken", &w.takeable, &s.taken);
    add(&mut out, "flag", &w.flags, &s.flags);
    add(&mut out, "counter", &w.counters, &s.counters);
    add(&mut out, "chosen", &w.options, &s.chosen);
    let rooms: Vec<_> = s.actors.iter().map(|r| &w.rooms[*r]).collect();
    add(&mut out, "actor", &w.actors, &rooms);
    out
}

/// Encode the settled state using canonical names, never runtime slots.
pub fn save(w: &World, s: &State) -> Result<String, String> {
    validate(w, s)?;
    let mut text = format!("klik-save 1 {:016x}\n", identity(w));
    for ((kind, name), value) in rows(w, s) {
        text.push_str(&format!("{kind}\t{name}\t{value}\n"));
    }
    Ok(text)
}

/// Decode and validate completely before handing a state to a client. Never run entry bindings.
pub fn restore(w: &World, text: &str) -> Result<State, String> {
    let mut lines = text.lines();
    if lines.next() != Some(format!("klik-save 1 {:016x}", identity(w)).as_str()) {
        return Err("save version or game content does not match".into());
    }
    let mut input = Rows::new();
    for line in lines {
        let parts: Vec<_> = line.split('\t').collect();
        if parts.len() != 3 {
            return Err("malformed save row".into());
        }
        if input
            .insert((parts[0].into(), parts[1].into()), parts[2].into())
            .is_some()
        {
            return Err("duplicate save field".into());
        }
    }
    let mut s = w.initial();
    if input.keys().ne(rows(w, &s).keys()) {
        return Err("save fields do not match the game".into());
    }
    let get = |kind: &str, name: &str| input[&(kind.to_string(), name.to_string())].as_str();
    let locate = |names: &[String], name: &str| {
        names
            .iter()
            .position(|n| n == name)
            .ok_or_else(|| format!("unknown saved name: {name}"))
    };
    s.room = locate(&w.rooms, get("room", "player"))?;
    s.mode = match get("mode", "player") {
        "-" => None,
        v => Some(locate(
            &w.convs,
            v.strip_prefix('+').ok_or("invalid saved conversation")?,
        )?),
    };
    fn read<'a, T: std::str::FromStr>(
        names: &[String],
        kind: &str,
        get: &impl Fn(&str, &str) -> &'a str,
    ) -> Result<Vec<T>, String> {
        names
            .iter()
            .map(|n| {
                get(kind, n)
                    .parse()
                    .map_err(|_| format!("invalid {kind} value for {n}"))
            })
            .collect()
    }
    s.items = read(&w.items, "item", &get)?;
    s.taken = read(&w.takeable, "taken", &get)?;
    s.flags = read(&w.flags, "flag", &get)?;
    s.counters = read(&w.counters, "counter", &get)?;
    s.chosen = read(&w.options, "chosen", &get)?;
    s.actors = w
        .actors
        .iter()
        .map(|a| locate(&w.rooms, get("actor", a)))
        .collect::<Result<_, _>>()?;
    validate(w, &s)?;
    Ok(s)
}
