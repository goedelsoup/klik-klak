//! **The declaration half of this format has no written specification, and the two lines the
//! design document does write about it describe a shape the reader refuses.**
//!
//! `.yidam/decisions/parser-silence.yml` closed on it: *"Sixteen branches each stating their own
//! arity is better than sixteen branches stating nothing, and it is not a specification… Whether
//! the declaration half should be described the way the expression half is — a written grammar,
//! from which the reader is derived rather than compared — is a real question and is not answered
//! here."*
//!
//! [`design/bounded-expression-language.md`] states **nine productions**. Seven are the predicate
//! grammar, which `normal-form` showed is disjunctive normal form and which the reader implements
//! exactly. The other two are Layer 0:
//!
//! ```text
//! site    :=  verb target [ object ]        -- what the player did
//!          |  'on-enter' room               -- where the player got to
//!
//! decl    :=  'exit' name 'to' room         -- an edge between two rooms
//! ```
//!
//! **Two of those three alternatives name an operand the reader rejects.** `look-at door` is *"`door`
//! is not part of `look-at`"* and `on-enter hall` is *"`hall` is not part of `on-enter`"*, because a
//! binding's target and room come from the `hotspot`, `actor` or `room` line above it and never from
//! the binding's own line. Only `exit <name> to <room>` is a line an author can type.
//!
//! So the Layer 0 block is a description of the **model** — where a binding hangs — written in the
//! notation of a **syntax**, and nothing has ever held it against the thing that reads the file.
//! Meanwhile the reader states sixteen forms of its own, in a different notation, in error
//! messages. The document quotes two of their keywords as terminals and describes one of the two
//! correctly.
//!
//! [`design/bounded-expression-language.md`]: ../../../design/bounded-expression-language.md

use game_format::{parse, resolve};

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

/// Every form the reader states, transcribed by hand from its error messages.
///
/// By hand because there is nowhere else to read them from: they are string literals scattered
/// across sixteen match arms, six as `const FORM` and the rest inline, and a regular expression
/// over the source picks up the doc comments that quote them. Each entry is checked below against
/// the reader's own source, so the transcription is verified even though it is a transcription —
/// which is the whole of what a single declared table would replace.
const STATED: &[&str] = &[
    "room <name> \\\"<label>\\\"",
    "room <name>",
    "actor <name> \\\"<label>\\\"",
    "actor <name>",
    "item <name> [<lo>..<hi>] \\\"<label>\\\"",
    "flag <name> [latched] [in <conversation>] \\\"<label>\\\"",
    "counter <name> <lo>..<hi> \\\"<label>\\\"",
    "conversation <name> with <actor>",
    "hotspot <name> \\\"<label>\\\"",
    "option <name> \\\"<line>\\\"",
    "start",
    "holds <actor>",
    "exit <name> to <room>",
    "on-enter",
    "is <item>",
    "use-with [<item>]",
];

/// The reader's own source, read as text because that is where its specification currently lives.
const READER: &str = include_str!("../src/parse.rs");
const DOCUMENT: &str = include_str!("../../../design/bounded-expression-language.md");

/// **Sixteen forms in the reader, nine productions in the document, two keywords in both.**
///
/// And each of the two is spelled twice. `decl := 'exit' name 'to' room` against
/// `exit <name> to <room>`, which agree; `'on-enter' room` against `on-enter`, which do not.
/// Two notations for two constructs is what having no specification looks like when somebody
/// writes half of one — and the half that was written is the half that is wrong.
#[test]
fn the_reader_and_the_document_describe_one_construct_in_common() {
    for form in STATED {
        assert!(
            READER.contains(form),
            "the transcription is stale: the reader no longer states {form:?}"
        );
    }

    let productions = DOCUMENT
        .lines()
        .filter(|l| l.contains(":=") && !l.trim_start().starts_with('>'))
        .count();
    assert_eq!(productions, 9, "production rules written in the document");

    // A construct is in both when the document's grammar names its keyword as a terminal.
    let keywords: Vec<&str> = STATED
        .iter()
        .map(|f| f.split([' ', '<']).next().unwrap_or(f))
        .collect();
    let shared: Vec<&str> = keywords
        .iter()
        .filter(|k| DOCUMENT.contains(&format!("'{k}'")))
        .copied()
        .collect();
    assert_eq!(
        shared,
        ["exit", "on-enter"],
        "keywords the document's grammar quotes as terminals"
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

/// **A quoted string is an argument slot, and no construct accounts for it.**
///
/// `ignored.rs` holds the rule that *"every construct accounts for every token on its line"*, and
/// it tests the tokens only — its mutation lands *"before the first quote, so the mutation lands in
/// the grammar rather than inside a response"*. That is the right call for what it was measuring
/// and it leaves the other half of every line untested.
///
/// The comparison is with and against itself: parse the line, parse the line with a quoted string
/// appended, and see whether the reader says anything different. At six constructs it says exactly
/// the same thing, which is the definition of a slot nothing reads. `start "x"`, `holds ann "x"`,
/// `look-at: "x"` — accepted, dropped, and nothing said.
#[test]
fn a_quoted_string_is_an_argument_slot_no_construct_accounts_for() {
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

    let lines = [
        ("start", "  start"),
        ("holds", "  holds ann"),
        (
            "verb",
            "  hotspot gate \"a gate\"\n    look-at:\n      \"A gate.\"",
        ),
        ("on-enter", "  on-enter:\n      \"You arrive.\""),
        ("exit", "  exit stair to hall"),
        (
            "is",
            "  hotspot gate \"a gate\"\n    is key\n    pick-up:\n      do give key\n      say \"Taken.\"",
        ),
    ];

    // The appended string goes on the first line of each fragment, which is the line under test.
    let mutate = |frag: &str| -> String {
        let mut it = frag.splitn(2, '\n');
        let head = it.next().unwrap_or("");
        match it.next() {
            Some(tail) => format!("{head} \"an appended string\"\n{tail}"),
            None => format!("{head} \"an appended string\""),
        }
    };

    let silent: Vec<&str> = lines
        .iter()
        .filter(|(_, l)| said(&with(l)) == said(&with(&mutate(l))))
        .map(|(n, _)| *n)
        .collect();
    assert_eq!(
        silent,
        ["start", "holds", "verb", "on-enter", "exit", "is"],
        "constructs whose diagnosis is unchanged by a quoted string their form does not mention"
    );
}
