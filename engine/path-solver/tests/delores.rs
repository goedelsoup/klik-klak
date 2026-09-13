//! realizes: subsystem/path-solver
//!
//! **The only component in this engine whose evidence is borrowed.**
//!
//! Every other one is exercised against `design/specimen/`, five games written in the vocabulary
//! this corpus is about. This one cannot be: a `.game` file has no geometry and
//! `.yidam/decisions/drawn-boundary.yml` is the decision that it never will. So the material here
//! is a shipped game's — *Delores*, two rooms of its 15, transcribed in
//! `crates/design-gate/tests/adjacency.rs` when `.yidam/decisions/adjacency.yml` needed real
//! polygons to measure.
//!
//! **The polygons are read out of that file rather than copied into this one.** A pinned figure is
//! a signal only when its input is the world — `.yidam/decisions/unread-specimen.yml` — and a
//! second transcription of somebody else's coordinates is the purest form of the copy that
//! decision prohibits. What holds the two implementations equal is arithmetic rather than a shared
//! function: this file re-derives the adjacency counts that file pins, from the same rings,
//! without linking to it.
//!
//! `.yidam/decisions/unbuilt-solver.yml`.

use path_solver::{adjacent, contains, solve, Point, Region, Route};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("engine/path-solver is two levels below the root")
        .to_path_buf()
}

/// One room, as the instrument transcribed it.
struct Room {
    name: &'static str,
    boxes: Vec<(String, Vec<Point>)>,
}

/// Pull a `&[Box_]` table out of the instrument's source.
///
/// The shape is fixed by `rustfmt` and the parse is held by the reproduction criteria the catalog
/// states — box count, vertex counts, names — so a table that moved fails here rather than
/// quietly measuring something else.
fn transcribed(src: &str, konst: &str) -> Vec<(String, Vec<Point>)> {
    let head = format!("const {konst}: &[Box_] = &[");
    let from = src.find(&head).expect("the table") + head.len();
    let body = &src[from..from + src[from..].find("\n];").expect("its end")];

    let mut out = Vec::new();
    let mut rest = body;
    while let Some(q) = rest.find('"') {
        let name_end = rest[q + 1..].find('"').expect("a closed name") + q + 1;
        let name = rest[q + 1..name_end].to_string();
        let open = rest[name_end..].find("&[").expect("its ring") + name_end + 2;
        let mut depth = 1usize;
        let mut end = open;
        for (i, c) in rest[open..].char_indices() {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = open + i;
                        break;
                    }
                }
                _ => {}
            }
        }
        out.push((name, vertices(&rest[open..end])));
        rest = &rest[end..];
    }
    out
}

/// Every `(x, y)` in a ring, in order.
fn vertices(s: &str) -> Vec<Point> {
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(o) = rest.find('(') {
        let Some(c) = rest[o..].find(')') else { break };
        let inner = &rest[o + 1..o + c];
        let mut parts = inner.split(',').map(|v| v.trim().parse::<i32>());
        if let (Some(Ok(x)), Some(Ok(y)), None) = (parts.next(), parts.next(), parts.next()) {
            out.push((x, y));
        }
        rest = &rest[o + c..];
    }
    out
}

fn rooms() -> Vec<Room> {
    let src = std::fs::read_to_string(root().join("crates/design-gate/tests/adjacency.rs"))
        .expect("the instrument's transcription");
    vec![
        Room {
            name: "SheriffsOffice",
            boxes: transcribed(&src, "SHERIFFS_OFFICE"),
        },
        Room {
            name: "Nickel",
            boxes: transcribed(&src, "NICKEL"),
        },
    ]
}

fn regions(boxes: &[(String, Vec<Point>)]) -> Vec<Region<'_>> {
    boxes
        .iter()
        .map(|(n, p)| Region {
            name: n,
            polygon: p,
        })
        .collect()
}

fn without<'a>(rs: &[Region<'a>], name: &str) -> Vec<Region<'a>> {
    rs.iter().filter(|r| r.name != name).copied().collect()
}

/// Twice the signed area of a ring, which is exact in integers.
fn twice_area(p: &[Point]) -> i64 {
    (0..p.len())
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % p.len()]);
            a.0 as i64 * b.1 as i64 - b.0 as i64 * a.1 as i64
        })
        .sum::<i64>()
        .abs()
}

fn centroid(p: &[Point]) -> Point {
    (
        (p.iter().map(|v| v.0).sum::<i32>() as f64 / p.len() as f64).round() as i32,
        (p.iter().map(|v| v.1).sum::<i32>() as f64 / p.len() as f64).round() as i32,
    )
}

