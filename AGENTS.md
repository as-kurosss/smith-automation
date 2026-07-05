# Project Rules

## Tool Documentation

Specifications for implemented tools are located in `docs/design/<tool-name>/specification.md`:
- `docs/design/windows-process/specification.md` -- process management
- `docs/design/windows-find/specification.md` -- UI element search
- `docs/design/windows-click/specification.md` -- clicking an element

When generating a new tool, create a specification using the template `docs/templates/specification-template.md`.

## Code Search

Use a combination of tools for codebase navigation:

- **ast-index** (${\textsf{\color{green}installed}}$) — fast structural search for symbols, files, usages
- **graphify-rs** (${\textsf{\color{green}installed}}$) — architectural analysis: dependency graph, module relationships
- **codebase_semantic_search** (${\textsf{\color{orange}built into Oz}}$) — semantic search by meaning
- **grep** (${\textsf{\color{orange}built into Oz}}$) — regex and exact string search

**Usage hierarchy:**
1. `ast-index` — structural search (symbols, trait implementations, usages, callers)
2. `graphify-rs` — architectural analysis (module relationships, dependency graph, high-level questions)
3. `codebase_semantic_search` — when you don't know the exact name or are searching by meaning/concept (ast-index and graphify-rs cannot do this)
4. `grep` — if ast-index returns empty, or you need regex/string pattern search
5. Before reading a file >500 lines — first run `ast-index outline <file>`

### Maintaining the ast-index

```bash
# Check that the index is up to date
ast-index stats

# After git pull/checkout/switch
ast-index update
```

### Basic ast-index commands

**Search:**
- `ast-index search "<query>"` -- universal search
- `ast-index file "<pattern>"` -- file search
- `ast-index symbol "<name>"` -- find symbol definition
- `ast-index class "<name>"` -- find class/struct
- `ast-index outline <file>` -- file structure (before reading large files)

**Usages and calls:**
- `ast-index usages "<symbol>"` -- all usages of a symbol
- `ast-index callers "<function>"` -- who calls a function
- `ast-index refs "<symbol>"` -- cross-references
- `ast-index implementations "<trait>"` -- interface implementations
- `ast-index hierarchy "<class>"` -- hierarchy tree
- `ast-index call-tree "<function> -d 3"` -- call tree

**Modules:**
- `ast-index deps "<module>"` -- module dependencies
- `ast-index dependents "<module>"` -- who depends on a module

**Code quality:**
- `ast-index todo` -- all TODO/FIXME/HACK
- `ast-index deprecated` -- deprecated items
- `ast-index changed` -- what changed in the current branch

### Examples for smith-automation

```bash
# Find all implementations of the Tool trait
ast-index implementations "Tool"

# Where ExecutionContext is used
ast-index usages "ExecutionContext"

# Who calls execute
ast-index callers "execute"

# File structure before reading
ast-index outline crates/smith-core/src/registry.rs

# Module dependencies
ast-index deps "smith-windows"
```

### Rules for sub-agents

When launching a sub-agent for code search, pass these instructions:
```
Search hierarchy (use in this order):

1. Structural search:
   ast-index search "query"           -- universal search
   ast-index file "Name"              -- find a file
   ast-index symbol "Name"            -- find a symbol definition
   ast-index usages "Name"            -- all usages of a symbol
   ast-index implementations "Trait"  -- implementations
   ast-index callers "func"           -- who calls this function
   ast-index outline <file>            -- file structure before reading

2. Architecture queries:
   graphify-rs query --graph <path>   -- knowledge graph questions

3. Semantic search (when you don't know exact names):
   codebase_semantic_search           -- search by concept/meaning

4. Regex/string search (if ast-index returns empty):
   grep "<pattern>"                    -- regex/string search
```

## graphify-rs

Use the `graphify-rs` utility to build a knowledge graph of the project.

### Graph generation

Command:
```bash
graphify-rs build --no-llm --output ./smith-graphify
```

### Artifacts

- `smith-graphify/graph.json` — graph in JSON format (nodes, edges, communities)
- `smith-graphify/GRAPH_REPORT.md` — analytical report on the graph

### Integration with smith-context

The `smith-context` utility automatically loads graphify-rs artifacts with the `--graphify` flag:
```bash
cargo run -p smith-context -- --graphify
```

### Graph queries

Ask questions about the project architecture via:
```bash
graphify-rs query --graph ./smith-graphify/graph.json "<query>"
```

Examples:
```bash
graphify-rs query --graph ./smith-graphify/graph.json "how does auth work?"
graphify-rs query --graph ./smith-graphify/graph.json "which modules depend on smith-core?"
graphify-rs query --graph ./smith-graphify/graph.json "what does ExecutionContext do?"
```

### Installation

If `graphify-rs` is not installed:
```bash
cargo install graphify-rs
```

## Excluded from context

The `apps/smith-context` directory is excluded from context. AI agents and code analysis tools are prohibited from reading, analyzing, or modifying files in `apps/smith-context`. This package is an independent utility for context gathering and is not part of the core automation codebase.

## Language

All code, comments, documentation, commit messages, specifications, and other project artifacts MUST be written in English. This ensures consistency and accessibility for all contributors.

## Git commits

Do not add `Co-Authored-By` to commit messages. Attribution is not required.
