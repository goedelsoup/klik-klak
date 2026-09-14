//! realizes: subsystem/path-solver
//!
//! **Find a route across a room's floor, or report which of two ways it failed.**
//!
//! The component's responsibility has been written down since genesis and the second half of it
//! is the part that was never obvious: *"Given a start point, a goal point and a walkbox set,
//! return a path or report that none exists. Reporting **that none exists** is half the job and
//! the half most often left out."*
//!
//! **There are two ways for none to exist and the corpus named one.** A click can land on floor
//! that no sequence of regions reaches — the failure
//! `.yidam/corpus/subsystem/path-solver.yml` describes — or it can land on no floor at all, which
//! is what most of a room is. Both are here, and they are separate variants because they are
//! separate authored situations: the first is usually a shut door and the second is usually a
//! wall.
//!
//! **The solver reports how far it could get and does not decide whether to go there.** The node
//! states the open half: *"Doing nothing reads as a broken click; walking as far as possible reads
//! as intent the player did not have. Neither is wrong, and the vocabulary currently has no way
//! for an author to say which they meant."* Both failures carry `nearest`, which is the far end of
//! the second arm; taking it or ignoring it is one decision, made once, by whatever runs this.
//! It is not a decision an author makes, and `.yidam/decisions/drawn-boundary.yml` is why — an
//! author never writes a region, so there is nothing for them to write it on.
//!
//! **No state, by contract.** The node says *"None across frames, given the walkbox set. Where
//! walkboxes are state-dependent the solver must be re-run rather than cached."* So a hidden
//! region is not a flag here: it is a region the caller did not pass in. Nothing in this file
//! remembers a room.
//!
//! `.yidam/decisions/unbuilt-solver.yml`.

use std::collections::BTreeSet;

/// A point in a room's own pixel coordinates. Integer, because that is what a walkbox file holds.
pub type Point = (i32, i32);

/// One region of walkable floor: a polygon, and the name the drawing tool gave it if it gave one.
///
/// A walkbox carries nothing else. That is measured rather than assumed — across a shipped game's
/// 84 polygons, `polygon` and `name` are the only two keys that ever appear on one
/// (`.yidam/catalog/delores-source.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region<'a> {
    /// empty when the tool gave it none, which is the common case — 70 of 84
    pub name: &'a str,
    /// the ring, in order, without a repeated closing vertex
    pub polygon: &'a [Point],
}

/// What the solver answers, and the two failures are not one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    /// Malformed polygon input, named by its position in the supplied floor set.
    InvalidFloor { region: usize },
    /// Waypoints from the start to the goal, start first and goal last.
    ///
    /// Interior points turn at polygon vertices; every segment stays on connected floor.
    Walk(Vec<Point>),
    /// The goal is on the floor and no sequence of regions reaches it.
    ///
    /// The common author's spelling of this is a shut door: a shipped game hides a four-vertex
    /// quad and the room falls into two pieces.
    Unreachable {
        /// the closest the actor can get, which is the other arm and not a recommendation
        nearest: Point,
    },
    /// The goal is not on any region, which is most of a room.
    OffFloor {
        /// the closest reachable point to it, on the same terms
        nearest: Point,
    },
}

/// Every edge of a ring as an unordered vertex pair, so two regions that meet write the same edge
/// in opposite winding and it compares equal.
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

/// **Two regions are adjacent when they share an edge exactly.**
///
/// No tolerance, and none could be chosen from the data: across a shipped game's 84 polygons every
/// contact is an exact shared edge and none meet along a collinear segment without sharing one
/// (`.yidam/decisions/adjacency.yml`). The walkboxes tile because the editor snaps.
pub fn adjacent(a: &[Point], b: &[Point]) -> bool {
    !edges(a).is_disjoint(&edges(b))
}

/// **Whether a point is on a region**, boundary included.
///
/// Ray casting. A point exactly on a shared edge is on both regions, which is what a player
/// standing in a doorway is.
pub fn contains(polygon: &[Point], p: Point) -> bool {
    contains_xy(polygon, (p.0 as f64, p.1 as f64))
}

