# openpfe: The Problem First Engineering Suite

## 1. Overview
**openpfe** is the reference implementation of the Problem First Engineering methodology. It is a service and toolset designed to facilitate the transition from raw problem statements to structured, component-based implementations using AI assistance.

The primary goal of **openpfe** is to maintain the "Source of Truth" in a Problem Graph, preventing context bloat and enabling precise, agentic development.

---

## 2. System Architecture

### A. The Graph Engine (The Brain)
At the heart of **openpfe** is a Graph Database that stores:
- **Problem Nodes:** The atomic units of the PFE drill-down.
- **Dependency Edges:** The links between problems and the contracts that govern them.
- **Component Clusters:** Logical groupings of problems that define the software architecture.

### B. The WebUI (The Canvas)
An interactive interface where the human engineer and the AI collaborate:
- **Drill-Down View:** Expand problem nodes into trees using "Fishbone" logic.
- **Architecture View:** Drag-and-drop nodes to form clusters and define component boundaries.
- **Contract Editor:** Review and approve Consumer-Driven Contracts.

### C. The MCP Server (The Context Bridge)
**openpfe** provides a **Model Context Protocol (MCP)** server that allows external AI agents (in IDEs like Cursor or Claude) to:
- Query the graph for relevant component context.
- Fetch only the necessary Specs and Contracts for a specific task.
- Ensure that the agent never "sees" more code than is required for the current component (The Context Shield).

---

## 3. Key Features

### Agent Orchestration (C-SDD)
**openpfe** manages the lifecycle of an implementation task:
1. **Selection:** Choose a component cluster for a "Version 0.0.1" spike.
2. **Branching:** Automatically create a Git branch for the spike.
3. **Execution:** Feed the Component Spec to an agent via MCP.
4. **Validation:** Review the implementation against the original problem nodes.

### Automated Refinement
Using "Skills" (pre-defined AI instruction sets), **openpfe** can:
- Suggest sub-problems you might have missed.
- Identify circular dependencies in the graph.
- Draft boilerplate contracts based on the cluster's problem statements.

---

## 4. Usage Flow
1. **Initialize:** `openpfe init project-name`
2. **Discovery:** Use the WebUI to map the problem space.
3. **Define:** Group nodes into components and approve contracts.
4. **Implement:** Use the MCP server to point your AI agent at a specific component for implementation.
5. **Iterate:** If the spike reveals a flaw, update the graph in **openpfe** and re-generate.
