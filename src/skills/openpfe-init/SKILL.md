---
name: openpfe-init
description: Elicits a terse single-sentence description of what the software exists to solve, derives a filesystem-safe forest folder name from it, and writes `.openpfe/problem/<forest>/problem.md`. Use when starting a new OpenPFE problem forest, before decomposition, or when the user attaches this skill without a populated problem file for that forest.
disable-model-invocation: true
---

# OpenPFE init — new problem forest

## Scope

- **Target path:** `.openpfe/problem/<forest>/problem.md` — create the forest folder and this file (and `.openpfe/problem/` if missing) once the user supplies the line. **`<forest>`** is a short **slug** derived from the problem (see **Forest folder name**).
- This skill **does not** invent the problem statement; stop and reply in chat until the user has given substantive input (see **Conversation**).

## Forest folder name

Pick **one** slug **`<forest>`** from the approved summary sentence:

- **ASCII `kebab-case`:** lowercase letters, digits, and single hyphens between tokens — no spaces, slashes, dots, or leading/trailing hyphens.
- **Meaningful and short:** usually **2–5** words worth of tokens (drop articles like *a/an/the* when they add no meaning). Aim for a name a teammate would recognize a month later.
- **Length:** keep it reasonable for all common filesystems (prefer **≤ 40** characters excluding the `.openpfe/problem/` prefix).
- **Collision:** if `.openpfe/problem/<forest>/` **already exists**, do **not** create or overwrite anything until resolved in chat:
  1. Ask whether this is **the same problem** as that forest or a **new problem**.
  2. **Same:** do **not** mint a **new** slug for the same initiative — send them to **`.openpfe/problem/<forest>/problem.md`** (and **openpfe-drill** on that path when they drill). If that file is missing and they confirm **same**, you may **create only** that **`problem.md`** in the existing folder (still not a new sibling forest). If it already holds their problem, **stop**; no duplicate tree.
  3. **New:** ask **how it differs** from what lives under that folder (one focused question or ask for a short contrast); use their answer to derive a **new** **`<forest>`** slug that reflects the distinction. If that slug collides too, repeat from step 1 until you have a free path.

If the folder exists but **`problem.md`** is missing or empty, still run **same vs new** so the user does not accidentally fork a forest they consider identical.

State the chosen **`<forest>`** in chat before the first write (one line); if the user objects, adjust once.

## What you must convey to the user

Deliver this in chat (you may swap “the user” for “you” / “your” so it reads naturally):

Ask the user for **a high-level problem description that this software should solve**. Ask the user to **think hard about it**, and to **state it in one sentence with as little detail as possible—deeper breakdown happens when we drill down**.

If they already pasted a candidate in their message before you replied, acknowledge it briefly and only refine wording if necessary to satisfy **Problem line** below (one short clarification max).

## Conversation

1. Deliver the prompt unless the user's message clearly contains their final one-sentence problem already.
2. If the answer is a **paragraph**, restate politely that you need **a single sentence** and optionally offer to compress their text into **one suggested sentence** for them to approve (do **not** write **`problem.md`** until they approve that wording or submit an acceptable substitute).
3. If **`.openpfe/problem/<forest>/problem.md` already exists** (for the slug you intend to use) and has **children** signaled by nonzero **`problems`** in YAML and/or **`Drill:`** links to **`./NN/problem.md`**, **stop**: warn that overwriting the file risks mismatching numbered folders unless they intend a reset—ask how to proceed (**abort**, **rewrite `problem.md` only**, or **update `## Summary` only** preserving frontmatter/links). Until they confirm, **do not** blindly replace the whole file.

## Problem line — quality bar

- **Outcome / tension**, not roadmap: what is broken, risky, wasteful, or blocked **without naming features or stack**.
- **One readable sentence**, not a brainstorm list.
- **No sub-problems** here (those belong in **openpfe-drill**).

## Writing `problem.md`

When saving, place the user's approved wording exactly under **`## Summary`**. Use this skeleton (minimal **Navigation**: drill links are added later by **openpfe-drill**):

```markdown
---
problems: {}
---

# Problem

## Summary

<the user's single-sentence problem>

## Navigation

**Drill:**
```

After write: tell them the next step is to **`@`** **openpfe-drill** and **`@`** `.openpfe/problem/<forest>/problem.md` when they want to decompose into sub-problems.
