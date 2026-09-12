//! **The declaration half of this format is specified by a table, and this is what holds the table
//! against the reader and against the document.**
//!
//! `.yidam/decisions/parser-silence.yml` closed on it: *"Sixteen branches each stating their own
//! arity is better than sixteen branches stating nothing, and it is not a specification… Whether
//! the declaration half should be described the way the expression half is — a written grammar,
//! from which the reader is derived rather than compared — is a real question and is not answered
//! here."*
//!
//! **Described, yes; described that way, no.** A declaration does not nest, so BNF for it is twenty
//! productions of one alternative each. [`game_format::FORMS`] is those twenty rows, the reader
//! reads its error text out of them, and [`design/bounded-expression-language.md`] quotes them.
//! `.yidam/decisions/line-format.yml`.
//!
//! What this file pinned before the table existed, and now holds as rules:
//!
//! - The document's Layer 0 block stated three site forms and the reader refused all three.
//!   `look-at door` is *"`door` is not part of `look-at`"* and `on-enter hall` is *"`hall` is not
//!   part of `on-enter`"*, because a binding's target comes from the `hotspot` or `actor` line
//!   above it and its room from the `room` section. It was a description of the **model** in the
//!   notation of a **syntax**, and it is labelled as the model now.
//! - Five forms stated a quoted label the reader has never required. The forms were wrong, not the
//!   reader: `exit` expands into a hotspot with no label and `print` writes that back out bare.
//! - A quoted string was an argument slot six constructs dropped in silence.
//!
//! [`design/bounded-expression-language.md`]: ../../../design/bounded-expression-language.md

use game_format::{form, parse, resolve, Quoted, FORMS};

/// Does the reader object to this source, at parse or at admission?
fn complains(src: &str) -> bool {
    let (mut game, problems) = parse(src, "forms");
    !problems.is_empty() || !resolve(&mut game).is_empty()
}

/// A whole small game with `{X}` standing where a line under test goes.
///
/// It declares more than it uses so that a substituted line has something to name, and it reaches
/// its ending so that nothing in `resolve` complains about the frame rather than about the line.
const GAME: &str = "\
room hall \"The Hall\"
actor ann \"Ann\"
item key \"a key\"
flag done \"it is done\"

room hall
  start
{X}
  hotspot door \"The Door\"
    look-at:
      do    set done
      ending
";

fn with(line: &str) -> String {
    GAME.replace("{X}", line)
}

/// The reader's own source, read as text because a literal form string left in it would be a
/// second copy of the specification.
const READER: &str = include_str!("../src/parse.rs");
const DOCUMENT: &str = include_str!("../../../design/bounded-expression-language.md");

/// **The table is the only copy, and the document quotes all of it.**
///
/// Three things had to agree and none of them could: forms scattered across sixteen match arms, a
/// document writing nine productions in a different notation, and a reader nobody had compared to
/// either. Now there is one list, the reader reads its messages out of it, and the document carries
/// every row verbatim.
///
/// The check against `parse.rs` is the negative half: a form written as a literal there would be a
/// second copy, and a second copy is what the whole phase was about.
#[test]
fn the_table_is_the_only_copy_and_the_document_quotes_all_of_it() {
    for row in FORMS {
        assert!(
            DOCUMENT.contains(row.written),
            "the document does not carry the form {:?}",
            row.written
        );
        // A one-word form *is* the keyword the reader matches on, so there is nothing there to
        // duplicate — `start` appears in `parse.rs` because the arm has to say `"start"`. The
        // check is for the forms with operands in them, which are the ones that drifted.
        if row.written != row.keyword {
            assert!(
                !READER.contains(row.written),
                "the reader writes {:?} as a literal, which is a second copy of the specification",
                row.written
            );
        }
    }

    // Every keyword the reader dispatches on has a row, so `fm`'s fallback is unreachable.
    for row in FORMS {
        assert!(
            READER.contains(&format!("\"{}\"", row.keyword)),
            "the table carries {:?} and the reader never mentions it",
            row.keyword
        );
    }

    // Two keywords have two rows apiece, and the quoted string is what tells them apart.
    let selects: Vec<&str> = FORMS
        .iter()
        .filter(|r| r.quoted == Quoted::Selects)
        .map(|r| r.keyword)
        .collect();
    assert_eq!(
        selects,
        ["room", "actor"],
        "keywords whose quoted string selects the form rather than filling a slot"
    );
    assert_eq!(form("room", true), Some("room <name> \"<label>\""));
    assert_eq!(form("room", false), Some("room <name>"));
    assert_eq!(
        form("when", true),
        None,
        "the binding body is not a line format"
    );
}

