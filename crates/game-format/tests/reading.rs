//! **Does the file that carries the writing read like writing?** — the measurement
//! [`data-format/interaction-table`] has wanted since genesis, taken over the material that now
//! exists.
//!
//! The question is `authored_by`'s: *"This is the file that carries the writing, and it should read
//! like writing rather than like data — which is a constraint on the syntax that pulls against the
//! keying."* `.yidam/decisions/resolver-kind.yml` classified it as the one question here anybody
//! could act on, and said what would close it: a count over 904 authored lines and 120 bindings.
//!
//! **Three measurements, three different answers, and the three together are the answer.**
//!
//! **1. On the line, decisively yes.** A response line is 51.9 characters of sentence behind 9.2 of
//! prefix — **84% of what you read on that line is the sentence**, median prefix six characters
//! (`say` and two spaces). The same 154 responses written as the flat table this node describes,
//! each row restating room, target, verb and guard, would carry a 26-character prefix: **66%**. And per
//! binding the ratio of writing to frame is **2.02**, against **0.56** in Delores — a shipped,
//! hand-authored adventure game by the author whose formats this corpus keeps reading
//! (`.yidam/catalog/delores-source.md`). Excluding that game's 62 prose-free availability gates it
//! is 0.62. Either way this design carries **three times the writing per character of frame that a
//! shipped game does**.
//!
//! **2. Down the page, no — and it must not.** The median run of adjacent sentences is **one**: 154
//! sentences in 126 runs, the longest four. Nothing here reads like a page of prose. That is not a
//! defect and a format that fixed it would be the wrong format: an adventure game's writing is not
//! paragraphs, it is 154 separate answers to 154 separate clicks. The thing with runs is the
//! *other* node, [`data-format/dialogue-script`], and this corpus keeps them apart on purpose.
//! **"Reads like writing" cannot mean "looks like a screenplay"**, which is the half of the
//! question that had to be answered before the other half could be measured.
//!
//! **3. Above the line, the keying is real, and it is distance rather than characters.** The
//! nesting does not delete the key, it moves it: **2,628 characters** of `room`, `hotspot`, `actor`
//! and `<verb>:` headers — 13% of the file — written once and read from below. What that costs is reach — the verb is
//! a median **2** lines above its sentence, the target **4**, the room **22**, and the furthest
//! room reach in the corpus is **94**.
//!
//! **So the pull the node named is real and now has a price.** A response line gives up 16% of
//! itself to keying, or it gives up 34% and reads as a flat table; this syntax takes the first and
//! pays a median twenty-two-line reach back to the room. `.yidam/decisions/reads-like-writing.yml`.
//!
//! [`data-format/interaction-table`]: ../../../.yidam/corpus/data-format/interaction-table.yml
//! [`data-format/dialogue-script`]: ../../../.yidam/corpus/data-format/dialogue-script.yml

mod common;

/// One authored sentence, and what a reader passes to reach it.
struct Line {
    /// characters before the opening quote, indentation stripped
    prefix: usize,
    /// the sentence with its quotes, which is what the author typed
    sentence: usize,
    /// lines above this one to the header naming the verb, the target, the room
    to_verb: usize,
    to_target: usize,
    to_room: usize,
    /// what a flat row would have to restate
    flat_key: usize,
}

/// The five spellings that carry a line the game says — `authored.rs`'s list, and the count below
/// is held against its figure so the two censuses cannot drift.
const SAYING: [&str; 4] = ["say", "otherwise", "when", "else"];

const VERBS: [&str; 7] = [
    "look-at", "pick-up", "walk-to", "talk-to", "use-with", "give-to", "on-enter",
];

