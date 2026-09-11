//! The gate for `design/`.
//!
//! `directories.md` asks a new top-level directory for a gate, and *"if what it holds
//! derives from the corpus, the gate is the derivation."* This is that derivation, moved
//! here from a shell script on the condition `.yidam/decisions/design-directory.yml` wrote
//! down: the day `crates/` exists for another reason, or the day the gate needs to parse
//! rather than pattern-match. Both fired at once.
//!
//! What the move bought, beyond tidiness:
//!
//! * **`.game` files are parsed rather than scanned.** The shell gate matched names against
//!   one flat set, so `carrying quay` passed. [`game_format::resolve`] knows kinds.
//! * **Front matter is read as a document**, so a malformed list is an error rather than an
//!   empty one that quietly satisfies every check over it.
//!
//! What it does *not* do is unchanged and is stated in `design/README.md`: it checks a
//! document rather than a sentence, and `rests_on` proves a path resolves rather than that
//! the node still says what the design thought.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/design-gate is two levels below the root")
        .to_path_buf()
}

#[derive(Debug, Default)]
pub struct Findings(pub Vec<String>);

impl Findings {
    fn say(&mut self, file: &Path, root: &Path, msg: impl AsRef<str>) {
        let shown = file.strip_prefix(root).unwrap_or(file).display();
        self.0.push(format!("{shown}: {}", msg.as_ref()));
    }
    pub fn report(&self) -> String {
        let mut s = String::new();
        for f in &self.0 {
            let _ = writeln!(s, "  {f}");
        }
        s
    }
}

// ── a reader for the front matter this directory writes ─────────────────────
// Deliberately not a YAML library. Three shapes are read — a scalar, a list of scalars,
// and nothing else — and a dependency would be larger than the code it replaced in a
// workspace whose CI has to stay hermetic.

pub struct FrontMatter {
    pub scalars: Vec<(String, String)>,
    pub lists: Vec<(String, Vec<String>)>,
}

pub fn front_matter(source: &str) -> Option<FrontMatter> {
    let mut lines = source.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    let mut fm = FrontMatter {
        scalars: Vec::new(),
        lists: Vec::new(),
    };
    let mut key: Option<String> = None;
    for line in lines {
        if line.trim() == "---" {
            return Some(fm);
        }
        if let Some(item) = line.trim().strip_prefix("- ") {
            if let Some(k) = &key {
                match fm.lists.iter_mut().find(|(n, _)| n == k) {
                    Some((_, v)) => v.push(item.trim().to_string()),
                    None => fm.lists.push((k.clone(), vec![item.trim().to_string()])),
                }
            }
            continue;
        }
        if line.starts_with(char::is_whitespace) {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim().to_string();
            let v = v.trim();
            if v.is_empty() {
                fm.lists.push((k.clone(), Vec::new()));
                key = Some(k);
            } else {
                fm.scalars.push((k.clone(), v.to_string()));
                key = Some(k);
            }
        }
    }
    None // unterminated front matter is not front matter
}

impl FrontMatter {
    pub fn scalar(&self, k: &str) -> Option<&str> {
        self.scalars
            .iter()
            .find(|(n, _)| n == k)
            .map(|(_, v)| v.as_str())
    }
    pub fn list(&self, k: &str) -> &[String] {
        self.lists
            .iter()
            .find(|(n, _)| n == k)
            .map(|(_, v)| v.as_slice())
            .unwrap_or(&[])
    }
}

/// Every relationship a class declares, read from its `.ont.yml`.
pub fn declared_relationships(ont: &str) -> Vec<String> {
    ont.lines()
        .filter_map(|l| l.trim().strip_prefix("- relationship:"))
        .map(|r| r.trim().to_string())
        .collect()
}

fn markdown_links(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = body.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == ']' && chars[i + 1] == '(' {
            let mut j = i + 2;
            let mut depth = 1;
            let mut link = String::new();
            while j < chars.len() && depth > 0 {
                match chars[j] {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                link.push(chars[j]);
                j += 1;
            }
            out.push(link);
            i = j;
        }
        i += 1;
    }
    out
}

fn walk(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            walk(&p, ext, out);
        } else if p.extension().map(|e| e == ext).unwrap_or(false) {
            out.push(p);
        }
    }
}

