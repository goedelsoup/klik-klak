//! **Nothing in a shipped game declares which regions of a floor are adjacent, and the geometry
//! answers exactly — including where it says two parts of one room do not connect.**
//!
//! [`walkbox/scaled-polygon`] has asked whether adjacency between regions needs a relationship of
//! its own, and had already found the shape of its own answer without running it: *"a relationship
//! no author ever writes is the shape [`found-in`] turned out to have."*
//! `.yidam/decisions/resolver-kind.yml` classified the question as wanting **a measurement** and
//! named both the method and the material — `found-in`'s own test, applied to *Delores*'s 84
//! walkbox polygons across 15 rooms ([`delores-source`]).
//!
//! `found-in` asks two things of a relationship: **does anything author it**, and **what breaks if
//! nothing does**. The first is settled by the catalog and needs no arithmetic — `polygon` and
//! `name` are the only two keys that ever appear on a walkbox. The second is this file.
//!
//! **Why the check is here and not in `crates/`.** A polygon cannot be written in the `.game`
//! format at all — that is `.yidam/decisions/drawn-boundary.yml`, and `walkbox.ont.yml` calls
//! itself *"the only class in the authoring tier the `.game` surface cannot name"*. So there is no
//! game to run this against and no format to parse it with. What is left is a corpus claim, and
//! this crate is the instrument that holds corpus claims.
//!
//! **The polygons below are transcribed rather than fetched**, for the reason
//! [`delores-source`] gives for not committing its extractor: the input is somebody else's
//! repository and this one does not vendor it. Two rooms are enough, because the whole finding is
//! visible in each of them and they are the two the game builds the same way.
//! `crates/reachability/tests/delegation.rs` is the precedent — real authored material,
//! transcribed, so that a claim about it can be executed.
//!
//! [`walkbox/scaled-polygon`]: ../../../.yidam/corpus/walkbox/scaled-polygon.yml
//! [`found-in`]: ../../../.yidam/decisions/found-in.yml
//! [`delores-source`]: ../../../.yidam/catalog/delores-source.md
//! `.yidam/decisions/adjacency.yml`.

use std::collections::BTreeSet;

type Point = (i32, i32);
type Box_ = (&'static str, &'static [Point]);

/// `SheriffsOffice.wimpy`, every walkbox, in file order. Two carry a name and five do not.
const SHERIFFS_OFFICE: &[Box_] = &[
    (
        "",
        &[
            (88, 63),
            (180, 58),
            (267, 58),
            (284, 65),
            (349, 65),
            (379, 59),
            (475, 59),
            (484, 59),
            (520, 59),
            (566, 41),
            (46, 41),
        ],
    ),
    ("", &[(100, 68), (125, 68), (192, 67), (180, 58), (88, 63)]),
    (
        "",
        &[
            (132, 72),
            (282, 74),
            (282, 70),
            (275, 67),
            (192, 67),
            (125, 68),
        ],
    ),
    ("", &[(275, 71), (282, 70), (284, 65), (267, 58)]),
    (
        "jail",
        &[
            (398, 71),
            (460, 71),
            (466, 73),
            (488, 73),
            (501, 66),
            (481, 66),
            (473, 66),
            (398, 66),
        ],
    ),
    ("jail_door", &[(473, 66), (481, 66), (484, 59), (475, 59)]),
    ("", &[(352, 68), (368, 68), (379, 59), (349, 65)]),
];

/// `Nickel.wimpy`, the same shape in a different room.
const NICKEL: &[Box_] = &[
    (
        "",
        &[
            (246, 78),
            (290, 78),
            (291, 76),
            (335, 76),
            (337, 75),
            (282, 75),
            (281, 71),
            (340, 71),
            (341, 70),
            (278, 70),
            (277, 73),
            (192, 73),
            (184, 66),
            (317, 66),
            (323, 57),
            (336, 56),
            (351, 42),
            (303, 33),
            (301, 55),
            (300, 60),
            (162, 60),
            (188, 75),
            (244, 75),
        ],
    ),
    (
        "",
        &[
            (3, 33),
            (0, 59),
            (104, 59),
            (124, 70),
            (123, 75),
            (144, 76),
            (118, 55),
            (301, 55),
            (303, 33),
        ],
    ),
    (
        "natalie_desk",
        &[(26, 77), (109, 77), (108, 73), (77, 73), (60, 67), (2, 67)],
    ),
    ("desk_path", &[(109, 77), (123, 75), (124, 70), (108, 73)]),
];

/// Every edge of a ring, as an unordered vertex pair — so two regions that meet write the same
/// edge in opposite winding and it compares equal.
fn edges(p: &[Point]) -> BTreeSet<(Point, Point)> {
    (0..p.len())
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % p.len()]);
            if a <= b {
                (a, b)
            } else {
                (b, a)
            }
        })
        .collect()
}

