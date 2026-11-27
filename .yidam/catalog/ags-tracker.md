---
name: Adventure Game Studio issue tracker and changelog
description: The AGS project's GitHub issues and its Changes.txt, where the people maintaining the engine since Chris Jones say what they changed in it and why.
type: other
obtained: true
retrieved: 2026-11-26
ttl_days: 365
location:
  - kind: url
    value: https://github.com/adventuregamestudio/ags/issues/1134
    description: "\"Don't error on Dialog having no active options\", ivan-mogilko, opened 20 November 2020, closed 30 January 2021 by commit 197d5acac6e7"
  - kind: url
    value: https://github.com/adventuregamestudio/ags/commit/197d5acac6e7b8378dc9a24497efaca111273e4b
    description: "\"Engine: stop dialog without errors if all options have been turned off\", 30 January 2021, Engine/ac/dialog.cpp"
  - kind: url
    value: https://github.com/adventuregamestudio/ags/issues/2981
    description: "\"New compiler: false positive trigger for \\\"code execution cannot reach\\\" in Dialog scripts\", ivan-mogilko, opened 6 June 2026, three posts to 13 June 2026, open when read"
  - kind: url
    value: https://raw.githubusercontent.com/adventuregamestudio/ags/master/Changes.txt
    description: "the project changelog at master 9c7f66d4da16 (17 November 2026), 7374 lines; the VERSION 3.5.1, June 2021 section"
  - kind: url
    value: https://raw.githubusercontent.com/adventuregamestudio/ags/ags4/Editor/AGS.Editor/Utils/DialogScriptConverter.cs
    description: "the editor's dialog-script converter at branch ags4, 4888edc04d11 (25 November 2026)"
  - kind: url
    value: https://raw.githubusercontent.com/adventuregamestudio/ags/ags4/Compiler/script2/cs_parser.cpp
    description: "the AGS 4 compiler's parser at the same commit; the two sites that emit \"Code execution cannot reach this point\""
---

**This is the second AGS testimony entry, and the first from the people who maintain AGS now.**
[ags-forums](ags-forums.md) holds Chris Jones's posts and what authors asked. This entry holds what
the maintainers since 2011 wrote in the project's own tracker and changelog: what they changed and
why. Most of it is by ivan-mogilko, the repository's most frequent contributor by a wide margin (7,701
commits when read, against 895 for the next). That he posts on the forum as Crimson Wizard is
widely said and was not checked here. A maintainer's issue is testimony about intent in the same way a dev diary is. A commit
or a source file next to it is an artifact, and [ags-source](ags-source.md) is where the artifact
half already lives.

It was read for one question: whether AGS authors experienced an orphaned dialogue option as a loss
(`.yidam/corpus/failure-mode/orphaned-dialogue-branch.yml`). The forum had been sampled twice for it.
The tracker is a different kind of document, written by the people who decide what the engine
checks, so it shows what they treated as a problem.

## What was read

**Searches.** GitHub issue search over the repository, which held about 3,200 issues and pull
requests when read. All 37 issues with "dialog" in the title were listed. Searches for
"unreachable", "dead code", "never shown", "option state", "OptionOffForever", "eOptionOffForever" and
"off-forever" returned nothing about dialog option reachability except #2981. The phrase searches
"dialog option unreachable", "dialog option never", "unused dialog option" and "dialog option
warning" returned nothing at all.

**#1134 (2020-2021).** Opened by ivan-mogilko: when every option has been turned off, "user expects
for current dialog to just end automatically when currently running option finishes, but instead
AGS reports a script error. It should just stop the dialog instead." Closed by commit 197d5ac. That
commit replaces `quit("!DoDialog: all options have been turned off")` with
`debug_script_warn("Dialog: all options have been turned off, stopping dialog.")` and a return. The
changelog's VERSION 3.5.1, June 2021 section records it as "Dialogs with no enabled options left will
be now stopped, instead of raising script error." Master still has the warning-and-stop, in
`Engine/ac/dialog.cpp`.

**#2981 (2026).** AGS 4's new compiler warns "code execution cannot reach" after an explicit `return`
in a dialog option. ivan-mogilko calls it a false positive, because the converted script is "a bunch
of "if/else if" blocks, so what happens in one block should not affect another". fernewelten, who
wrote the new compiler, says the warning fires on code following a `return`, and guesses the
converter's own appended `return` makes two in a row. The thread agrees to fix the converter.

**`DialogScriptConverter.cs` (ags4).** `option-off-forever N` becomes
`dialog[ID].SetOptionState(N, eOptionOffForever);`, and `option-off` and `option-on` become the same
call with `eOptionOff` and `eOptionOn`. Before that it checks one thing: that `N` is between 1 and
the dialog's option count, or it raises "Dialog does not have an option number". Each `return` and
`stop` becomes a `return` of a `RUN_DIALOG_*` constant that closes the option's block.

**`cs_parser.cpp` (ags4).** Both sites that emit "Code execution cannot reach this point" are gated
on `_nest.JumpOutLevel() < _nest.TopLevel()`. That is the parser's record of having passed a jump
out of the current nesting level. It is a control-flow check inside one function.

## What it shows

- **The engine's one dialog reachability check became a warning in 3.5.1.** Before June 2021, a
  dialog with every option off stopped the game, which is the error the 2011 forum thread in
  [ags-forums](ags-forums.md) was about. Since then the dialog ends and a debug warning is logged.
  [verified]
- **The maintainer framed the all-off state as an ending an author means, not as a mistake.** The
  issue gives what the "user expects" as the reason. [verified] for the wording.
- **Option state is a runtime argument to every static check AGS has.** The converter checks that
  an option number exists. The compiler checks control flow inside a function. Neither has the
  state of an option, because by the time the compiler sees it, `option-off-forever` is an ordinary
  function call. [verified] for what each check is gated on; that no other AGS check reads option
  state is [inference] from the two read and from the searches.
- **Nobody in the tracker asked for orphan detection.** The searches found no issue requesting it.
  The one unreachability report involving dialogs is about a warning that should not have fired.
  [inference] — an absence in a search, over an issue tracker that holds the project's own work
  items rather than every author's wishes.

## What the tracker and changelog hold that nobody here has looked at

The forum's "Editor Development" board, where feature requests were argued before the project moved
to GitHub in 2011-2012, was not searched for this question. Pull requests were listed only where the
issue search returned them. The other open dialog issues — #1056 (not repeating a dialog's startup
section), #3090 (a `goto` between options in one dialog) and #2972 — were listed and not opened. The
changelog was searched for dialog and option entries and not read whole, and its entries before
3.3.3 were not read.