fn contains_xy(polygon: &[Point], (x, y): (f64, f64)) -> bool {
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        let cross = (b.0 as f64 - a.0 as f64) * (y - a.1 as f64)
            - (b.1 as f64 - a.1 as f64) * (x - a.0 as f64);
        if cross.abs() < 1e-8
            && x >= a.0.min(b.0) as f64
            && x <= a.0.max(b.0) as f64
            && y >= a.1.min(b.1) as f64
            && y <= a.1.max(b.1) as f64
        {
            return true;
        }
    }
    let mut inside = false;
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        let (x1, y1) = (a.0 as f64, a.1 as f64);
        let (x2, y2) = (b.0 as f64, b.1 as f64);
        if (y1 > y) != (y2 > y) && x < x1 + (y - y1) * (x2 - x1) / (y2 - y1) {
            inside = !inside;
        }
    }
    inside
}

/// How far a point is from a segment, and the point on it that is closest.
fn distance_to_segment(a: Point, b: Point, p: Point) -> (f64, Point) {
    let (x1, y1) = (a.0 as f64, a.1 as f64);
    let (x2, y2) = (b.0 as f64, b.1 as f64);
    let (px, py) = (p.0 as f64, p.1 as f64);
    let (dx, dy) = (x2 - x1, y2 - y1);
    let t = if dx == 0.0 && dy == 0.0 {
        0.0
    } else {
        (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0)
    };
    let (qx, qy) = (x1 + t * dx, y1 + t * dy);
    (
        ((px - qx).powi(2) + (py - qy).powi(2)).sqrt(),
        (qx.round() as i32, qy.round() as i32),
    )
}

/// Which region a point is on, the first one if it is on a shared edge.
fn locate(floor: &[Region], p: Point) -> Option<usize> {
    floor.iter().position(|r| contains(r.polygon, p))
}

/// The regions reachable from one, by shared edges.
fn reachable(floor: &[Region], from: usize) -> Vec<usize> {
    let mut seen = vec![false; floor.len()];
    let (mut stack, mut out) = (vec![from], Vec::new());
    while let Some(v) = stack.pop() {
        if std::mem::replace(&mut seen[v], true) {
            continue;
        }
        out.push(v);
        for (w, r) in floor.iter().enumerate() {
            if !seen[w] && adjacent(floor[v].polygon, r.polygon) {
                stack.push(w);
            }
        }
    }
    out.sort_unstable();
    out
}

/// The closest point to `goal` on any of `within`, and how far it is.
fn closest(floor: &[Region], within: &[usize], goal: Point) -> (f64, Point) {
    let mut best = (f64::INFINITY, goal);
    for &i in within {
        let p = floor[i].polygon;
        if contains(p, goal) {
            return (0.0, goal);
        }
        for k in 0..p.len() {
            let near = distance_to_segment(p[k], p[(k + 1) % p.len()], goal).1;
            let mut candidates = vec![p[k]];
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let (Some(x), Some(y)) = (near.0.checked_add(dx), near.1.checked_add(dy)) {
                        candidates.push((x, y));
                    }
                }
            }
            for point in candidates {
                let d = distance(point, goal);
                if d < best.0 && contains(p, point) {
                    best = (d, point);
                }
            }
        }
    }
    best
}