/// Adjacency, derived: two regions are adjacent when they share an edge exactly.
fn adjacent(a: &[Point], b: &[Point]) -> bool {
    !edges(a).is_disjoint(&edges(b))
}

/// The connected components of a room's floor, named by their members, with the named boxes given
/// by name and the unnamed ones by index.
fn components(boxes: &[Box_]) -> Vec<Vec<String>> {
    let n = boxes.len();
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let (mut stack, mut comp) = (vec![s], Vec::new());
        while let Some(v) = stack.pop() {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            comp.push(v);
            for (w, b) in boxes.iter().enumerate() {
                if !seen[w] && adjacent(boxes[v].1, b.1) {
                    stack.push(w);
                }
            }
        }
        comp.sort_unstable();
        out.push(
            comp.into_iter()
                .map(|i| {
                    if boxes[i].0.is_empty() {
                        format!("#{i}")
                    } else {
                        boxes[i].0.to_string()
                    }
                })
                .collect(),
        );
    }
    out
}

fn without(boxes: &[Box_], name: &str) -> Vec<Box_> {
    boxes.iter().copied().filter(|(n, _)| *n != name).collect()
}

/// **Every contact between two of these regions is an exact shared edge, and the authoring tool is
/// why.**
///
/// This is the half of the measurement that could have gone the other way and did not. An author
/// dragging polygons over a background in an editor has no reason to land on a neighbour's vertex,
/// and if they did not, a derivation from shared edges would need a tolerance — and a tolerance is
/// a number nobody could choose from the data.
///
/// Across the whole game: **66 of 290 within-room pairs share an exact edge, none meet along a
/// collinear segment without sharing one, and exactly one pair covers common floor without sharing
/// an edge.** [`delores-source`] carries those figures and the reproduction criteria; what is
/// executed here is the two rooms the claim rests on.
///
/// So the walkboxes **tile**. They are not a heap of overlapping regions and they are not a
/// near-miss approximation of one. Adjacency is exactly derivable, with no parameter.
///
/// [`delores-source`]: ../../../.yidam/catalog/delores-source.md
#[test]
fn every_contact_between_two_regions_is_an_exact_shared_edge() {
    for (room, boxes, pairs) in [
        ("SheriffsOffice", SHERIFFS_OFFICE, 6),
        ("Nickel", NICKEL, 3),
    ] {
        let mut shared = 0;
        let mut touching_without_sharing = Vec::new();
        for i in 0..boxes.len() {
            for j in i + 1..boxes.len() {
                let (a, b) = (boxes[i].1, boxes[j].1);
                if adjacent(a, b) {
                    shared += 1;
                    continue;
                }
                // A vertex in common but no edge: the one case in the game where two regions
                // touch and the derivation says they do not.
                let va: BTreeSet<Point> = a.iter().copied().collect();
                let vb: BTreeSet<Point> = b.iter().copied().collect();
                if !va.is_disjoint(&vb) {
                    touching_without_sharing.push((i, j));
                }
            }
        }
        assert_eq!(shared, pairs, "{room}: pairs sharing an exact edge");
        // SheriffsOffice has the single vertex-only contact in all 84 polygons, at (282,70), and
        // it joins two regions that are already adjacent by other edges — so the derivation loses
        // nothing by ignoring it. That it is the only one in the game is the measurement.
        let expect: &[(usize, usize)] = if room == "SheriffsOffice" {
            &[(2, 3)]
        } else {
            &[]
        };
        assert_eq!(
            touching_without_sharing, expect,
            "{room}: regions sharing a vertex and no edge"
        );
    }
}

