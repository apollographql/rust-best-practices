# Editing the agent guide

The public entry point is [../llms.txt](../llms.txt); chapters and examples live here. Keep the human handbook (`book/`, root `examples/`, Confluence inputs) separate.

- Audience: LLM agents fetching by URL. Dense agentese: established anchors + decision criteria + exceptions. Cut motivation and syntax lessons, not conditions; preserve legibility.
- Spend tokens where agent defaults drift. Anchor with established terms; keep coined phrases beside recognized equivalents when useful across model families.
- Name ecosystem delegation targets. Where alternatives matter, give 2–4 options with compact selection criteria; prefer the project's existing stack.
- One authoritative statement per decision: core summarizes, chapters own detail. Keep maintainer instructions here, outside the fetched core/router.
- Keep chapter paths stable and links current. Examples stay small, compiled, representative; link source rather than duplicating it in prose.
- Review guidance for literal-reading failures, consistency, links, and net compression. Report evidence and limitations; model self-evaluation isn't measured efficacy.
- For Rust examples, follow [verification](verification.md). Preserve meaningful property oracles and explicit lifecycle/ownership contracts.
