//! **The specimens, read from the directory rather than named.**
//!
//! Every test here that makes a claim about *the specimens* goes through this. The alternative —
//! an `include_str!` list of the games that existed when the file was written — is what twenty-five
//! files held, and `.yidam/decisions/unread-specimen.yml` is what that cost: a fifth game written,
//! eleven commits passing, and three quarters of the suite measuring four fifths of the material
//! without a single failure to say so.
//!
//! The point is not tidiness. **It makes the input a thing that can change.** A figure pinned over
//! a pinned list is two constants agreeing with each other, and a green test over a list nobody
//! updated is indistinguishable from a green test over the artifact. A figure pinned over the
//! directory fails the moment a sixth game lands, and a failure is the only signal this repository
//! has ever acted on.
//!
//! A test that is *about* a named game still names it — `effects.rs` reads `ferry` and `lighthouse`
//! because they are the two that write the effect it is about, and that stays a choice however many
//! specimens exist. What no test may do is name all of them, which is holding a copy of this
//! directory. `crates/design-gate/tests/enumerated.rs` is where that is checked.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// The repository root — two levels above any crate that uses this.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("a crate two levels below the root")
        .to_path_buf()
}

/// Every specimen on disk, by name, in path order.
pub fn sources() -> Vec<(String, String)> {
    let dir = root().join("design/specimen");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the specimen directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "game"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            let name = p
                .file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned();
            let src = std::fs::read_to_string(&p).expect("readable");
            (name, src)
        })
        .collect()
}

/// One specimen by name, for a test that is about that game rather than about the set.
pub fn source(name: &str) -> String {
    sources()
        .into_iter()
        .find(|(n, _)| n == name)
        .unwrap_or_else(|| panic!("no specimen named {name}"))
        .1
}

/// The same, read and admitted, with any complaint from either pass a hard failure.
pub fn games() -> Vec<(String, game_format::Game)> {
    sources()
        .into_iter()
        .map(|(name, src)| {
            let (mut game, mut problems) = game_format::parse(&src, &name);
            problems.extend(game_format::resolve(&mut game));
            let msgs: Vec<String> = problems.into_iter().map(|p| p.to_string()).collect();
            assert!(msgs.is_empty(), "{name} does not read clean: {msgs:?}");
            (name, game)
        })
        .collect()
}