fn dist(a: Point, b: Point) -> f64 {
    (((a.0 - b.0) as f64).powi(2) + ((a.1 - b.1) as f64).powi(2)).sqrt()
}

/// **The transcription reads, and the component agrees with the instrument about what it says.**
///
/// The catalog states the reproduction criteria for the whole game — *"84 polygons in 15 of 23
/// files, 42 quadrilaterals, vertex counts 4 to 118, 14 named, and exactly two distinct keys
/// across all 84"* — and the same discipline applies to a parse of two of them: a table that moved
/// under this file fails here rather than quietly measuring something else.
///
/// **The last assertion is the one doing real work.** `crates/design-gate/tests/adjacency.rs`
/// pins six pairs and three, from its own private `adjacent`. This file re-derives them from the
/// same rings through `path_solver::adjacent`, and the two never link. Two implementations of the
/// same rule, held equal by arithmetic — which is what was available, because the instrument may
/// not depend on the engine and the engine must not depend on the instrument.
#[test]
fn the_transcription_reads_and_the_two_implementations_agree() {
    let all = rooms();
    let shape: Vec<(usize, Vec<usize>, Vec<&str>, usize)> = all
        .iter()
        .map(|room| {
            let rs = regions(&room.boxes);
            let mut pairs = 0;
            for i in 0..rs.len() {
                for j in i + 1..rs.len() {
                    if adjacent(rs[i].polygon, rs[j].polygon) {
                        pairs += 1;
                    }
                }
            }
            (
                room.boxes.len(),
                room.boxes.iter().map(|(_, p)| p.len()).collect(),
                room.boxes
                    .iter()
                    .filter(|(n, _)| !n.is_empty())
                    .map(|(n, _)| n.as_str())
                    .collect(),
                pairs,
            )
        })
        .collect();

    assert_eq!(
        shape,
        [
            (7, vec![11, 5, 6, 4, 8, 4, 4], vec!["jail", "jail_door"], 6),
            (4, vec![23, 9, 6, 4], vec!["natalie_desk", "desk_path"], 3),
        ],
        "the two rooms as transcribed, and the pairs sharing an exact edge"
    );
}

/// **The failure this component's node describes is the rarer of the two, and the common one is
/// not named.**
///
/// `subsystem/path-solver` says the failure it owns is *"the unreachable destination: a click
/// inside a walkbox that no path from the current one connects to."* That is 47 parts per thousand
/// of the floor in one room and 67 in the other, with the door shut.
///
/// **A third of the clicks inside the floor's own bounding box land on no walkbox at all** — 33%
/// and 34%, exactly, by area rather than by sampling. And a bounding box is the most generous
/// framing available: these floors are strips 33 and 45 pixels tall, and the catalog records room
/// images 180 and 240 pixels tall, so the real share is several times that. [inference]
///
/// So the case the component is documented for is five to seven times rarer than the case it is
/// not, on the kindest possible measurement. That is not an argument that the node is wrong about
/// what is interesting — a wall is a boring failure and a shut door is not — but a component that
/// reports only the interesting one returns nothing for most of what a player clicks.
#[test]
fn the_named_failure_is_the_rarer_of_the_two() {
    let mut shares = Vec::new();
    for room in rooms() {
        let all: Vec<Point> = room.boxes.iter().flat_map(|(_, p)| p.clone()).collect();
        let w = (all.iter().map(|p| p.0).max().expect("pts")
            - all.iter().map(|p| p.0).min().expect("pts")) as i64;
        let h = (all.iter().map(|p| p.1).max().expect("pts")
            - all.iter().map(|p| p.1).min().expect("pts")) as i64;
        let floor: i64 = room.boxes.iter().map(|(_, p)| twice_area(p)).sum();
        let off_floor = 100 - floor * 100 / (2 * w * h);

        let rs = regions(&room.boxes);
        let (gate, orphan) = gated(room.name);
        let shut = without(&rs, gate);
        let cut = twice_area(
            shut.iter()
                .find(|r| r.name == orphan)
                .expect("the orphan")
                .polygon,
        );
        let total: i64 = shut.iter().map(|r| twice_area(r.polygon)).sum();
        shares.push((off_floor, cut * 1000 / total, h));
    }
    assert_eq!(
        shares,
        [(33, 47, 33), (34, 67, 45)],
        "percent of the bounding box that is not floor, per-mille of the floor cut off by a shut \
         door, and how many pixels tall the floor strip is"
    );
}

