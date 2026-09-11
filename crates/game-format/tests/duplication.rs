//! **How much of a repeated shape is writing?** — the measurement that settles what
//! `.yidam/decisions/duplication.yml` left open.
//!
//! That record split duplication into *generation* (one shape, many instances) and *identity*
//! (several regions that are one thing), admitted a generation primitive under the amended rule
//! in `AGENTS.md`, and closed on a worry: `exit` is one archetype for one shape, and **an
//! archetype per shape does not reach shapes nobody anticipated** — which was the argument
//! against archetypes in the first place.
//!
//! The worry assumes the difficulty is enumerating shapes. It is not. Every figure below is a
//! count of the same four authored specimens, and together they say the shapes are not scarce —
//! **seventy of ninety-one bindings share one with something else** — while the thing that would
//! make factoring them worth anything is: there is exactly one repeated shape in this language
//! whose repeated part carries no prose, and it already has its archetype.
//!
//! The unit throughout is a *response block*: a `<verb>:` line and the indented lines under it.
//! **Frame** is every non-space character outside a string literal; **prose** is the characters
//! inside one. The ratio is what an author gets back for what they had to type.

use game_format::{Effect, Pred, Site};
use std::collections::{BTreeMap, BTreeSet};

const SPECIMENS: [(&str, &str); 4] = [
    ("ferry", include_str!("../../../design/specimen/ferry.game")),
    (
        "lighthouse",
        include_str!("../../../design/specimen/lighthouse.game"),
    ),
    (
        "nightwatch",
        include_str!("../../../design/specimen/nightwatch.game"),
    ),
    (
        "patrol",
        include_str!("../../../design/specimen/patrol.game"),
    ),
];

fn load(name: &str, src: &str) -> game_format::Game {
    let (mut g, mut p) = game_format::parse(src, name);
    p.extend(game_format::resolve(&mut g));
    let msgs: Vec<String> = p.into_iter().map(|x| x.to_string()).collect();
    assert!(msgs.is_empty(), "{name} does not resolve: {msgs:?}");
    g
}

/// A binding with every *name* removed. What is left is the shape an archetype would factor: the
/// verb, whether there is a second object, the structure of the guard, the sequence of effects.
fn shape(b: &game_format::Binding) -> String {
    fn pred(p: &Pred) -> String {
        match p {
            Pred::Flag(_) => "flag".into(),
            Pred::Carrying(_) => "carrying".into(),
            Pred::Chosen(_) => "chosen".into(),
            Pred::Cmp(_, c, _) => format!("cmp{}", c.spelling()),
            Pred::Count(_, c, _) => format!("count{}", c.spelling()),
            Pred::At(_, _) => "at".into(),
            Pred::Not(x) => format!("!{}", pred(x)),
            Pred::And(a, b) => format!("({} & {})", pred(a), pred(b)),
            Pred::Or(a, b) => format!("({} | {})", pred(a), pred(b)),
        }
    }
    fn effect(e: &Effect) -> &'static str {
        match e {
            Effect::SetFlag(_) => "set",
            Effect::ClearFlag(_) => "clear",
            Effect::SetCounter(_, _) => "count=",
            Effect::Give(_, _) => "give",
            Effect::Spend(_, _) => "spend",
            Effect::Move(None, _) => "move-player",
            Effect::Move(Some(_), _) => "move-actor",
            Effect::Enter(_) => "enter",
            Effect::Leave => "leave",
        }
    }
    let site = match &b.site {
        Site::InRoom { .. } => "room",
        Site::InConversation { .. } => "conv",
        Site::OnEnter { .. } => "enter",
    };
    format!(
        "{site} {} {}| when {} | guards [{}] | do [{}]{}",
        b.verb,
        if b.object.is_some() { "<obj> " } else { "" },
        b.when.as_ref().map(pred).unwrap_or_else(|| "-".into()),
        b.guards().map(pred).collect::<Vec<_>>().join(", "),
        b.effects.iter().map(effect).collect::<Vec<_>>().join(", "),
        if b.ending { " ENDING" } else { "" },
    )
}

fn shapes() -> BTreeMap<String, usize> {
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for (name, src) in SPECIMENS {
        for b in &load(name, src).bindings {
            *m.entry(shape(b)).or_default() += 1;
        }
    }
    m
}

/// A response block as it is written: the header verb, and the frame and prose under it.
struct Block {
    verb: String,
    frame: usize,
    prose: usize,
}

