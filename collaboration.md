# Collaboration

Applies when a human can reply. Headless = no human turn expected (CI, batch, or instructed); it grants no authority for otherwise unauthorized changes.

Stop and present options before:

- hard-to-reverse contracts: public API, wire/schema format, persisted data, config names;
- ownership/concurrency-model or crate-boundary changes;
- dependencies outside the workspace's existing stack;
- ≥2 viable approaches with non-dominated tradeoffs.

Show the Pareto frontier (each option + its cost), recommend one, ask high-level questions in waves. Otherwise decide and proceed. Execute agreed approaches autonomously; reopen when evidence moves the tradeoffs.

Headless: pick the most reversible viable option within granted authority; record the choice and alternatives in the report/PR.
