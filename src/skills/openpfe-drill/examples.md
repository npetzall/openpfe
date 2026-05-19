# OpenPFE drill — examples

Sibling of **`SKILL.md`**. Referenced from the main skill for path walk-throughs and canonical **`problem.md`** shape.

## Path walk-through

Assume **`.openpfe/problem/`** as the common prefix.

| Step | Input file | New file(s) |
|------|------------|-------------|
| 1 | `problem.md` | `01/problem.md` |
| 2 | `01/problem.md` | `01/01/problem.md` |
| Fork | `01/problem.md` again | `01/03/problem.md` (if `01`,`02` taken) |
| Slug | `building-software/problem.md` | `building-software/01/problem.md` |
| Multi | `01/problem.md`, **`N = 1`**, **`K = 2`** | `01/01/problem.md`, `01/02/problem.md`; parent registry updated |

There must already be a **`problem.md`** to drill (at any allowed path under **`.openpfe/problem/`**); attach it or paste its path with the skill in the same message.

## Registry-driven paths (no `ls`)

- Input **`.openpfe/problem/alpha/problem.md`**, empty **`problems`**, **`K = 1`** ⇒ **`.openpfe/problem/alpha/01/problem.md`**, input gets **`"01": "<short desc>"`**.
- Same input, keys **`01`**, **`02`**, **`K = 2`** ⇒ **`03`**, **`04`** under **`alpha/`**.
- Input **`.openpfe/problem/problem.md`** (file at tree “root” of the folder): children **`01/problem.md`**, **`02/problem.md`** under **`.openpfe/problem/`**.

## Minimal full example (one new child, `K = 1`)

Paths are under **`.openpfe/problem/demo-area/`**; **`C`** = `demo-area/`. User answer supports one distinct sub-problem; next id **`N = 1`** ⇒ child **`01/`**.

**1. Input — `.openpfe/problem/demo-area/problem.md`** (before patch)

```markdown
---
problems: {}
---

## Summary

Releases slip because coordination between teams breaks down.

## Navigation

(No **Deeper** links yet.)
```

**2. New child — `.openpfe/problem/demo-area/01/problem.md`**

```markdown
---
problems: {}
---

## Summary

Design–build handoffs have no clear owner, so work waits on the wrong queue.

## Navigation

(No **Deeper** links yet—add `./NN/problem.md` entries after this node gains children.)
```

**3. Same input file** (after patch: **merge** **`problems`**, extend **`Deeper`**)

```markdown
---
problems:
  "01": "Design–build handoffs have no clear owner, so work waits on the wrong queue."
---

## Summary

Releases slip because coordination between teams breaks down.

## Navigation

- **Deeper:** [`./01/problem.md`](./01/problem.md)
```
