# PFE: Problem First Engineering

## 1. Introduction: The Problem-First Paradigm
**Problem First Engineering (PFE)** is a software engineering methodology designed for the AI era. It shifts the primary focus of development from "How do we build this?" (Solution-First) to "What exactly is the problem we are solving?" (Problem-First).

Traditional development often leads to **Context Bloat** and **Architectural Drift** because implementation starts before the problem space is fully mapped. PFE enforces a rigorous, iterative drill-down process where the problem model itself serves as the bedrock for all subsequent specifications and code.

---

## 2. Core Pillars of PFE

### A. Problem Decomposition (The Drill-Down)
Instead of a flat list of requirements, PFE treats the problem as a fractal tree.
- **Fishbone Analysis:** Using structured questioning to identify root causes and latent sub-problems.
- **Recursive Refinement:** Continuously asking "What am I missing?" until problems are "atomic"—small enough to be solved without further division.
- **Latent Surfacing:** Using AI to simulate edge cases and constraints at the problem level, before they become bugs in the code.

### B. Structural Synthesis (The Graph)
Once decomposed, problems are not stored in a list, but in a **Dependency Graph**.
- **Clustering:** Atomic problems are grouped into logical clusters based on domain proximity and state sharing.
- **Component Birth:** These clusters define the boundaries of a **Component**. Architecture emerges from the problem clusters rather than being imposed from the top down.
- **The Graph Model:** The graph tracks how solving Problem A is a prerequisite for addressing Problem B.

### C. Consumer-Driven Contracts (CDC)
The edges between clusters in the graph represent the **Interfaces**.
- Every dependency is governed by a contract defined by the "Consumer" (the component that has the problem) and fulfilled by the "Provider" (the component that solves it).
- These contracts act as a **Context Shield**, allowing components to be developed in isolation.

---

## 3. The PFE Workflow

1. **Problem Statement:** Define the high-level "Master Intent."
2. **Decomposition:** Drill down into sub-problems using a tree structure.
3. **Architecture Mapping:** Cluster the tree nodes into a Dependency Graph.
4. **Contract Definition:** Establish the interfaces between components.
5. **C-SDD (Component Spec-Driven Development):** Use the component spec to implement a "Version 0.0.1" spike.

---

## 4. Rationale and Benefits

- **Minimal Token Burn:** By decomposing problems into small clusters, AI agents only need the context of a single component, not the entire project.
- **Context Bloat Prevention:** The "Context Shield" ensures that implementation remains local and focused.
- **Traceability:** Every line of code can be traced back through a Spec to a specific Problem Node in the graph.
- **Greenfield Integrity:** Ensures that new projects start with a solid foundation (bedrock) rather than a pile of loosely coupled features.