/// Every response line in every specimen, read the way a person reads the file.
///
/// From the source rather than the parse tree, because the question is about reading it. The count
/// is cross-checked against `authored.rs`, which compares what was written against what the tree
/// kept — so this cannot quietly start counting something else.
fn lines_of(src: &str) -> (Vec<Line>, Vec<usize>, usize) {
    let raw: Vec<&str> = src.lines().collect();
    let (mut out, mut runs) = (Vec::new(), Vec::new());
    let (mut room, mut target, mut verb) = (String::new(), String::new(), String::new());
    let (mut room_at, mut target_at, mut verb_at) = (0usize, 0usize, 0usize);
    let (mut key_chars, mut run) = (0usize, 0usize);

    for (i, line) in raw.iter().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            if run > 0 {
                runs.push(run);
                run = 0;
            }
            continue;
        }
        let head = match line.find('"') {
            Some(o) => line[..o].trim(),
            None => t,
        };
        let word = head
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches(':');
        let named = |n: usize| t.split_whitespace().nth(n).unwrap_or("").to_string();

        // a header keys everything under it, and is read from below
        let header = match word {
            "room" => {
                room = named(1);
                target.clear();
                verb.clear();
                room_at = i;
                true
            }
            "hotspot" | "actor" | "conversation" => {
                target = named(1);
                verb.clear();
                target_at = i;
                true
            }
            "option" => {
                target = named(1);
                verb = "option".into();
                target_at = i;
                verb_at = i;
                true
            }
            v if head.ends_with(':') && VERBS.contains(&v) => {
                verb = v.to_string();
                verb_at = i;
                true
            }
            _ => false,
        };
        if header {
            // the header without its label: a hotspot's quoted description is writing, not keying
            key_chars += match line.find('"') {
                Some(o) => line[..o].trim_start().chars().count(),
                None => line.trim_start().chars().count(),
            };
            if run > 0 {
                runs.push(run);
                run = 0;
            }
            continue;
        }

        let (Some(open), Some(close)) = (line.find('"'), line.rfind('"')) else {
            if run > 0 {
                runs.push(run);
                run = 0;
            }
            continue;
        };
        if close <= open || !(head.is_empty() || SAYING.contains(&word)) {
            if run > 0 {
                runs.push(run);
                run = 0;
            }
            continue;
        }

        let guard = head
            .strip_prefix("else when ")
            .or_else(|| head.strip_prefix("when "))
            .unwrap_or("")
            .trim();
        out.push(Line {
            prefix: line[..open].trim_start().chars().count(),
            sentence: line[open..=close].chars().count(),
            to_verb: i - verb_at,
            to_target: i - target_at,
            to_room: i - room_at,
            // `<room>.<target> <verb> <guard>` — the key a row of the table this node describes
            // has to carry, plus the space before the sentence
            flat_key: format!("{room}.{target} {verb} {guard}")
                .trim()
                .chars()
                .count()
                + 1,
        });
        run += 1;
    }
    if run > 0 {
        runs.push(run);
    }
    (out, runs, key_chars)
}

fn corpus() -> (Vec<Line>, Vec<usize>, usize) {
    let (mut all, mut runs, mut key) = (Vec::new(), Vec::new(), 0);
    for (_, src) in common::sources() {
        let (l, r, k) = lines_of(&src);
        all.extend(l);
        runs.extend(r);
        key += k;
    }
    (all, runs, key)
}

fn mean(v: &[usize]) -> f64 {
    v.iter().sum::<usize>() as f64 / v.len() as f64
}

fn median(v: &[usize]) -> usize {
    let mut s = v.to_vec();
    s.sort_unstable();
    s[s.len() / 2]
}