pub fn check(root: &Path) -> Findings {
    let mut f = Findings::default();
    let design = root.join("design");
    if !design.is_dir() {
        return f;
    }

    let mut docs = Vec::new();
    walk(&design, "md", &mut docs);
    for doc in &docs {
        // design/README.md is the directory's contract rather than a design
        if doc == &design.join("README.md") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(doc) else {
            f.say(doc, root, "cannot be read");
            continue;
        };
        let Some(fm) = front_matter(&source) else {
            f.say(
                doc,
                root,
                "opens without YAML front matter; the first line must be `---`",
            );
            continue;
        };

        match fm.scalar("status") {
            Some("proposal") | Some("adopted") | Some("withdrawn") => {}
            Some(other) => f.say(
                doc,
                root,
                format!("has status: {other} — not one of proposal, adopted, withdrawn"),
            ),
            None => f.say(
                doc,
                root,
                "declares no status:. Use proposal, adopted or withdrawn",
            ),
        }
        if fm.scalar("title").is_none() {
            f.say(doc, root, "declares no title:");
        }

        // `reconciled:` names the commit this document's premises were last read against. A
        // premise moving is not a defect, which is why nothing could ever catch it — so what is
        // checked is that somebody looked since it moved. `.yidam/decisions/moved-ground.yml`.
        //
        // Required rather than honoured where offered. Read under `if let Some`, this covered the
        // one document of two whose author had been thinking about drift, which is the population
        // a check should never be allowed to choose. `.yidam/decisions/second-document.yml`.
        match fm.scalar("reconciled") {
            None => f.say(doc, root, "declares no reconciled:. A design names the commit its premises were last read against, or nothing can say when the ground moved under it"),
            Some(at) => for path in fm.list("rests_on") {
                let since = std::process::Command::new("git")
                    .arg("-C")
                    .arg(root)
                    .args(["rev-list", "--count", &format!("{at}..HEAD"), "--", path])
                    .output();
                let moved = since
                    .ok()
                    .filter(|o| o.status.success())
                    .and_then(|o| {
                        String::from_utf8_lossy(&o.stdout)
                            .trim()
                            .parse::<usize>()
                            .ok()
                    })
                    .unwrap_or(0);
                if moved > 0 {
                    f.say(doc, root, format!(
                        "rests_on: {path} has {moved} commit(s) since `reconciled: {at}`. The premise moved; read the document against it and move the watermark"
                    ));
                }
            },
        }

        let rests = fm.list("rests_on");
        if rests.is_empty() {
            f.say(doc, root, "declares no rests_on:. A design names the corpus nodes it derives from, or it is not derived from them");
        }
        for path in rests {
            if !root.join(path).exists() {
                f.say(doc, root, format!(
                    "rests_on: {path} — no such file. The premise is gone; the design has not noticed"
                ));
            }
        }

        for pair in fm.list("derives_effects_from") {
            let Some((class, rel)) = pair.split_once(':') else {
                f.say(
                    doc,
                    root,
                    format!("derives_effects_from: {pair} — expected `class:relationship`"),
                );
                continue;
            };
            let ont = root.join(".yidam/corpus").join(format!("{class}.ont.yml"));
            match std::fs::read_to_string(&ont) {
                Err(_) => f.say(
                    doc,
                    root,
                    format!(
                        "derives_effects_from: {pair} — the corpus declares no class `{class}`"
                    ),
                ),
                Ok(text) => {
                    if !declared_relationships(&text).iter().any(|r| r == rel) {
                        f.say(doc, root, format!(
                            "derives_effects_from: {pair} — `{class}` declares no `{rel}`. Either the effect is wrong, or the ontology is missing one the design needs. Both are findings; neither is silence"
                        ));
                    }
                }
            }
        }

        let body = source.splitn(3, "---").nth(2).unwrap_or(&source);
        let links = markdown_links(body);
        if body.contains("[verified]") && !links.iter().any(|l| l.contains(".yidam/catalog/")) {
            f.say(doc, root, "carries [verified] and links no catalog entry. design/ may propose; it may not source a claim to nothing");
        }
        for link in links {
            if link.starts_with("http") || link.starts_with("mailto:") || link.starts_with('#') {
                continue;
            }
            let target = link.split('#').next().unwrap_or("");
            if target.is_empty() {
                continue;
            }
            let base = doc.parent().unwrap_or(root);
            if !base.join(target).exists() {
                f.say(
                    doc,
                    root,
                    format!("links {target} — which does not resolve"),
                );
            }
        }
    }

    let mut games = Vec::new();
    walk(&design, "game", &mut games);
    for game_path in &games {
        let Ok(source) = std::fs::read_to_string(game_path) else {
            f.say(game_path, root, "cannot be read");
            continue;
        };
        let name = game_path.display().to_string();
        let (mut game, mut problems) = game_format::parse(&source, &name);
        problems.extend(game_format::resolve(&mut game));
        for p in problems {
            f.say(game_path, root, p.to_string());
        }
    }
    f
}
