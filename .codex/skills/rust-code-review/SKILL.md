---
name: rust-code-review
description: "Review Rust changes for domain invariants, ownership, lifecycle, and behavioral regressions using this repository's agent guide."
---

# Rust Code Review

Read the repository's [compact core](../../../llms.txt), then only relevant chapters. Use the local checkout as authority for project constraints and supported dependencies.

Trace changed behavior from input boundary through domain decisions, effects, and teardown. Check whether types carry the claimed invariants, who can mutate the data, and whether error/recovery paths preserve a valid state. Evaluate ecosystem usage and tests against actual dependency lifecycles.

Report actionable findings with a concrete trigger, consequence, and source location. Prioritize by impact and likelihood, not a fixed mapping from syntax to severity. Distinguish demonstrated defects from design alternatives; do not frame an unmeasured clone or stylistic preference as a performance regression.

Review the tests' oracle, setup, and observations: could they pass without exercising the claimed behavior? Consult [testing](../../../book/chapter_05.md) and [verification](../../../book/chapter_02.md). Report checks run and remaining uncertainty; green tools do not prove domain correctness. Avoid repeating mechanical diagnostics already supplied by tooling.

Recommend the narrowest structural fix for a demonstrated bug class. Seek steering before redesigning domain or ownership boundaries; review alone does not authorize implementation.