/// **The document's Layer 0 grammar states three alternatives and the reader refuses all three.**
///
/// This is the census and it pins the defect. `verb target`, `verb target object` and
/// `'on-enter' room` are all refused, because every operand in them comes from an enclosing line.
/// Written the way the reader wants them, the same three bindings parse — so nothing is missing
/// from the language and the sentence describing it is about a different artifact.
///
/// `decl := 'exit' name 'to' room` is not in the block and is the one production the document
/// states as a line an author types. It is also the only one added by a phase that built the
/// construct rather than argued for it.
#[test]
fn the_document_states_three_site_forms_and_the_reader_takes_one() {
    let as_written = [
        ("verb target", "    look-at door:"),
        ("verb target object", "    use-with door key:"),
        ("'on-enter' room", "  on-enter hall:"),
    ];
    for (production, line) in as_written {
        assert!(
            complains(&with(line)),
            "the reader now accepts {production:?} as the document writes it, which would make \
             this check the wrong way round"
        );
    }

    // The same three sites, spelled the way the reader takes them. The target is the `hotspot`
    // line above; only `use-with` writes an operand of its own.
    for line in [
        "  hotspot gate \"a gate\"\n    look-at:\n      \"A gate.\"",
        "  hotspot gate \"a gate\"\n    use-with key:\n      \"It does not fit.\"",
        "  on-enter:\n      \"You arrive.\"",
    ] {
        assert!(
            !complains(&with(line)),
            "the reader refuses a site it is supposed to take: {line:?}"
        );
    }

    // `exit` is the one production written as a line an author types, and it is the one the
    // document got right — it was added to the document by the phase that built it.
    assert!(!complains(&with("  exit stair to hall")));
}

/// **Five forms state a quoted label the reader does not require, and the reader is right.**
///
/// `item`, `flag`, `counter`, `hotspot` and `option` all write `"<label>"` unbracketed and all
/// five parse without one. The forms are what is wrong, not the reader: `exit <name> to <room>`
/// expands into a hotspot declared with no label at all, and `print` writes that expansion back as
/// a bare `hotspot door` — so a reader that required labels could not read what this crate's own
/// writer produces, and `.yidam/decisions/round-trip.yml` is the finding that would break.
///
/// `room` and `actor` are not on the list and are not exceptions: their two branches are selected
/// *by* the label, so `room hall` is a section rather than an unlabelled declaration.
#[test]
fn five_forms_state_a_label_the_reader_does_not_require() {
    let bare = [
        ("item", "item spare\n"),
        ("flag", "flag extra\n"),
        ("counter", "counter tally 0..3\n"),
        (
            "hotspot",
            "  hotspot bare\n    look-at:\n      \"Nothing.\"",
        ),
        (
            "option",
            "conversation chat with ann\n  option bare\n    \"Hello.\"\n",
        ),
    ];
    for (name, line) in bare {
        let src = if line.starts_with(' ') {
            with(line)
        } else {
            format!("{line}{}", with(""))
        };
        assert!(
            !complains(&src),
            "`{name}` now requires the label its form states, which is the other repair"
        );
    }
}

/// **Every construct accounts for its quoted string, which six of them used to drop in silence.**
///
/// `ignored.rs` holds the rule that *"every construct accounts for every token on its line"*, and
/// it tests the tokens only — its mutation lands *"before the first quote, so the mutation lands in
/// the grammar rather than inside a response"*. That was the right call for what it measured and it
/// left the other half of every line untested by anything.
///
/// The comparison is the line against itself: parse it, parse it with a quoted string appended, and
/// see whether the reader says anything different. At `start`, `holds`, a verb, `on-enter`, `exit`
/// and `is` it used to say exactly the same thing, which is the definition of a slot nothing reads.
/// It says something different at all six now, out of the same table that gives them their forms.
#[test]
fn every_construct_accounts_for_its_quoted_string() {
    /// Everything the reader objects to about this source, as one string.
    fn said(src: &str) -> String {
        let (mut game, problems) = parse(src, "forms");
        let mut out: Vec<String> = problems
            .iter()
            .map(|p| format!("{}: {}", p.line, p.message))
            .collect();
        out.extend(
            resolve(&mut game)
                .iter()
                .map(|p| format!("{}: {}", p.line, p.message)),
        );
        out.sort();
        out.join("\n")
    }

    // The line under test, and whatever has to stand above it for the line to make sense.
    let cases = [
        ("start", "", "  start", ""),
        ("holds", "", "  holds ann", ""),
        (
            "verb",
            "  hotspot gate \"a gate\"\n",
            "    look-at:",
            "\n      \"A gate.\"",
        ),
        ("on-enter", "", "  on-enter:", "\n      \"You arrive.\""),
        ("exit", "", "  exit stair to hall", ""),
        (
            "is",
            "  hotspot gate \"a gate\"\n",
            "    is key",
            "\n    pick-up:\n      do give key\n      say \"Taken.\"",
        ),
    ];

    let silent: Vec<&str> = cases
        .iter()
        .filter(|(_, before, line, after)| {
            let plain = with(&format!("{before}{line}{after}"));
            let mutated = with(&format!("{before}{line} \"an appended string\"{after}"));
            said(&plain) == said(&mutated)
        })
        .map(|(n, ..)| *n)
        .collect();
    assert!(
        silent.is_empty(),
        "constructs that still take a quoted string their form does not mention: {silent:?}"
    );

    // And the complaint names the form, so an author is told what the line is instead of only that
    // it is wrong.
    let src = with("  start \"a label on start\"");
    assert!(
        said(&src).contains("`start` reads no quoted text — the form is `start`"),
        "the diagnosis should quote the row: {}",
        said(&src)
    );
}