/// **Five sixths of a response line is the sentence, and a flat table would make it two thirds.**
///
/// The direct form of the question. A reader arriving at an authored line reads 9.2 characters
/// before the writing starts and 51.9 of writing — median prefix six, which is `say` and two
/// spaces. The longest prefix in the corpus is 32 characters and it is a guard, not a key:
/// `when gates-shut and paddle-up`.
///
/// The comparison is the same 154 sentences keyed the way this node says an interaction table is
/// keyed — by verb and target, with the second object as an optional third key — one row each. The
/// row has to carry room, target, verb and guard, which is 26 characters, and the sentence's share
/// of its own line falls from 84% to 66%.
///
/// **That is the whole of the trade and it is smaller than the argument around it.** Eighteen
/// points of line share. What makes it worth having is the third test below: the 16% is a constant
/// per line, and the 34% would be paid 154 times.
#[test]
fn five_sixths_of_a_response_line_is_the_sentence() {
    let (all, _, _) = corpus();
    assert_eq!(
        all.len(),
        194,
        "response lines — `authored.rs` holds the same figure against the parse tree"
    );

    let prefix: Vec<usize> = all.iter().map(|l| l.prefix).collect();
    let sentence: Vec<usize> = all.iter().map(|l| l.sentence).collect();
    let flat: Vec<usize> = all.iter().map(|l| l.flat_key).collect();

    assert_eq!(
        (median(&prefix), *prefix.iter().max().expect("lines")),
        (6, 32),
        "median and longest prefix — the longest is a guard rather than a key"
    );
    assert_eq!(
        (median(&sentence), *sentence.iter().max().expect("lines")),
        (56, 145),
        "median and longest sentence"
    );

    let share = |p: &[usize]| {
        let (a, b): (usize, usize) = (sentence.iter().sum(), p.iter().sum());
        a * 100 / (a + b)
    };
    assert_eq!(
        (share(&prefix), share(&flat)),
        (84, 67),
        "the sentence's share of its own line, as written and as a flat table would key it"
    );
    assert_eq!(
        (mean(&prefix).round() as usize, mean(&flat).round() as usize),
        (10, 26),
        "mean prefix, as written and flat"
    );
}

/// **The corpus does not read like prose, because an adventure game is not prose.**
///
/// 154 sentences in **126 runs**, median run one, longest four. Nothing here is a page of writing
/// and nothing here should be: what an author writes in this format is 154 separate answers to 154
/// separate clicks, and a sentence is adjacent to another sentence only where a binding has a
/// refusal or an `otherwise` under it.
///
/// This is the half of the question that had to be settled before the other half could mean
/// anything. *"Reads like writing"* could be read as *looks like a screenplay*, and on that reading
/// the answer is no and the remedy would be the wrong format. The corpus already has the format
/// with runs — [`dialogue-script`], where an author writes a scene — and keeps it a separate node
/// from the interaction table on exactly this ground.
///
/// So the measurable content of *reads like writing* is what the first and third tests measure:
/// how much of the line the sentence gets, and how far away the thing it answers is.
///
/// [`dialogue-script`]: ../../../.yidam/corpus/data-format/dialogue-script.yml
#[test]
fn the_writing_comes_one_sentence_at_a_time_because_the_medium_does() {
    let (all, runs, _) = corpus();
    assert_eq!(
        (
            all.len(),
            runs.len(),
            median(&runs),
            *runs.iter().max().expect("runs")
        ),
        (194, 154, 1, 4),
        "sentences, runs of adjacent sentences, the median run and the longest"
    );
    assert!(
        mean(&runs) < 1.5,
        "runs have grown, which would mean an author started writing scenes in a file keyed by \
         verb and target — and `dialogue-script` is the node for that: {:.2}",
        mean(&runs)
    );
}