/// Split a candidate segment at every boundary crossing. Each open interval must lie
/// on the connected floor. Checking only its endpoints misses concave notches and holes.
fn segment_inside(floor: &[Region], a: Point, b: Point) -> bool {
    let (ax, ay) = (a.0 as f64, a.1 as f64);
    let (dx, dy) = (b.0 as f64 - ax, b.1 as f64 - ay);
    let length2 = dx * dx + dy * dy;
    if length2 == 0.0 {
        return locate(floor, a).is_some();
    }
    let mut cuts = vec![0.0, 1.0];
    for region in floor {
        let p = region.polygon;
        for i in 0..p.len() {
            let (c, d) = (p[i], p[(i + 1) % p.len()]);
            let (cx, cy) = (c.0 as f64 - ax, c.1 as f64 - ay);
            let (ex, ey) = (d.0 as f64 - c.0 as f64, d.1 as f64 - c.1 as f64);
            let det = dx * ey - dy * ex;
            if det.abs() > 1e-8 {
                let t = (cx * ey - cy * ex) / det;
                let u = (cx * dy - cy * dx) / det;
                if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                    cuts.push(t);
                }
            } else if (cx * dy - cy * dx).abs() < 1e-8 {
                for v in [c, d] {
                    let t = ((v.0 as f64 - ax) * dx + (v.1 as f64 - ay) * dy) / length2;
                    if (0.0..=1.0).contains(&t) {
                        cuts.push(t);
                    }
                }
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    cuts.windows(2).all(|pair| {
        let t = (pair[0] + pair[1]) / 2.0;
        floor
            .iter()
            .any(|r| contains_xy(r.polygon, (ax + dx * t, ay + dy * t)))
    })
}

fn distance(a: Point, b: Point) -> f64 {
    (a.0 as f64 - b.0 as f64).hypot(a.1 as f64 - b.1 as f64)
}

/// Visibility graph over polygon vertices, followed by shortest-path search. Concave
/// regions need internal turns even when no region boundary is crossed.
fn path(floor: &[Region], from: Point, to: Point) -> Option<Vec<Point>> {
    let mut nodes = vec![from, to];
    for r in floor {
        for p in r.polygon {
            if !nodes.contains(p) {
                nodes.push(*p);
            }
        }
    }
    let mut distances = vec![f64::INFINITY; nodes.len()];
    let mut came = vec![None; nodes.len()];
    let mut visited = vec![false; nodes.len()];
    distances[0] = 0.0;
    loop {
        let u = (0..nodes.len())
            .filter(|i| !visited[*i])
            .min_by(|a, b| distances[*a].total_cmp(&distances[*b]))?;
        if !distances[u].is_finite() {
            return None;
        }
        if u == 1 {
            let mut route = vec![to];
            let mut at = u;
            while let Some(previous) = came[at] {
                route.push(nodes[previous]);
                at = previous;
            }
            route.reverse();
            route.dedup();
            return Some(route);
        }
        visited[u] = true;
        for v in 0..nodes.len() {
            let cost = distances[u] + distance(nodes[u], nodes[v]);
            if !visited[v] && cost < distances[v] && segment_inside(floor, nodes[u], nodes[v]) {
                distances[v] = cost;
                came[v] = Some(u);
            }
        }
    }
}

/// Solve on nondegenerate simple polygon rings with exact shared edges. The caller omits
/// disabled regions. An off-floor actor starts at a snapped, valid floor point in the route.
pub fn solve(floor: &[Region], from: Point, to: Point) -> Route {
    if floor.is_empty() {
        return Route::OffFloor { nearest: from };
    }
    if let Some(region) = floor.iter().position(|r| !valid_ring(r.polygon)) {
        return Route::InvalidFloor { region };
    }
    let all: Vec<_> = (0..floor.len()).collect();
    let from = if locate(floor, from).is_some() {
        from
    } else {
        closest(floor, &all, from).1
    };
    let start = locate(floor, from).expect("closest returns floor");
    let within = reachable(floor, start);
    if locate(floor, to).is_none() {
        return Route::OffFloor {
            nearest: closest(floor, &within, to).1,
        };
    }
    let connected: Vec<_> = within.iter().map(|i| floor[*i]).collect();
    if locate(&connected, to).is_none() {
        return Route::Unreachable {
            nearest: closest(floor, &within, to).1,
        };
    }
    match path(&connected, from, to) {
        Some(points) => Route::Walk(points),
        None => Route::Unreachable {
            nearest: closest(floor, &within, to).1,
        },
    }
}

fn valid_ring(p: &[Point]) -> bool {
    if p.len() < 3 || p.iter().collect::<BTreeSet<_>>().len() != p.len() {
        return false;
    }
    let cross = |a: Point, b: Point, c: Point| {
        (b.0 as i128 - a.0 as i128) * (c.1 as i128 - a.1 as i128)
            - (b.1 as i128 - a.1 as i128) * (c.0 as i128 - a.0 as i128)
    };
    let area: i128 = (0..p.len())
        .map(|i| cross((0, 0), p[i], p[(i + 1) % p.len()]))
        .sum();
    if area == 0 {
        return false;
    }
    for i in 0..p.len() {
        for j in i + 1..p.len() {
            if j == i + 1 || (i == 0 && j + 1 == p.len()) {
                continue;
            }
            let (a, b, c, d) = (p[i], p[(i + 1) % p.len()], p[j], p[(j + 1) % p.len()]);
            let intersects = cross(a, b, c).signum() * cross(a, b, d).signum() <= 0
                && cross(c, d, a).signum() * cross(c, d, b).signum() <= 0
                && a.0.min(b.0) <= c.0.max(d.0)
                && c.0.min(d.0) <= a.0.max(b.0)
                && a.1.min(b.1) <= c.1.max(d.1)
                && c.1.min(d.1) <= a.1.max(b.1);
            if intersects {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concave_floor_requires_a_turn_and_every_segment_stays_inside() {
        let polygon = [(0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10)];
        let floor = [Region {
            name: "L",
            polygon: &polygon,
        }];
        assert!(!segment_inside(&floor, (8, 1), (1, 8)));
        let Route::Walk(points) = solve(&floor, (8, 1), (1, 8)) else {
            panic!("route")
        };
        assert!(points.len() >= 3);
        assert!(points
            .windows(2)
            .all(|s| segment_inside(&floor, s[0], s[1])));
        let Route::Walk(snapped) = solve(&floor, (12, 1), (1, 8)) else {
            panic!("snap")
        };
        assert_ne!(snapped[0], (12, 1));
        assert!(contains(&polygon, snapped[0]));
        assert!(snapped
            .windows(2)
            .all(|s| segment_inside(&floor, s[0], s[1])));
    }

    #[test]
    fn a_hole_between_connected_regions_cannot_be_crossed() {
        let a = [(0, 0), (3, 0), (3, 8), (0, 8)];
        let b = [
            (3, 0),
            (7, 0),
            (7, 8),
            (3, 8),
            (3, 5),
            (5, 5),
            (5, 3),
            (3, 3),
        ];
        let floor = [
            Region {
                name: "a",
                polygon: &a,
            },
            Region {
                name: "b",
                polygon: &b,
            },
        ];
        // The rings only partially meet; exact shared edges remain the adjacency contract.
        assert!(!segment_inside(&floor, (2, 4), (6, 4)));
    }

    #[test]
    fn malformed_rings_are_distinct_from_unreachable_and_off_floor() {
        let p = [(0, 0), (8, 8), (0, 8), (8, 0)];
        assert!(matches!(
            solve(
                &[Region {
                    name: "crossed",
                    polygon: &p
                }],
                (0, 0),
                (1, 1)
            ),
            Route::InvalidFloor { region: 0 }
        ));
        let a = [(0, 0), (2, 0), (2, 2), (0, 2)];
        let b = [(4, 0), (6, 0), (6, 2), (4, 2)];
        let floor = [
            Region {
                name: "a",
                polygon: &a,
            },
            Region {
                name: "b",
                polygon: &b,
            },
        ];
        assert!(matches!(
            solve(&floor, (1, 1), (5, 1)),
            Route::Unreachable { .. }
        ));
        assert!(matches!(
            solve(&floor, (1, 1), (3, 1)),
            Route::OffFloor { .. }
        ));
    }
}
