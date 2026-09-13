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
    /// Waypoints from the start to the goal, start first and goal last.
    ///
    /// Interior points are the midpoints of the shared edges crossed. Smoothing them into a
    /// straight line where one exists is a question about how walking *looks*, which is on the
    /// far side of `.yidam/decisions/drawn-boundary.yml` and is not asked here.
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

/// The edge two adjacent regions share, as the doorway to walk through.
fn portal(a: &[Point], b: &[Point]) -> Option<(Point, Point)> {
    edges(a).intersection(&edges(b)).next().copied()
}

/// **Whether a point is on a region**, boundary included.
///
/// Ray casting. A point exactly on a shared edge is on both regions, which is what a player
/// standing in a doorway is.
pub fn contains(polygon: &[Point], p: Point) -> bool {
    let (x, y) = (p.0 as f64, p.1 as f64);
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        if distance_to_segment(a, b, p).0 == 0.0 {
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
            let d = distance_to_segment(p[k], p[(k + 1) % p.len()], goal);
            if d.0 < best.0 {
                best = d;
            }
        }
    }
    best
}

/// The sequence of regions from one to another, or nothing.
fn route(floor: &[Region], from: usize, to: usize) -> Option<Vec<usize>> {
    let mut came: Vec<Option<usize>> = vec![None; floor.len()];
    let mut seen = vec![false; floor.len()];
    let mut queue = std::collections::VecDeque::from([from]);
    seen[from] = true;
    while let Some(v) = queue.pop_front() {
        if v == to {
            let mut path = vec![to];
            while let Some(p) = came[*path.last().expect("non-empty")] {
                path.push(p);
            }
            path.reverse();
            return Some(path);
        }
        for (w, r) in floor.iter().enumerate() {
            if !seen[w] && adjacent(floor[v].polygon, r.polygon) {
                seen[w] = true;
                came[w] = Some(v);
                queue.push_back(w);
            }
        }
    }
    None
}

/// **Solve one click.**
///
/// `from` is where the actor is. An actor is always on the floor — whatever placed it is
/// responsible for that — and a `from` that is not gets snapped to the nearest region rather than
/// refused, because a solver that can fail on the actor's own position gives its caller a failure
/// it has nothing to do with.
///
/// The walkbox set is the room *as it is now*. A hidden region is one the caller did not pass.
pub fn solve(floor: &[Region], from: Point, to: Point) -> Route {
    if floor.is_empty() {
        return Route::OffFloor { nearest: from };
    }
    let start = locate(floor, from).unwrap_or_else(|| {
        let all: Vec<usize> = (0..floor.len()).collect();
        let near = closest(floor, &all, from).1;
        locate(floor, near).unwrap_or(0)
    });
    let within = reachable(floor, start);

    let Some(goal) = locate(floor, to) else {
        return Route::OffFloor {
            nearest: closest(floor, &within, to).1,
        };
    };
    let Some(regions) = route(floor, start, goal) else {
        return Route::Unreachable {
            nearest: closest(floor, &within, to).1,
        };
    };

    let mut walk = vec![from];
    for pair in regions.windows(2) {
        let (a, b) = (floor[pair[0]].polygon, floor[pair[1]].polygon);
        if let Some((p, q)) = portal(a, b) {
            let mid = ((p.0 + q.0) / 2, (p.1 + q.1) / 2);
            if *walk.last().expect("non-empty") != mid {
                walk.push(mid);
            }
        }
    }
    if *walk.last().expect("non-empty") != to {
        walk.push(to);
    }
    Route::Walk(walk)
}