/// Every `<verb>:`, `option <name> "<prompt>"` and `on-enter:` in the four specimens, with the
/// indented lines belonging to each. Header prose counts — an option's prompt is the line the
/// player reads.
fn blocks() -> Vec<Block> {
    let verbs = [
        "look-at", "pick-up", "walk-to", "talk-to", "use-with", "give-to",
    ];
    let mut out = vec![];
    for (_, src) in SPECIMENS {
        let lines: Vec<&str> = src.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            let head = t
                .strip_suffix(':')
                .map(|h| h.split_whitespace().next().unwrap_or(""));
            let verb = match head {
                Some(h) if verbs.contains(&h) || h == "on-enter" => Some(h.to_string()),
                _ if t.starts_with("option ") => Some("option".to_string()),
                _ => None,
            };
            let Some(verb) = verb else {
                i += 1;
                continue;
            };
            let indent = lines[i].len() - lines[i].trim_start().len();
            let mut body = vec![lines[i]];
            let mut j = i + 1;
            while j < lines.len() {
                let l = lines[j];
                if l.trim().is_empty() {
                    j += 1;
                    continue;
                }
                if l.len() - l.trim_start().len() <= indent {
                    break;
                }
                body.push(l);
                j += 1;
            }
            let (mut frame, mut prose) = (0, 0);
            for l in body {
                let l = l.trim();
                if l.starts_with('#') {
                    continue;
                }
                let mut quoted = false;
                for c in l.chars() {
                    match c {
                        '"' => quoted = !quoted,
                        _ if quoted => prose += 1,
                        ' ' => {}
                        _ => frame += 1,
                    }
                }
            }
            out.push(Block { verb, frame, prose });
            i = j;
        }
    }
    out
}

/// **Structural repetition is pervasive**, which is the half of the picture the open was right
/// about. Ninety-one bindings, twenty-nine shapes between them; the two commonest are a bare
/// `look-at` at thirty-one sites and a bare `walk-to` that moves at sixteen.
#[test]
fn seventy_of_ninety_one_bindings_share_their_shape_with_another() {
    let m = shapes();
    let total: usize = m.values().sum();
    let repeated: usize = m.values().filter(|n| **n > 1).sum();
    assert_eq!((total, m.len(), repeated), (91, 29, 70));

    let mut top: Vec<(&String, &usize)> = m.iter().collect();
    top.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    assert_eq!(*top[0].1, 31, "{}", top[0].0);
    assert!(top[0]
        .0
        .starts_with("room look-at | when - | guards [] | do []"));
    assert_eq!(*top[1].1, 16, "{}", top[1].0);
    assert!(top[1]
        .0
        .starts_with("room walk-to | when - | guards [] | do [move-player]"));
}

/// **And it is almost all frame around writing nobody else did.** One hundred and eighty-six
/// string literals in the four specimens and one hundred and eighty-four of them are different;
/// the two that repeat are four and five words long.
///
/// That is what a shape census cannot see, and it is why the count above does not settle
/// anything on its own: two bindings with the same shape are the same binding only if the shape
/// is what the author was writing.
#[test]
fn the_writing_in_an_authored_game_is_all_different() {
    let mut all = vec![];
    for (_, src) in SPECIMENS {
        for l in src.lines() {
            let t = l.trim();
            if t.starts_with('#') {
                continue;
            }
            for (n, part) in t.split('"').enumerate() {
                if n % 2 == 1 {
                    all.push(part.to_string());
                }
            }
        }
    }
    let distinct: BTreeSet<&String> = all.iter().collect();
    assert_eq!((all.len(), distinct.len()), (186, 184));
}

/// **The finding.** Sixteen response blocks in six hundred and sixty-four lines contain no prose
/// at all. Fifteen are the bare exit — `walk-to:` over `do move <room>` — and the sixteenth is
/// the corpus's one conversation opener, which appears once.
///
/// `.yidam/decisions/exit-archetype.yml` admitted the archetype for the fifteen on a census of
/// two independent bodies of authored work, before this ratio had been measured. It picks out
/// the same shape and nothing else, and "a primitive with no site" is what this corpus has
/// refused four times.
#[test]
fn the_bare_exit_is_the_only_prose_free_shape_in_the_language() {
    let free: Vec<Block> = blocks().into_iter().filter(|b| b.prose == 0).collect();
    assert_eq!(free.len(), 16);
    assert_eq!(free.iter().filter(|b| b.verb == "walk-to").count(), 15);
    assert_eq!(free.iter().filter(|b| b.verb == "talk-to").count(), 1);
}

/// **Ranked, the criterion picks out one verb and it is the one that got the archetype.**
/// `walk-to` is the only verb in this language whose blocks carry less writing than frame, and
/// it is the second commonest. `look-at` is the commonest by nearly double and the most
/// prose-dense of all, so an archetype there would save the frame and leave the work.
///
/// So the answer to *an archetype per shape does not reach shapes nobody anticipated* is that
/// the shapes do not have to be anticipated. The test is mechanical, it runs on any candidate,
/// and applied to this language it returns one.
#[test]
fn walk_to_is_the_only_verb_that_costs_more_frame_than_it_carries_writing() {
    let mut per: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for b in blocks() {
        let e = per.entry(b.verb.clone()).or_default();
        e.0 += 1;
        e.1 += b.frame;
        e.2 += b.prose;
    }
    let thin: Vec<&String> = per
        .iter()
        .filter(|(_, (_, f, p))| p < f)
        .map(|(v, _)| v)
        .collect();
    assert_eq!(thin, vec!["walk-to"], "{per:?}");

    let (n, frame, prose) = per["look-at"];
    assert_eq!(n, 42, "look-at is the commonest verb in the corpus");
    assert!(
        prose > 4 * frame,
        "and the most prose-dense: {prose} / {frame}"
    );

    let (total, frame, prose) = per
        .values()
        .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));
    assert_eq!(total, 91);
    assert!(
        prose > 2 * frame - frame / 10,
        "an authored game is roughly two characters of writing per character of frame: \
         {prose} prose against {frame} frame"
    );
}
