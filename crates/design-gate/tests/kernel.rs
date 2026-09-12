//! **A kernel function table is the complete list of what an engine offered a script, so a thing
//! absent from it is a thing the engine did not do.**
//!
//! `prior-art/sierra-sci` has asked since [input-side](../../../.yidam/decisions/input-side.yml)
//! whether SCI1's icon bar was a game script or an engine feature. The marker named the wrong
//! document, was corrected by [class-library](../../../.yidam/decisions/class-library.yml), stood
//! wrong in the node for two phases, and was repaired by
//! [retraction-reach](../../../.yidam/decisions/retraction-reach.yml) to ask for an SCI1-era
//! artifact.
//!
//! **What was retrieved instead is better than an artifact**: SCI0's kernel table, all 114 calls,
//! from the community's reverse engineering of the interpreter
//! ([sci-specifications](../../../.yidam/catalog/sci-specifications.md)). A shipped title's scripts
//! would show what *one* game built; a kernel table shows what *any* game could ask the engine for.
//!
//! The figures live in the catalog, where quotations live. What this file does is refuse to let
//! anything else restate them differently — `.yidam/decisions/collected-verdict.yml`'s remedy,
//! applied for the third phase running.

use design_gate::repo_root;
use std::path::Path;

const CATALOG: &str = ".yidam/catalog/sci-specifications.md";
const NODE: &str = ".yidam/corpus/prior-art/sierra-sci.yml";

fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

fn flat(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every kernel call the entry names, and what each is for.
const NAMED: [(&str, &str); 8] = [
    ("AddMenu", "menu"),
    ("DrawMenuBar", "menu"),
    ("MenuSelect", "menu"),
    ("GetMenu", "menu"),
    ("SetMenu", "menu"),
    ("DrawControl", "control"),
    ("HiliteControl", "control"),
    ("EditControl", "control"),
];

/// **The entry carries the census every claim about this rests on.**
///
/// Five of the 114 are the menu bar and three are controls, and a control has six types of which
/// the fourth is an icon. The entry states each of those, and the point of asserting it here is
/// that the corpus and the records are about to quote it.
#[test]
fn the_catalog_carries_the_kernel_census() {
    let root = repo_root();
    let src = flat(&read(&root, CATALOG));

    assert!(
        src.contains("114 calls, `0x00` to `0x71`"),
        "the entry no longer states the size of the table"
    );
    for (name, _) in NAMED {
        assert!(src.contains(&format!("`{name}`")), "the entry drops {name}");
    }
    assert_eq!(
        NAMED.iter().filter(|(_, k)| *k == "menu").count(),
        5,
        "menu calls"
    );
    assert_eq!(
        NAMED.iter().filter(|(_, k)| *k == "control").count(),
        3,
        "control calls"
    );

    // The six control types, and the one that matters.
    assert!(
        src.contains("1 Button, 2 Text, 3 Edit, **4 Icon**, 5 unused, 6 Selector"),
        "the entry no longer lists the control types"
    );
    // And the five event types, which is the other half of the argument: the engine reports a
    // click and never a choice.
    assert!(
        src.contains("five event types and no more"),
        "the entry no longer counts the event vocabulary"
    );

    // The markup `AddMenu` takes, which is the finding that was not being looked for.
    for ch in ["right-justify what follows", "draw a separation line"] {
        assert!(src.contains(ch), "the entry drops the menu markup: {ch}");
    }
}

/// **CENSUS. The question is still open and the artifact that answers it is now in the house.**
///
/// The marker reads *"What would settle it is an SCI1-era artifact: a system script from 1990 or
/// later, or a shipped title's own `Game` and `User` subclasses. Neither is retrieved."* Both
/// halves were true when they were written and the second is about to stop being the point: a
/// kernel table is neither of the two things the marker names, and it settles the question better
/// than either would.
///
/// **A marker that names its resolver can be wrong about the artifact and right about the
/// question**, which is a third way for the convention to be useful and was not predicted. The two
/// already recorded are being wrong about the date ([class-library]) and being wrong about whether
/// the thing is already held ([retraction-reach]).
///
/// [class-library]: ../../../.yidam/decisions/class-library.yml
/// [retraction-reach]: ../../../.yidam/decisions/retraction-reach.yml
#[test]
fn the_marker_is_open_and_the_answer_is_now_retrievable() {
    let root = repo_root();
    let node = flat(&read(&root, NODE));

    assert!(node.contains("`[open]`"), "the question has been closed");
    assert!(
        node.contains("Neither is retrieved"),
        "the marker no longer says the artifact it names is not held"
    );
    // And the thing that does answer it is neither of the two it names. The node says "kernel
    // calls" once already — inside Stephenson's own sentence about the language not being enough —
    // so the census is that it does not yet count them.
    assert!(
        !node.contains("114"),
        "the node already argues from the kernel table, which is what the revision does"
    );
}
