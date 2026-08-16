# patch_shared_docs.awk — append an alias directory to every shared field type.
#
# svd2rust emits one canonical field type (e.g. `scroly::ScreenW`) and re-exports
# it under other names (`pub use ... as ModeW;`) for every field sharing its enum.
# Jump-to-definition on an alias lands on the canonical type, whose doc describes
# only its first use ("Enable the display; blanks the screen") — confusing in an
# unrelated context (a CIA interrupt-mask field reusing the generic enable type).
#
# For each canonical type shared across more than one module, this appends a
# "Shared field type" directory: an intra-doc link to every alias so the reader
# can click through to each one's own description. Purely-local reuse (all aliases
# in the defining register) is left alone — it is already clear in context.
#
# Invoke over the whole tree in one pass, e.g.:
#   awk -f patch_shared_docs.awk $(fd -e rs . src)
# Uses only base awk. Sole caller: generate_c64_pac.sh.

function module_of(fname,   p) {
    p = fname
    sub(/^.*\/src\//, "", p)
    sub(/\.rs$/, "", p)
    if (p == "lib") return "crate"
    gsub(/\//, "::", p)
    return "crate::" p
}

function resolve(target, mod,   parent, rest) {
    if (target ~ /^crate::/) return target
    if (target ~ /^super::/) {
        parent = mod
        sub(/::[^:]+$/, "", parent)       # drop this leaf module
        rest = target
        sub(/^super::/, "", rest)
        return parent "::" rest
    }
    return mod "::" target                 # bare local name, or `x::y` from this module
}

function ultimate(sym,   guard) {
    while (sym in edge) {                   # `guard` is function-local: fresh each call
        if (sym in guard) return ""        # cycle
        guard[sym] = 1
        sym = edge[sym]
    }
    return (sym in def) ? sym : ""
}

# --- pass 1: record definitions, edges, and every line (for rewrite in END) ---
FNR == 1 { mod = module_of(FILENAME) }
{
    line[FILENAME, FNR] = $0
    if (FNR > nlines[FILENAME]) nlines[FILENAME] = FNR
}
/^[[:space:]]*pub use[[:space:]]+[A-Za-z0-9_:]+[[:space:]]+as[[:space:]]+[A-Za-z0-9_]+[[:space:]]*;/ {
    t = $0
    sub(/^[[:space:]]*pub use[[:space:]]+/, "", t)
    split(t, a, /[[:space:]]+as[[:space:]]+/)
    target = a[1]
    alias = a[2]
    sub(/[[:space:]]*;.*$/, "", alias)
    sym = mod "::" alias
    edge[sym] = resolve(target, mod)
    amod[sym] = mod
    aliaslist[++naliases] = sym
    next
}
/^[[:space:]]*pub type[[:space:]]+[A-Za-z0-9_]+/ {
    t = $0
    sub(/^[[:space:]]*pub type[[:space:]]+/, "", t)
    name = t
    sub(/[^A-Za-z0-9_].*$/, "", name)
    sym = mod "::" name
    def[sym] = FILENAME
    defline[sym] = FNR
}

END {
    # group aliases under their ultimate definer
    for (i = 1; i <= naliases; i++) {
        root = ultimate(aliaslist[i])
        if (root != "") members[root] = members[root] SUBSEP aliaslist[i]
    }

    for (root in members) {
        defmod = root
        sub(/::[^:]+$/, "", defmod)

        n = split(members[root], mem, SUBSEP)
        cross = 0
        m = 0
        for (i = 1; i <= n; i++) {
            if (mem[i] == "") continue     # leading empty field from SUBSEP prefix
            list[++m] = mem[i]
            if (amod[mem[i]] != defmod) cross = 1
        }
        if (!cross) { delete list; delete mem; continue }

        # insertion-sort the alias symbols for deterministic output
        for (i = 2; i <= m; i++) {
            key = list[i]; j = i - 1
            while (j >= 1 && list[j] > key) { list[j+1] = list[j]; j-- }
            list[j+1] = key
        }

        block = "///\n"
        block = block "/// **Shared field type.** The same writer is reused for the fields below;\n"
        block = block "/// each keeps its own description — click through to read it in context:"
        for (i = 1; i <= m; i++) {
            disp = list[i]; sub(/^crate::/, "", disp)
            block = block "\n/// - [`" disp "`](" list[i] ")"
        }

        f = def[root]
        insertAt[f, defline[root]] = block
        rewrite[f] = 1
        delete list; delete mem
    }

    for (f in rewrite) {
        for (i = 1; i <= nlines[f]; i++) {
            if ((f, i) in insertAt) printf "%s\n", insertAt[f, i] > f
            printf "%s\n", line[f, i] > f
        }
        close(f)
        patched++
    }
    print "patch_shared_docs: annotated shared field types in " patched " file(s)"
}