/// **The actor walks the width of the room to stop ten pixels short, and a player cannot see the
/// difference between that and arriving.**
///
/// This is the question the node left open — *"Doing nothing reads as a broken click; walking as
/// far as possible reads as intent the player did not have. Neither is wrong"* — measured instead
/// of weighed. The two arms are as far apart as they can be: one moves the actor up to 412 pixels
/// and the other moves it none.
///
/// **And the stop is 10 pixels from the goal, in a floor strip 33 pixels tall.** The distance is
/// small because adjacency is exact and the door that was hidden is a quad 7 pixels thick — so
/// "as far as possible" parks the actor against the shut door, which is precisely where a player
/// who was walking to the cell would expect to end up, and precisely where they would end up if
/// the door were open. The arm that reads as *intent the player did not have* is the one that
/// looks like success.
#[test]
fn the_actor_walks_the_room_to_stop_ten_pixels_short() {
    let mut arms = Vec::new();
    for room in rooms() {
        let rs = regions(&room.boxes);
        let (gate, orphan) = gated(room.name);
        let quad = rs.iter().find(|r| r.name == gate).expect("the gate");
        let qy: Vec<i32> = quad.polygon.iter().map(|p| p.1).collect();
        let thickness = qy.iter().max().expect("y") - qy.iter().min().expect("y");

        let shut = without(&rs, gate);
        let op = shut
            .iter()
            .find(|r| r.name == orphan)
            .expect("the orphan")
            .polygon;
        let goal = centroid(op);
        let start = centroid(shut[0].polygon);
        let Route::Unreachable { nearest } = solve(&shut, start, goal) else {
            panic!("{}: the cut-off region is reported reachable", room.name)
        };
        let walked = shut
            .iter()
            .filter(|r| r.name != orphan)
            .flat_map(|r| r.polygon.iter())
            .map(|v| dist(*v, nearest))
            .fold(0.0_f64, f64::max);
        arms.push((
            quad.polygon.len(),
            thickness,
            dist(goal, nearest).round() as i32,
            walked.round() as i32,
        ));
    }
    assert_eq!(
        arms,
        [(4, 7, 10, 412), (4, 7, 13, 288)],
        "the hidden quad's vertices and thickness, how far short the actor stops, and how far it \
         walks to do it"
    );
}

/// **The same answer is a door in one room and a defect in the other, and nothing distinguishes
/// them.**
///
/// With the quad present both rooms are whole and the solver routes through it. With the quad
/// hidden both rooms come apart and the solver returns the identical shape. In `SheriffsOffice`
/// that is the cell door and the game opens it; in `Nickel` `walkboxHidden("desk_path", YES)` sits
/// inside `enter()` and is the only mention of the name in 49 script files, so the region behind
/// the counter is authored and can never be stood on
/// ([delores-source](../../../.yidam/catalog/delores-source.md)).
///
/// `.yidam/decisions/adjacency.yml` reached this from the geometry — *"nothing in the geometry
/// distinguishes a gap that means you may not go there from one left by a mouse that stopped
/// short"*. It arrives here from the other side: not the gap, the **behaviour**. The component
/// cannot tell a door from a bug because the difference was never written anywhere it can read.
#[test]
fn the_same_stop_is_a_door_in_one_room_and_a_defect_in_the_other() {
    for room in rooms() {
        let rs = regions(&room.boxes);
        let (gate, orphan) = gated(room.name);
        let op = rs
            .iter()
            .find(|r| r.name == orphan)
            .expect("the orphan")
            .polygon;
        let goal = centroid(op);
        let start = centroid(rs[0].polygon);

        let Route::Walk(way) = solve(&rs, start, goal) else {
            panic!("{}: open, and the goal is not reachable", room.name)
        };
        assert!(
            way.len() >= 3 && way[0] == start && *way.last().expect("non-empty") == goal,
            "{}: a route through the doorway, {way:?}",
            room.name
        );
        let quad = rs.iter().find(|r| r.name == gate).expect("the gate");
        assert!(
            way[1..way.len() - 1]
                .iter()
                .any(|p| contains(quad.polygon, *p)),
            "{}: and it goes through {gate}",
            room.name
        );

        assert!(
            matches!(
                solve(&without(&rs, gate), start, goal),
                Route::Unreachable { .. }
            ),
            "{}: hidden, and the goal is still reachable",
            room.name
        );
    }
}

/// The quad each room hides, and the region hiding it cuts off.
fn gated(room: &str) -> (&'static str, &'static str) {
    if room == "SheriffsOffice" {
        ("jail_door", "jail")
    } else {
        ("desk_path", "natalie_desk")
    }
}