/// **The keying is moved rather than removed, and what it costs is reach.**
///
/// 2,628 characters of `room`, `hotspot`, `actor` and `<verb>:` header — 13% of everything an
/// author types — written once and read from below. A hotspot's quoted description is not counted:
/// that is writing. The flat form would write the same information on every row instead, and the
/// first test's 26-character prefix is what that looks like.
///
/// The price of not doing that is distance. A sentence's verb is a median **2** lines above it, its
/// target **4**, its room **22**, and the furthest a room header gets from a sentence in this
/// corpus is **94 lines** — `lock.game`'s chamber, which is three rooms in 242 lines.
///
/// **That is the answer to *pulls against the keying*: it pulls, and the pull is twenty-two lines.**
/// A reader who wants to know what room a sentence is in scrolls; a reader who wants to know what
/// verb it answers looks up two lines. The nesting spends the reach on the thing least often
/// needed, which is the ordering a person would choose and not one anybody wrote down.
#[test]
fn the_key_is_moved_above_the_line_and_costs_a_median_of_twenty_two_lines() {
    let (all, _, key) = corpus();
    assert_eq!(key, 3_085, "characters of header, written once each");

    let to = |f: &dyn Fn(&Line) -> usize| all.iter().map(f).collect::<Vec<_>>();
    let verb = to(&|l: &Line| l.to_verb);
    let target = to(&|l: &Line| l.to_target);
    let room = to(&|l: &Line| l.to_room);
    assert_eq!(
        (median(&verb), median(&target), median(&room)),
        (2, 4, 22),
        "lines above a sentence to the header naming its verb, its target and its room"
    );
    assert_eq!(
        *room.iter().max().expect("lines"),
        94,
        "the furthest a sentence gets from the room it is in"
    );

    // and the header cost is a sixth of the file, against a little over half for the writing
    let authored: usize = common::sources()
        .iter()
        .map(|(_, s)| {
            s.lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                .map(|l| match l.split("??").next() {
                    Some(x) if !l.contains('"') => x.trim_end().chars().count(),
                    _ => l.trim_end().chars().count(),
                })
                .sum::<usize>()
        })
        .sum();
    assert_eq!(
        key * 100 / authored,
        12,
        "the share of the file that is keying"
    );
}

/// **Per binding this format carries two characters of writing for every character of frame. A
/// shipped game carries 0.56.**
///
/// The independent measurement, and the one that is not about this repository's own taste.
/// `.yidam/catalog/delores-source.md` counts Delores the same way `duplication.rs` counts these
/// specimens — frame is every non-space character outside a string literal, prose is what is inside
/// one — over 287 authored binding bodies in a game shipped by the author whose SCUMM, SPUTM and
/// Dinky formats this corpus has read at every turn.
///
/// | | sites | frame/site | prose/site | prose : frame |
/// |---|---|---|---|---|
/// | Delores, all | 287 | 73.1 | 41.1 | **0.56** |
/// | Delores, less its 62 prose-free availability gates | 225 | 83.8 | 52.2 | **0.62** |
/// | this design | 120 | 32.0 | 64.5 | **2.02** |
/// | this design, `look-at` only | 56 | 17.5 | 71.0 | **4.07** |
///
/// **Three and a quarter times the writing per character of frame**, on the fairer of the two
/// comparisons. A shipped game's binding body is mostly machinery with a line in it; one of these
/// is mostly a line.
///
/// That is not a claim that this format is better — Delores's bodies do things this language cannot
/// express, which is the whole of `.yidam/decisions/declared-reach.yml` — but it is the claim the
/// question asked for. The file reads like writing in the one sense that can be counted: what is
/// on it is mostly writing.
#[test]
fn a_binding_here_is_mostly_writing_and_a_shipped_one_is_mostly_machinery() {
    // the catalog's row, so a re-measurement of Delores fails this rather than passing quietly
    let root = common::root();
    let catalog =
        std::fs::read_to_string(root.join(".yidam/catalog/delores-source.md")).expect("readable");
    assert!(
        catalog.contains("| all | 287 | 73.1 | 41.1 | 0.56 |"),
        "the catalog's frame-against-prose row has moved"
    );

    // and this design's, computed
    let (mut frame, mut prose, mut blocks) = (0usize, 0usize, 0usize);
    for (_, src) in common::sources() {
        let lines: Vec<&str> = src.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            let head = t
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches(':');
            let opens = (t.ends_with(':') && VERBS.contains(&head)) || t.starts_with("option ");
            if !opens || t.starts_with('#') {
                i += 1;
                continue;
            }
            blocks += 1;
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
            i = j;
        }
    }
    assert_eq!(
        (blocks, frame, prose),
        (144, 4_596, 9_949),
        "blocks, frame, prose"
    );
    assert_eq!(
        (
            frame * 10 / blocks,
            prose * 10 / blocks,
            prose * 100 / frame
        ),
        (319, 690, 216),
        "frame and prose per block in tenths, and prose as a percentage of frame"
    );
}
