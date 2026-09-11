//! **A refusal with nothing to refuse.**
//!
//! [`interaction/entry-binding`] argued that an arrival trigger cannot carry a refusal, and gave
//! the reason as a definition: *"A refusal is what the game says when the player tries something
//! the precondition forbids, and nobody tried anything — they walked into a room."*
//!
//! The same sentence convicts any binding with a refusal and no precondition. With no `when` and no
//! guarded clause there is no state in which the binding is offered and declines, so the line under
//! `else` is text no player can reach — the third check of this kind in the format, after
//! `dead_writes` and `unreachable_values`, and the first to be found in the committed corpus rather
//! than constructed to test one.
//!
//! Both instances were binary bindings, and the second key is why. You cannot use the lantern on
//! the gate without the lantern, so `use-with lantern` is not offered and its refusal never runs.
//! `.yidam/decisions/availability.yml`.
//!
//! [`interaction/entry-binding`]: ../../../.yidam/corpus/interaction/entry-binding.yml

const HEAD: &str = "room a \"A\"\nitem x \"x\"\nflag seen \"seen\"\n\nroom a\n  start\n";
const TAIL: &str = "  hotspot out \"out\"\n    walk-to:\n      ending\n";

fn problems(body: &str) -> Vec<String> {
    let src = format!("{HEAD}  hotspot thing \"a thing\"\n{body}{TAIL}");
    let (mut g, mut p) = game_format::parse(&src, "refusals.game");
    p.extend(game_format::resolve(&mut g));
    p.into_iter().map(|p| p.to_string()).collect()
}

fn unreachable(body: &str) -> Vec<String> {
    problems(body)
        .into_iter()
        .filter(|m| m.contains("nothing to refuse"))
        .collect()
}

/// The shape found in the corpus: a binary binding whose refusal the second key had pre-empted.
#[test]
fn a_refusal_on_an_unguarded_binding_is_refused() {
    let m = unreachable("    use-with x:\n      do    set seen\n      else  \"You cannot.\"\n");
    assert_eq!(m.len(), 1, "{m:?}");
    assert!(m[0].contains("use-with"), "{m:?}");
}

/// And with a gate it is the ordinary form, which is what seventeen of the corpus's bindings are.
#[test]
fn a_refusal_under_a_gate_is_the_ordinary_form() {
    assert!(unreachable(
        "    use-with x:\n      when  not seen\n      do    set seen\n      else  \"Already done.\"\n"
    )
    .is_empty());
}

/// A guarded response list has no gate and is not a defect: every clause is a line the response
/// may take, and `otherwise` is the last of them rather than a refusal.
#[test]
fn a_guarded_response_list_is_not_a_refusal() {
    assert!(unreachable(
        "    look-at:\n      when  seen        \"It has been opened.\"\n      otherwise         \"It is shut.\"\n"
    )
    .is_empty());
}

/// `else when <pred> "<line>"` is a clause of that list too, so a binding carrying only those is
/// offered whenever its own preconditions hold and declines by selecting a line.
#[test]
fn an_else_when_clause_counts_as_something_to_refuse() {
    assert!(unreachable(
        "    use-with x:\n      when  not seen\n      do    set seen\n      else when seen   \"Already done.\"\n"
    )
    .is_empty());
}

/// A gate with no refusal is legal and says the binding is not offered — the form the corpus uses
/// at its two conversation options and its one `on-enter`, and nowhere else.
#[test]
fn a_gate_with_no_refusal_is_legal() {
    assert!(problems("    use-with x:\n      when  not seen\n      do    set seen\n").is_empty());
}
