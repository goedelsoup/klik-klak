# Name resolution for a `.game` specimen.
#
# The predicate layer's admission rule is that every operand must name something the game
# declares. This is that rule, checked. It says nothing about whether the game is winnable,
# coherent, or possible — only that no binding reaches for a name nothing declares, which is
# the one property a specimen must have before any other question about it means anything.
#
# Run over the same file twice: pass one collects declarations, pass two resolves references.

function clean(l) {
  if (l ~ /^[ \t]*#/) return ""          # whole-line comment
  gsub(/"[^"]*"/, " ", l)                # authored prose is not a name
  sub(/\?\?.*$/, " ", l)                 # `?? UNSAYABLE` annotations
  gsub(/:/, " ", l)                      # `use-with crowbar:` -> `use-with crowbar`
  sub(/^[ \t]+/, "", l)
  return l
}

function isname(t) {
  if (t == "") return 0
  if (t in KW) return 0
  if (t ~ /^[0-9]/) return 0             # literals, and `0..3` ranges
  if (t !~ /^[a-z][a-z0-9-]*$/) return 0 # operators, punctuation
  return 1
}

BEGIN {
  split("when else otherwise not and or carrying chosen count do say start ending holds " \
        "with set clear incr give take move enter leave remove enable disable retire " \
        "look-at use-with talk-to pick-up walk-to give-to " \
        "room actor item flag counter hotspot option conversation", k, " ")
  for (i in k) KW[k[i]] = 1

  split("room actor item flag counter hotspot option conversation", d, " ")
  for (i in d) DECLARES[d[i]] = 1
}

# ── pass one: what this game declares ────────────────────────────────────────
NR == FNR {
  l = clean($0)
  if (l == "") next
  n = split(l, t, /[ \t]+/)
  if (n >= 2 && (t[1] in DECLARES) && isname(t[2])) decl[t[2]] = FNR
  next
}

# ── pass two: what it reaches for ────────────────────────────────────────────
{
  l = clean($0)
  if (l == "") next
  n = split(l, t, /[ \t]+/)
  if (n >= 2 && (t[1] in DECLARES)) start_at = 3   # the declared name is not a reference
  else start_at = 1
  for (i = start_at; i <= n; i++)
    if (isname(t[i]) && !(t[i] in decl))
      printf "%s:%d: `%s` is not declared\n", FILENAME, FNR, t[i]
}
