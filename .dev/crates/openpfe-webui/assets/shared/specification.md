# Shared specification

Entity states and cross-view rules. Per-view behaviour: view folders under [../README.md](../README.md).

---

## Entity states

### Problem

| State | Definition | Visible in |
|-------|------------|------------|
| **Unhandled** | Active in discovery/refinement | [Problem](../problem/) default list; [Dashboard](../dashboard/) open count |
| **Handled** | No longer active in Problem view | Dashboard handled count; excluded from Problem default list |

### Specification

| State | Definition | Visible in |
|-------|------------|------------|
| **Unhandled** | Not approved | [Specification](../specification/) primary queue |
| **Approved / handled** | Review complete | Dashboard; optional history in Specification view |

**Rule:** Approving an unhandled specification **must** create one or more **tasks** (server-enforced; UI confirms).

### Task

| State | Definition | Visible in |
|-------|------------|------------|
| **Unhandled** | Not completed | [Task](../task/) default list; Dashboard open count |
| **Completed** | Done | Dashboard completed count |

### Problem domain

| Aspect | Definition |
|--------|------------|
| **Membership** | Each problem belongs to at most one domain in v1 |
| **Dependencies** | Directed relationships between domains; cycles surfaced as errors when detected |
| **Contracts** | Consumer-driven agreement at a domain boundary |

Concepts: [concepts.md](./concepts.md).

---

## Cross-view rules

| Rule | Description |
|------|-------------|
| **C1** | Handled problems do not appear in Problem view default list |
| **C2** | Only unhandled specifications appear in Specification default queue |
| **C3** | Approval is the only UI path that creates tasks from specifications (v1) |
| **C4** | Architecture view is the intended entry for “specification for domain” |
| **C5** | Dashboard numbers must stay consistent with list filters in other views |

---

## API boundary (informative)

View docs define **what** the UI shows and does. **`openpfe-ui`** defines **how** (REST, polling). Map each view’s specification to resources; keep business rules on the server.