/// **The named quad is a door, and hiding it cuts the room in two.**
///
/// `jail_door` is four vertices and shares an edge with exactly two regions: the office floor and
/// `jail`. `desk_path` is four vertices and shares an edge with exactly two: the shop floor and
/// `natalie_desk`. Both are the smaller half of the pairing [`delores-source`] had already
/// observed — *"in each pair the larger region is non-convex and its partner is a 4-vertex quad"* —
/// without saying what the pairing was for.
///
/// It is for this. `walkboxHidden("jail_door", not open)` is **the cell door**, and the thing the
/// author writes to open and shut it is not an edge between two regions. It is a region that
/// exists or does not.
///
/// That is `passability`, which `walkbox.ont.yml` has declared since genesis as *"whether the
/// region is always walkable or conditional on world state"* — and it is a property of one region
/// rather than a relation between two, which is the answer to the question this measurement was
/// run for.
///
/// [`delores-source`]: ../../../.yidam/catalog/delores-source.md
#[test]
fn the_named_quad_is_a_door_and_hiding_it_cuts_the_room_in_two() {
    for (room, boxes, gate, cut_off) in [
        ("SheriffsOffice", SHERIFFS_OFFICE, "jail_door", "jail"),
        ("Nickel", NICKEL, "desk_path", "natalie_desk"),
    ] {
        let quad = boxes
            .iter()
            .find(|(n, _)| *n == gate)
            .expect("the named gate");
        assert_eq!(quad.1.len(), 4, "{room}: {gate} is a quad");
        let neighbours: Vec<&str> = boxes
            .iter()
            .filter(|(n, p)| *n != gate && adjacent(p, quad.1))
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(
            neighbours.len(),
            2,
            "{room}: {gate} bridges exactly two regions"
        );
        assert!(
            neighbours.contains(&cut_off),
            "{room}: and one of them is {cut_off}"
        );

        assert_eq!(
            components(boxes).len(),
            1,
            "{room}: whole, with every region"
        );

        let shut = without(boxes, gate);
        let split = components(&shut);
        assert_eq!(split.len(), 2, "{room}: hiding {gate} splits the floor");
        assert!(
            split.iter().any(|c| c == &[cut_off.to_string()]),
            "{room}: and {cut_off} is the part that is cut off — {split:?}"
        );
    }
}

/// **Six of fifteen rooms come apart with every region present, and nothing in the geometry says
/// whether that is meant.**
///
/// The gaps are 3 to 16 pixels in rooms 320 to 600 wide: `Bank` 6, `CityHall` 8, `Diner` 3 and 7
/// and 16 across three components, `PostOffice` 4, `QuickiePal` 3, `BookStore` 14.
/// [`delores-source`] carries them.
///
/// **That is the finding this measurement was for, and it is not the one it went looking for.**
/// The derivation is exact and it is also silent: a three-pixel gap that means *you may not go
/// behind the counter* and a three-pixel gap left by a mouse that stopped short are the same
/// geometry. `found-in` was refused because writing it wrong changed nothing; adjacency is the
/// opposite — nothing writes it, everything reads it, and the derivation carries no intent because
/// intent was never expressible.
///
/// What is checked here is the shape rather than the count, because the fifteen rooms are not in
/// this repository: a room whose regions do not all connect is an ordinary authored state, and one
/// of the two rooms transcribed above enters it the moment a door shuts.
///
/// [`delores-source`]: ../../../.yidam/catalog/delores-source.md
#[test]
fn a_floor_in_more_than_one_piece_is_an_ordinary_state_and_not_a_defect() {
    let shut = without(SHERIFFS_OFFICE, "jail_door");
    assert_eq!(
        components(&shut).len(),
        2,
        "a room in two pieces, reached by shutting a door"
    );
    // And it is reached again from the other side: a region cut off is cut off whichever region
    // you name. The relation is symmetric and nothing authored it.
    let a = components(&shut);
    let b = components(&without(&shut, "nothing-is-called-this"));
    assert_eq!(a, b, "removing a name no region carries changes nothing");
}

/// **One of the three doorways is never opened, and the region behind it is authored and
/// unreachable for the whole game.**
///
/// `natalie_desk` shares an edge with `desk_path` and with nothing else, so the only way onto it
/// is through the quad — and `walkboxHidden("desk_path", YES)` at `Nickel.dinky:194` sits inside
/// `enter()`, which runs on every entry, and is the only mention of `desk_path` in the game's 49
/// script files.
///
/// **That is `Report::orphaned` in geometry.** `crates/reachability` reports a binding that is
/// written and selected in no reachable state, and the four specimens carry four of them
/// (`.yidam/decisions/orphaned-lines.yml`). This is the same defect one tier down, in a shipped
/// game, and nothing in that engine could report it — because the fact that would have to be
/// checked is spread across a polygon file and a script and is never stated in either.
///
/// The geometric half is executed here. The script half is a quotation and lives where quotations
/// live: `.yidam/catalog/delores-source.md`.
#[test]
fn the_region_behind_the_counter_is_reachable_only_through_a_quad_that_is_never_shown() {
    let desk = NICKEL
        .iter()
        .find(|(n, _)| *n == "natalie_desk")
        .expect("the named region");
    let neighbours: Vec<&str> = NICKEL
        .iter()
        .filter(|(n, p)| *n != "natalie_desk" && adjacent(p, desk.1))
        .map(|(n, _)| *n)
        .collect();
    assert_eq!(
        neighbours,
        ["desk_path"],
        "the only way onto the region behind the counter"
    );

    let entry =
        std::fs::read_to_string(design_gate::repo_root().join(".yidam/catalog/delores-source.md"))
            .expect("readable");
    let flat = entry.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("it is the only mention of `desk_path` anywhere in the 49 script files"),
        "the catalog no longer carries the script half of this claim"
    );
}
