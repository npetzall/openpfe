---
name: openpfe-drill
description: Under `.openpfe/problem/` only; user supplies one input `problem.md` (`@` skill + `@` file or pasted path); YAML `problems` on that file drives child ids; short drill chat; may emit child `problem.md` under two-digit folders; updates `problems` and Deeper links. PFE drill-down for openpfe.
disable-model-invocation: true
---

# OpenPFE drill (one pass, one or many sub-problems)

## Companion files (same directory as `SKILL.md`)

Use the **Read** tool on these paths **relative to this skill’s folder** (e.g. `src/skills/openpfe-drill/…` in this repo). Links are one level deep from here: [examples.md](examples.md), [prompt-log.md](prompt-log.md), [differentiation-extras.md](differentiation-extras.md).

| File | When |
|------|------|
| **[examples.md](examples.md)** | **Must read** once per invocation before **Workflow** step **6** (new child files) if not already read—covers canonical shapes before step **7** (input patch) too. |
| **[prompt-log.md](prompt-log.md)** | **Read** if the user or project requests **`prompt.md`** logging (**Workflow** step 4). |
| **[differentiation-extras.md](differentiation-extras.md)** | **Read** only if a **draft child** might duplicate **`../problem.md`** (ancestor), not for normal **draft vs existing child** work (**Differentiation**). |

You may **cite these filenames** in chat (“see **examples.md**”) without loading them; loading is only when you **Read** the file.

## Input (read once; say “the input” after this)

**Meaning:** **Input** is the **single** existing **`problem.md`** the user aims this skill at for **this** run—the file you read, quote **`## Summary`** from, and later patch with **`problems`** / **Deeper**. In chat, say **the input** or **the input file**; do **not** use a separate variable name like `SRC` in user-facing text.

**Valid path:** repo-relative, resolves logically under **`.openpfe/problem/`**, and the filename is exactly **`problem.md`** (e.g. **`.openpfe/problem/problem.md`**, **`.openpfe/problem/building-software/01/problem.md`**, **`.openpfe/problem/foo/bar/problem.md`**). Parent folders may use **any** naming the repo already uses; only **new child folders** this skill creates use two-digit **`NN`**, not the path to the input file.

**How to supply it (only these ways):**

1. **`@`** this skill **and** **`@`** exactly one file that satisfies **Valid path** above. That attachment **is** the input.
2. **Or** the same message includes the **full repo-relative path** to exactly one such file (when there is no qualifying attachment).

**If the problem file is missing:** zero qualifying attachments **and** no usable path → **stop immediately**; do not guess. Say this skill **must** run against a specific **`problem.md`** under **`.openpfe/problem/`**. Ask the user to **`@`** that file with the skill **or** paste the path (examples: **`.openpfe/problem/problem.md`**, **`.openpfe/problem/my-area/problem.md`**).

**Ambiguous:** more than one attached **`problem.md`** under **`.openpfe/problem/`** → **stop**; ask for a single **`@`** or one path.

**Do not** use **`SRC =`**, env vars, or other resolution tricks—only the two bullets under **How to supply it**.

If a supplied path fails **Valid path**, **stop** and say it must live under **`.openpfe/problem/`** and be named **`problem.md`**, then point them to **How to supply it** above.

## Scope

- **Tree:** only files this skill reads or writes live under **`.openpfe/problem/`**; the drill target is always one **`problem.md`** (see **Input**).
- **New children** this skill creates are always **`C/<NN>/problem.md`** where **`<NN>`** is a **two-digit** id (`01`…`99`)—that convention applies to **child folders only**, not to the path of the input file.
- **Optional sidecar:** **`prompt.md`** — only if the user or project asks; full contract in **[prompt-log.md](prompt-log.md)** (**Read** that file before appending).

## Goal

One invocation = one drill step:

1. Read **the input** `problem.md`.
2. **User-facing chat** (see that section): quote **`## Summary`**, ask what **causes** it; stay fork-finding, not solution design. Treat answers as **problems** (tension, harm, risk, blocked outcome—not feature lists).
3. **Analyze the user’s answer** for how many **distinct** sub-problems it supports (**`K_init` ≥ 1**). Before new work, compare to **every existing child** of the input: use the input’s **`problems`** map as the fast signal; if wording matches an existing **`NN`**, do **not** silently add a sibling—ask **“How is this different from …?”** (see **Differentiation**). Read each existing child’s **`## Summary`** when present. Prefer **one** sub-problem when one coherent problem covers the answer; avoid bullet inflation. If ambiguous, default to fewer or ask **one** short clarifier.
4. After **Differentiation**, write each accepted sub-problem only as **`C/<NN>/problem.md`**. Let **`K`** be the count accepted (**`K = 0`**: create nothing; usually **`K ≥ 1`**).
5. Patch **the input file** only: **Deeper** links and **`problems`** in frontmatter (see **Child registry**; path illustrations in **[examples.md](examples.md)**). New child files: **no** sibling links, **no** **`## Differentiation`** in the new file (see **Links**). No “parent” link in children—directory layout implies the parent.
6. End with a **brief** note on re-invoking (**`@`** this skill + the next **`problem.md`**, or paste path)—do not repeat this whole spec.

**Child ids** for the **next** run come only from **the input file’s `problems` map** (see **Child registry**), not from listing **`C/`**.

## User-facing chat (required)

Stay **short** and **plain**.

- **Do not** open with ops detail: no path banner, no “registry empty”, no “will add `01/`…”—do that **silently** (see **Workflow**). Mention paths or frontmatter only if the user debugs or asks.
- **Do** use this shape:

  1. **Let’s drill down.**
  2. **Your current problem is** — quote **`## Summary`** (or the closest short phrase if missing).
  3. Ask what **causes** it / what sits **underneath**—forks and boundaries, not a spec.
  4. One line: stay **a bit general** unless they already narrowed; they can revisit any node.

Differentiation questions: **one short sentence** when needed.

## Folder layout under `C` (path only)

Let **`C`** be the directory containing the input **`problem.md`**. All **new** children this run are **`C/01/problem.md`**, **`C/02/problem.md`**, … according to **Child registry** (consecutive two-digit ids).

## Child registry — `problems` (authoritative; do not scan `C/` for ids)

YAML **`problems`** on the input file is the edge list to children: **two-digit key** → one-line description (problem, not solution). Quoted YAML keys (`"01"`, `"08"`) avoid octal issues.

- **Empty / missing `problems`:** next id **`N = 1`** → folder **`01`**. No frontmatter yet ⇒ treat as **`{}`** until first patch.
- **Non-empty:** **`N = 1 + max`** of numeric values of keys matching **`^\d{2}$`**. New batch uses **`N … N+K-1`** as **`NN`**, require **`N + K - 1 ≤ 99`**. If **`K = 0`**, skip writes.
- After writes, **merge** new keys into the input’s **`problems`**. Each **new** child file gets **`problems: {}`** in frontmatter.
- If **`C/<NN>/problem.md`** already exists for a planned **`<NN>`**, stop—registry and disk disagree.

Path illustrations (registry-driven; no `ls`) and the **minimal full `problem.md` example**: **[examples.md](examples.md)**—**Read** before writes per **Companion files**.

## Problem framing (required)

New **`problem.md`** files: **problem node** tone—**Summary** + body as problems/constraints/tensions, not a causal essay. **Lean child:** no **`## Differentiation`**, no sibling links in **Navigation** for newly created files; differentiation stays in chat and in **the input node’s `problems` / Deeper** (the registry for **its** children).

## Differentiation (required before writing)

**Subject:** each **candidate child** you might write—**draft summaries** for **new** **`C/<NN>/problem.md`** files. **Goal:** every **accepted** draft is a distinct **sub-problem under the input (current node)** and does **not** duplicate an **existing child** on disk.

Avoid duplicate **siblings** (existing **`NN`** under the same input).

**Signals for each existing child `NN`:** **(A)** **`problems["NN"]`** on the input and **(B)** **`## Summary`** in **`C/NN/problem.md`** when it exists—those describe **children already produced**, not the parent’s own Summary as a substitute for a child.

