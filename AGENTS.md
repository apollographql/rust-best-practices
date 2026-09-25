# Repository Instructions

This repository publishes a human handbook (`book/`, `examples/`, synced to Confluence) and compact agent guidance: [llms.txt](llms.txt) is the core and router into `llms/`. Keep the two separate; agent-guidance changes don't edit `book/`. Keep reusable Rust advice there or in the relevant chapter rather than duplicating it in tool-specific instructions.

## Working agreement

- Check `jj status` before work; if unavailable, use `git status --short --branch`. Preserve user experiments and unrelated changes. Do not modify git/jj history or files outside this repository.
- Before consequential domain, ownership, or architectural changes, investigate, recommend an approach with meaningful tradeoffs, and seek human steering. Execute an agreed approach autonomously.
- Review the final diff before commit/merge decisions, focusing on regressions, ownership, error handling, and evidence. Do not publish or message others without authorization.

## Editing guidance

- Audience is LLM agents fetching by URL. Write dense agentese: established anchors + decision criteria + exceptions; cut human padding, motivation, and syntax lessons, not conditions. Maximize value per token; never trade legibility for inscrutability.
- Anchor with established terms (e.g. parse don't validate, boolean blindness, TOCTOU, cancel safety). Keep a coined phrase beside its established term when other model families already use it.
- Spend tokens where agent defaults drift (see `Drift → instead` in llms.txt); name concrete tools and crates as delegation targets. One authoritative statement per decision; llms.txt summarizes, chapters own detail.
- Keep `llms/` paths stable; update internal links when headings change. Avoid duplicating example source in prose. Keep examples small, compiled, and representative of the stated invariant.
- Use established ecosystem crates for standard mechanics; inspect installed APIs first. Keep identifiers explicit and centralized when they represent shared policy.
- Follow configured formatting. Comments preserve non-obvious intent, contracts, and safety; do not narrate explicit code changes.
- Do not add production `panic!`, `unwrap`, or `expect` without explicit approval. Tests may use invariant-specific `expect`. Remove prototype panic paths before shipping.

## Verification

For Rust changes, run the [workspace quality gates](llms/verification.md): formatting, Clippy across all targets/features with warnings denied, and workspace tests. Review public contracts and measure performance claims. For guidance changes, check links, consistency, and net compression; report limitations honestly.
