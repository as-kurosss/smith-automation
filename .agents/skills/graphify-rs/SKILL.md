---
name: graphify-rs
description: Use this skill when working with the smith-automation codebase to analyze architecture, find dependencies between modules, locate implementations, understand component relationships, or answer questions about project structure via the graphify-rs knowledge graph. Trigger this skill whenever the user asks about architecture, module dependencies, how components connect, where something is implemented, what uses a particular type or trait, or wants to query the project knowledge graph.
---

# graphify-rs: Knowledge Graph Skill (works alongside ast-index)

## Description

graphify-rs is a CLI tool for building and analyzing a knowledge graph of a codebase. It analyzes the code's AST, documentation, and project files, builds a graph of nodes (modules, functions, types, files) and edges between them, and then allows answering questions about the project architecture through semantic graph search.

The project also uses **ast-index** for fast structural search. The two tools complement each other.

## When to use graphify-rs vs ast-index

### graphify-rs (architecture and relationships)
- Understand project architecture, relationships between modules, communities
- Find which components depend on a specific module/type/function
- Get a high-level view of code structure and data flow between crates
- Questions: "how is X structured?", "what does Y do?", "how are A and B connected?"

### ast-index (fast structural search) — use BEFORE graphify-rs when:
- Find a symbol/function/type definition (st-index symbol, st-index class)
- Find all usages of a type/trait (st-index usages, st-index implementations)
- Who calls a function (st-index callers, st-index call-tree)
- File structure before reading (st-index outline)
- File search (st-index file)

### Hierarchy:
1. **ast-index** — first for symbol, file, and usage search
2. **graphify-rs** — if architectural insight is needed or answer not found via ast-index
3. **grep** — only if both returned nothing or regex/string pattern search is needed

## How to use

### 1. Ensure the graph is up to date

Before using, check that the graph is generated:

`ash
ls -la ./smith-graphify/graph.json
`

If the project has changed significantly (new crates, files, tools added), rebuild the graph:

`ash
graphify-rs build --no-llm --output ./smith-graphify
`

### 2. Query the graph

`ash
graphify-rs query --graph ./smith-graphify/graph.json "<your question>"
`

### 3. Slash command

If the user invokes the skill via /graphify-rs <query>, execute:

`ash
graphify-rs query --graph ./smith-graphify/graph.json "\"
`

### 4. Example questions

`ash
# General architecture questions
graphify-rs query --graph ./smith-graphify/graph.json "what crates exist in the project and how are they connected?"
graphify-rs query --graph ./smith-graphify/graph.json "what is ToolRegistry and what is it connected to?"

# Finding specific components
graphify-rs query --graph ./smith-graphify/graph.json "which Windows tools are implemented?"
graphify-rs query --graph ./smith-graphify/graph.json "how does ExecutionContext and its methods work?"

# Understanding dependencies
graphify-rs query --graph ./smith-graphify/graph.json "which modules depend on smith-core?"
graphify-rs query --graph ./smith-graphify/graph.json "what does SafeUIElement use?"

# Analyzing new components
graphify-rs query --graph ./smith-graphify/graph.json "how is smith-daemon and its API structured?"
graphify-rs query --graph ./smith-graphify/graph.json "how does ProcessTool start and stop processes?"

# Searching by specific files
graphify-rs query --graph ./smith-graphify/graph.json "what functions are defined in selector.rs?"
`

## Graph artifacts

After building the graph, the following files are available in smith-graphify/:

| File | Description |
|------|-------------|
| graph.json | Graph in JSON format (nodes, edges, communities) |
| GRAPH_REPORT.md | Analytical report with metrics, connections, communities |
| graph.html | Interactive graph visualization |
| graph.svg | Static graph visualization |
| wiki/ | Wiki pages for communities and key entities |
| obsidian/ | Obsidian-compatible markdown pages |

## Notes

- Graph queries do not modify project files — they are read-only operations
- If the graph is stale (does not reflect recent changes), rebuild it before querying
- For simple questions (file structure, function names), regular code search is sufficient — use the graph for complex architectural questions and relationship discovery
