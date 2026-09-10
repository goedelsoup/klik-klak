#!/usr/bin/env bash
#
# The gate for design/.
#
# `directories.md` asks a new top-level directory for three things: a note in AGENTS.md
# saying what rule it does not get to relax, a decision record, and a gate — "if what it
# holds derives from the corpus, the gate is the derivation." This is that derivation.
#
# Pure bash, awk and grep on purpose. It runs identically from `mise run design-check`,
# from CI, and from a fresh clone with no toolchain installed — which matters because the
# alternative was standing up `crates/`, and that is a separate structural decision this
# repository has not made. See .yidam/decisions/design-directory.yml.
#
# What it CANNOT do is in design/README.md, under "What this gate does not catch". Read it
# before trusting a green run.

set -uo pipefail
cd "$(dirname "$0")/.."

fail=0
err() { printf 'ERROR  %s\n' "$*" >&2; fail=1; }

# The front-matter block of $1 — everything between the first two `---` lines.
# Exits 1 when the file does not open with one.
frontmatter() {
  awk 'NR==1 && $0!="---" { exit 1 }
       NR>1 { if ($0=="---") exit; print }' "$1"
}

# The `- ` items under a `<key>:` block in $1's front matter.
listitems() {
  frontmatter "$1" | awk -v k="$2:" '
    $1==k                                  { grab=1; next }
    grab && /^[[:space:]]*-[[:space:]]/    { sub(/^[[:space:]]*-[[:space:]]*/,""); print; next }
    grab && /^[^[:space:]]/                { grab=0 }'
}

# The scalar value of `<key>:` in $1's front matter.
scalar() {
  frontmatter "$1" | awk -v k="$2:" '$1==k { $1=""; sub(/^[[:space:]]+/,""); print; exit }'
}

# Subdirectories included: a specimen lives under design/specimen/ and its findings are a
# document like any other. Only design/README.md is skipped, because it is the directory's
# contract rather than a design.
shopt -s nullglob
docs=()
while IFS= read -r f; do docs+=("$f"); done < <(find design -name '*.md' | sort)

if [ ${#docs[@]} -eq 0 ]; then
  echo "design-check: no documents in design/ — nothing to derive."
  exit 0
fi

checked=0
for doc in "${docs[@]}"; do
  [ "$doc" = "design/README.md" ] && continue
  checked=$((checked + 1))
  echo "design-check: $doc"

  if ! frontmatter "$doc" >/dev/null 2>&1; then
    err "$doc opens without YAML front matter; the first line must be '---'."
    continue
  fi

  # ── standing ────────────────────────────────────────────────────────────
  # A design document says what its own claims are worth. Absent, a reader cannot
  # tell a live proposal from one the corpus already refuted.
  status=$(scalar "$doc" status)
  case "$status" in
    proposal|adopted|withdrawn) ;;
    "") err "$doc declares no status:. Use proposal, adopted or withdrawn." ;;
    *)  err "$doc has status: $status — not one of proposal, adopted, withdrawn." ;;
  esac
  [ -n "$(scalar "$doc" title)" ] || err "$doc declares no title:."

  # ── the derivation ──────────────────────────────────────────────────────
  # Every premise must still be in the corpus. This is the check the directory
  # exists to have: a design standing on a node that was withdrawn is standing on
  # nothing, and this repository has twice overturned a conclusion by re-reading
  # material it already held.
  premises=0
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    premises=$((premises + 1))
    [ -e "$path" ] || err "$doc rests_on: $path — no such file. The premise is gone; the design has not noticed."
  done < <(listitems "$doc" rests_on)
  [ "$premises" -gt 0 ] || err "$doc declares no rests_on:. A design document names the corpus nodes it derives from, or it is not derived from them."

  # ── the effect vocabulary is read off the ontology, not invented here ───
  # Each entry is `class:relationship` and must be a relationship that class declares.
  # A failure here is ambiguous on purpose: either the design named an effect that does
  # not exist, or the ontology is missing a relationship the design found it needs.
  while IFS= read -r pair; do
    [ -n "$pair" ] || continue
    cls=${pair%%:*}
    rel=${pair#*:}
    ont=".yidam/corpus/$cls.ont.yml"
    if [ ! -f "$ont" ]; then
      err "$doc derives_effects_from: $pair — the corpus declares no class '$cls'."
    elif ! grep -qE "^[[:space:]]*-[[:space:]]*relationship:[[:space:]]*${rel}[[:space:]]*$" "$ont"; then
      err "$doc derives_effects_from: $pair — '$cls' declares no '$rel' relationship. Either the effect is wrong, or the ontology is missing one the design needs. Both are findings; neither is silence."
    fi
  done < <(listitems "$doc" derives_effects_from)

  # ── provenance is not relaxed here ──────────────────────────────────────
  # design/ may assert intent, which the corpus forbids. It may not assert evidence.
  # A document carrying [verified] must cite a catalog entry by link.
  if grep -q '\[verified\]' "$doc" && ! grep -q ']([^)]*\.yidam/catalog/' "$doc"; then
    err "$doc carries [verified] and links no catalog entry. design/ may propose; it may not source a claim to nothing."
  fi

  # ── links resolve ───────────────────────────────────────────────────────
  while IFS= read -r link; do
    [ -n "$link" ] || continue
    case "$link" in http*|mailto:*) continue ;; esac
    link=${link%%#*}
    [ -n "$link" ] || continue
    [ -e "$(dirname "$doc")/$link" ] || err "$doc links $link — which does not resolve."
  done < <(grep -oE '\]\([^)]+\)' "$doc" | sed -e 's/^](//' -e 's/)$//')
done

# ── specimens ───────────────────────────────────────────────────────────────
# A `.game` file is a worked example authored in a proposed language. The one property it
# must have before any other question about it means anything is that no binding reaches
# for a name nothing declares — which is the predicate layer's admission rule, checked.
#
# What this deliberately does not do is decide whether the game is winnable. That is
# `reachability`'s job, it needs the effect vocabulary to be complete, and
# design/specimen/README.md records that it is not.
games=(design/*/*.game design/*.game)
for game in "${games[@]}"; do
  echo "design-check: $game"
  out=$(awk -f design/names.awk "$game" "$game")
  if [ -n "$out" ]; then
    printf '%s\n' "$out" | while IFS= read -r line; do printf 'ERROR  %s\n' "$line" >&2; done
    err "$game reaches for names it does not declare (above)."
  fi
done

if [ "$fail" -ne 0 ]; then
  echo
  echo "design-check: FAILED. See design/README.md for what each check is for."
  exit 1
fi

echo "design-check: $checked document(s), ${#games[@]} specimen(s), derivation intact."
