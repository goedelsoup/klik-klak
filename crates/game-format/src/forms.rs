//! **The written form of every construct, which is the whole specification of the declaration
//! half of this format.**
//!
//! It is a table rather than a grammar, and that is the finding rather than a shortcut.
//! [`design/bounded-expression-language.md`] gives the predicate layer a real grammar because
//! predicates nest — `unary := 'not' unary | atom` has no bound on depth. A declaration does not
//! nest at all: a keyword, a fixed number of names, a couple of optional slots, and nothing on the
//! line can contain another line. Written in BNF that is twenty productions of one alternative
//! apiece, which is this table with more punctuation. `.yidam/decisions/line-format.yml`.
//!
//! **These strings are the error messages.** A form and the sentence that teaches it to an author
//! who got it wrong are the same bytes, so they cannot drift apart — which is what they had done:
//! five of them stated a quoted label the reader has never required, and nothing could see it
//! because each lived alone inside the arm that used it.
//!
//! What the table does **not** do is generate the reader. A construct's arity is checked against a
//! row and its meaning is still a match arm, which is the boundary
//! `.yidam/decisions/parser-silence.yml` drew one level further in.
//!
//! [`design/bounded-expression-language.md`]: ../../../design/bounded-expression-language.md

/// What the quoted string on a construct's line is to that construct.
///
/// Three answers rather than two, because `room` and `actor` use it for a third thing: the same
/// keyword means a declaration with one and a section or site without, so the string is not
/// optional there — it is the discriminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quoted {
    /// No slot for it. A line carrying one is an error, because a construct must account for its
    /// whole line and the quoted half was the half nothing checked.
    Never,
    /// Read if it is there, and the construct is the same construct without it.
    Reads,
    /// Selects this form over another with the same keyword.
    Selects,
}

/// One construct, as an author writes it.
#[derive(Debug, Clone, Copy)]
pub struct Form {
    /// The word the line begins with.
    pub keyword: &'static str,
    /// The line, in the notation an author reads — and the text of the reader's own complaint.
    pub written: &'static str,
    /// What the quoted string on the line is for.
    pub quoted: Quoted,
}

const fn f(keyword: &'static str, written: &'static str, quoted: Quoted) -> Form {
    Form {
        keyword,
        written,
        quoted,
    }
}

/// **Every construct of the declaration half, in the order a file introduces them.**
///
/// The binding body — `when`, `else`, `otherwise`, `say`, `do`, `ending` — is deliberately absent.
/// It is the one part of this format that is not a flat line: `when` carries a predicate, which is
/// the recursive half and has a grammar of its own, and `else when` chains. A table is the wrong
/// shape for it for exactly the reason it is the right shape for everything below.
pub const FORMS: &[Form] = &[
    f("room", "room <name> \"<label>\"", Quoted::Selects),
    f("room", "room <name>", Quoted::Never),
    f("actor", "actor <name> \"<label>\"", Quoted::Selects),
    f("actor", "actor <name>", Quoted::Never),
    f(
        "item",
        "item <name> [<lo>..<hi>] [\"<label>\"]",
        Quoted::Reads,
    ),
    f(
        "flag",
        "flag <name> [latched] [in <conversation>] [\"<label>\"]",
        Quoted::Reads,
    ),
    f(
        "counter",
        "counter <name> <lo>..<hi> [\"<label>\"]",
        Quoted::Reads,
    ),
    f(
        "conversation",
        "conversation <name> with <actor> [\"<label>\"]",
        Quoted::Reads,
    ),
    f("hotspot", "hotspot <name> [\"<label>\"]", Quoted::Reads),
    f("option", "option <name> [\"<line>\"]", Quoted::Reads),
    f("start", "start", Quoted::Never),
    f("holds", "holds <actor>", Quoted::Never),
    f("exit", "exit <name> to <room>", Quoted::Never),
    f("on-enter", "on-enter", Quoted::Never),
    f("is", "is <item>", Quoted::Never),
    f("look-at", "look-at", Quoted::Never),
    f("pick-up", "pick-up", Quoted::Never),
    f("talk-to", "talk-to", Quoted::Never),
    f("walk-to", "walk-to", Quoted::Never),
    f("use-with", "use-with [<item>]", Quoted::Never),
];

/// The form a line beginning with `keyword` is written in, given whether it carries a quoted
/// string.
///
/// `None` for a word this table does not cover, which is every keyword of the binding body and
/// every word that is not a keyword at all.
pub fn form(keyword: &str, quoted: bool) -> Option<&'static str> {
    let mut rows = FORMS.iter().filter(|r| r.keyword == keyword).peekable();
    rows.peek()?;
    let rows: Vec<&Form> = rows.collect();
    if rows.len() == 1 {
        return Some(rows[0].written);
    }
    rows.iter()
        .find(|r| (r.quoted == Quoted::Selects) == quoted)
        .or(rows.first())
        .map(|r| r.written)
}

/// **Whether a quoted string on this keyword's line is an argument nothing reads.**
///
/// True only for a keyword the table covers and none of whose forms has a slot for one. Six
/// constructs answered yes and dropped it in silence — `start "x"`, `holds ann "x"`,
/// `look-at: "x"`, `on-enter: "x"`, `exit a to b "x"`, `is key "x"` — each giving a diagnosis
/// byte-for-byte identical to the same line without it.
pub fn stray_quote(keyword: &str) -> bool {
    let mut any = false;
    for r in FORMS.iter().filter(|r| r.keyword == keyword) {
        any = true;
        if r.quoted != Quoted::Never {
            return false;
        }
    }
    any
}
