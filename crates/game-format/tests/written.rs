//! **The crate reads, admits and writes, and the third is what checks the first two against each
//! other.**
//!
//! It was *model, reader, and the admission rule*, and nothing could write a `.game` file — so
//! nothing had ever asked whether the tree holds what the file says.
//!
//! The reader accounts for every token on every line now —
//! `.yidam/decisions/parser-silence.yml` — which answers *did anything go unread*. It does not
//! answer *did anything go unkept*, and those are different questions with different failures: a
//! token the reader drops is a line nobody meant, and a token the tree drops is a line somebody
//! wrote and no consumer can find.
//!
//! **The tree has lost something three times in seven phases, and every time a consumer found it
//! from the outside.** Labels and response text went missing until a menu and a presenter wanted
//! them; `spoken_with` until a menu asked who a conversation was held with. None was found by
//! looking at the reader.
//!
//! Both are checked now, by being executed — `crates/game-format/tests/round_trip.rs`. What this
//! file keeps is the census that made the question worth asking, and the two claims in the
//! wording they were made in:
//!
//! * `exit-archetype` says an `exit` *"expands at parse time into the binding an author would
//!   otherwise have written"* — and nothing has ever written that binding out to compare.
//! * `authored-text` says *"`say` and `otherwise` are both a line with no predicate, and what
//!   makes `otherwise` a fallback is that it is written last"* — and the file has three
//!   spellings of that one thing.

use game_format::{parse, resolve, Game};

const ALL: [(&str, &str); 4] = [
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

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/game-format is two levels below the root")
        .to_path_buf()
}

fn load(source: &str, name: &str) -> Game {
    let (mut game, problems) = parse(source, name);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    let problems = resolve(&mut game);
    assert!(problems.is_empty(), "{name}: {problems:?}");
    game
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **The crate reads, admits, writes — and says what it reads.**
///
/// Its description said *"model, reader, and the admission rule"* and the fourth module is the
/// one that lets either direction be checked against the other. The fifth is the specification:
/// `forms.rs` holds the written form of every construct of the declaration half, which lived in
/// sixteen match arms and in a document that disagreed with them.
/// `.yidam/decisions/line-format.yml`.
#[test]
fn the_crate_can_write_what_it_reads() {
    let root = root();
    let src = root.join("crates/game-format/src");
    let modules: Vec<String> = std::fs::read_dir(&src)
        .expect("the source directory")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            (p.extension()? == "rs").then(|| p.file_stem()?.to_str().map(str::to_string))?
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(
        modules,
        ["forms", "lib", "parse", "print", "resolve", "world"],
        "the modules this crate has"
    );

    let writer = std::fs::read_to_string(src.join("print.rs")).expect("readable");
    assert!(
        flat(&writer).contains("It is not a formatter."),
        "and the writer says what it is not, because a third of the file is not in the tree"
    );
    let manifest =
        std::fs::read_to_string(root.join("crates/game-format/Cargo.toml")).expect("readable");
    assert!(
        manifest.contains("the written form of every construct, a model, a reader, a writer"),
        "and the crate says what it is"
    );
}

/// **The tree has lost something three times, and the reader is not where any of it was found.**
///
/// Both fields carry their own account of how they went missing, which is this repository's
/// convention and is also the evidence: a field that has to explain why it exists is a field
/// something noticed the absence of, from outside, after the fact.
#[test]
fn the_fields_that_went_missing_say_how_they_were_found() {
    let lib = std::fs::read_to_string(root().join("crates/game-format/src/lib.rs")).expect("it");
    let lib = flat(&lib);
    assert!(
        lib.contains(
            "The `with` was written, unchecked and unread for the whole life of this format."
        ),
        "`spoken_with`, found by rendering a menu"
    );
    assert!(
        lib.contains("`.yidam/decisions/authored-text.yml`"),
        "`labels`, found by wanting to say what a thing is called"
    );
    let parse =
        std::fs::read_to_string(root().join("crates/game-format/src/parse.rs")).expect("it");
    assert!(
        flat(&parse)
            .contains("This arm was `{}` for six phases and dropped every word a game says."),
        "and the response text, found by wanting to say anything at all"
    );
}

/// **Two claims about the tree, in the wording they were made in.**
///
/// `round_trip.rs` executes both. This holds the sentences, so that a record edited into
/// agreement with a passing test would be caught rather than quietly become the claim.
#[test]
fn two_standing_claims_need_a_file_to_be_checked() {
    let root = root();
    let archetype =
        std::fs::read_to_string(root.join(".yidam/decisions/exit-archetype.yml")).expect("it");
    assert!(
        flat(&archetype).contains(
            "It **expands at parse time** into the binding an author would otherwise have written"
        ),
        "the expansion claim"
    );
    let authored =
        std::fs::read_to_string(root.join(".yidam/decisions/authored-text.yml")).expect("it");
    assert!(
        flat(&authored).contains("`say` and `otherwise` are both a line with no predicate, and what makes `otherwise` a fallback is that it is written last"),
        "the collapse claim"
    );

    // and nothing uses `exit`, so the expansion has never even been read in anger
    for (name, src) in ALL {
        assert!(
            !src.lines().any(|l| l.trim_start().starts_with("exit ")),
            "{name} writes an `exit` now"
        );
    }
}

/// **Seventy-three lines are written three ways and the tree holds one thing.**
///
/// `say`, `otherwise`, and a bare quoted line: the file distinguishes them and the parse tree
/// does not — every one is a `Response` with `when: None`. It is a collapse and not a loss, and
/// `round_trip.rs` is where that is shown: the writer emits `say` for all three and no tree and
/// no figure moves. What tells them apart is where they sit in the list, which is order.
#[test]
fn one_construct_is_written_three_ways() {
    let (mut say, mut otherwise, mut bare) = (0, 0, 0);
    for (_, src) in ALL {
        for line in src.lines() {
            let t = line.trim_start();
            if t.starts_with("say ") {
                say += 1;
            } else if t.starts_with("otherwise") {
                otherwise += 1;
            } else if t.starts_with('"') {
                bare += 1;
            }
        }
    }
    assert_eq!((say, otherwise, bare), (28, 11, 34), "the three spellings");

    // and every one of them is the same thing in the tree
    let mut unguarded = 0;
    for (name, src) in ALL {
        let game = load(src, name);
        unguarded += game
            .bindings
            .iter()
            .flat_map(|b| b.says.iter())
            .filter(|r| r.when.is_none())
            .count();
    }
    assert_eq!(
        unguarded,
        say + otherwise + bare,
        "unguarded response clauses, however they were spelled"
    );
}

/// **A third of the file is not in the tree, and the round trip measures it: 662 lines in, 441
/// out.**
///
/// 662 lines across the four specimens, of which 133 are comments, 3 are `??` annotations and
/// 143 are blank. None of it survives parsing and none of it should — but nothing has ever said
/// how much of a `.game` file is for the reader of the file rather than for the game.
#[test]
fn a_third_of_the_file_is_for_whoever_reads_the_file() {
    let (mut total, mut hash, mut annotation, mut blank) = (0, 0, 0, 0);
    for (_, src) in ALL {
        for line in src.lines() {
            total += 1;
            let t = line.trim();
            if t.is_empty() {
                blank += 1;
            } else if t.starts_with('#') {
                hash += 1;
            } else if t.contains("??") {
                annotation += 1;
            }
        }
    }
    assert_eq!((total, hash, annotation, blank), (662, 133, 3, 143));
}