1. Build the **sibling corpus** (existing **children** of the input) without listing **`C/`**: from the input’s **`problems`** keys (`^\d{2}$` → **`./NN/problem.md`** relative to **`C`**), plus parse **Navigation** / **Deeper** for relative links to **`problem.md`** that resolve under **`C`**. Merge paths; dedupe. If the corpus is empty, still compare each draft to the input’s **Summary** to catch a draft that **only restates the current node** instead of naming a **deeper child** problem.
2. Read the input’s **Summary**, **Navigation**, and all **`problems`** lines; for each corpus **child** that exists, read its **Summary** and **Navigation**.
3. Draft one **Summary** (and body angle) per candidate child—**`K_init`** drafts.

4. **Within batch:** merge overlapping drafts or ask **“How is this different from …?”** until distinct **child** problems.

5. **Against each existing child:** if a draft matches an existing **`NN`**, stop and ask one differentiation question; cite **`problems["NN"]`**, **`./NN/problem.md`**, and that **child’s** **Summary**. Do not allocate until distinguished or merged.

6. Undifferentiable draft → **drop** or stop the run—never write an ambiguous **child** file. No **`## Differentiation`** in new child files.

Optional **ancestor** overlap (when a draft sounds like **`../problem.md`** rather than a new fork under **this** node): **[differentiation-extras.md](differentiation-extras.md)**—**Read** only when that applies; it is **not** the default differentiation target.

## Links in each new `problem.md`

Relative links. Order: YAML frontmatter (at least **`problems: {}`**), **`## Summary`**, **Navigation** ( **Deeper** to **`./<NN>/problem.md`** only once children exist—no sibling links in new files), optional body. Full before/after shape: **[examples.md](examples.md)**.

## Patch the input after creating children

For each new **`C/<NN>/problem.md`**, extend the input’s **`Deeper:`** (e.g. `./03/problem.md`) and **merge** **`problems`** with the new **`NN`** lines. If **`K = 0`**, do not patch.

## Workflow (one run)

1. Resolve **the input** (see **Input**—**How to supply it** / **Valid path**). If nothing resolves, stop using **If the problem file is missing** in **Input**. Read the file; if the path is invalid, stop.
2. Set **`C`** = directory of the input. Derive **`N`** from **`problems`**. Build sibling corpus ( **Differentiation** §1 ) and read needed files.
3. Send **User-facing chat**.
4. Analyze answer → **`K_init`**, draft summaries. If the user or project wants **`prompt.md`** logging, **Read [prompt-log.md](prompt-log.md)** then append as specified there.
5. Run **Differentiation** to final **`K`**. If **`K = 0`**, stop without new folders.
6. If **`K ≥ 1`**, re-read **`N`**, check **`N + K - 1 ≤ 99`**, ensure no **`C/<NN>/problem.md`** collision. **Must read [examples.md](examples.md)** once this run before the first **`problem.md`** create or input patch in step **7** (if not already read). Write new files.
7. Patch **the input**: **Deeper** + **`problems`**. If **`K = 0`**, skip.
8. Brief re-invoke hint (**`@`** skill + **`problem.md`** or path). Do not repeat the full spec. The input path must stay under **`.openpfe/problem/`**.

## Verification checklist (no duplicate rules)

**Authoritative detail** lives in **Input**, **Scope**, **Goal**, **Child registry**, **Differentiation**, **Links**, **Patch the input**, and **Workflow**—do not restate those sections here. After a run, confirm:

- **Input / scope:** Resolved per **Input**; reads and writes only under **`.openpfe/problem/`**; never empty **`NN/`** folders.
- **Ids:** Next **`NN`** from **Child registry** math on **the input** only—not from listing **`C/`**.
- **Differentiation:** Completed before writes; **`K = 0`** ⇒ no new child files and no input patch (**Patch the input**).
- **Artifacts:** New children follow **Links** and **Problem framing** (lean child, **`problems: {}`** in frontmatter); shape per **[examples.md](examples.md)** when read per **Workflow**.

## Companion files index

For a single place to link in chat: **[examples.md](examples.md)** (must read before first writes), **[prompt-log.md](prompt-log.md)** (logging), **[differentiation-extras.md](differentiation-extras.md)** (optional **ancestor** overlap). Details: **Companion files** at the top of **`SKILL.md`**.
