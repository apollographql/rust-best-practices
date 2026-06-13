---
name: rust-code-review
description: "Rust Code review checklist and decision framework for Rust PRs, derived from rust-best-practices."
globs:
  - "**/*.rs"
alwaysApply: true
---

# Rust Code Review

Use this for Rust review work where consistency, safety, and maintainability matter.

Follow ths standards in https://github.com/apollographql/rust-best-practices

## Mandatory pre-merge checks

- Ownership and data flow are intentional.
- Error handling is explicit and aligns with crate/binary boundaries.
- Clippy and format quality are clean in touched files.
- Performance changes are measured before acceptance.
- Public APIs are documented; docs match runtime behavior.
- Tests cover intended behavior and error paths.
- Unsafe or raw-pointer usage is justified and constrained.

## Severity matrix

- P0: unsafe memory bug, panic in recoverable production path, silent data corruption.
- P1: correctness bug, missing error propagation, invalid API contract.
- P2: likely performance regression, missing public API docs, flaky tests.
- P3: style/readability issues, avoidable clone/allocation, unnecessary complexity.

## Review skills

### Ownership-first coding

- Prefer borrowing (`&T`, `&mut T`) over cloning.
- Use `Clone` only when ownership is required or snapshots are explicitly needed.
- Treat unnecessary clones (especially in loops) as likely regressions.
- Reject `clone` on `Copy` types.

### Value vs reference

- Pass `Copy`/small POD types by value.
- Pass large heap-backed or non-trivial objects by reference.
- Surface ownership intent in function signatures.
- Use `Cow<'_, T>` when input may be borrowed or owned.

### Fallible control flow

- Use `let PATTERN = EXPR else { ... }` for expected early exits.
- Use `if let ... else` when divergence needs additional logic.
- Prefer `?` for bubbling errors.
- Avoid `unwrap`/`expect` in production except when impossible-by-design cases are documented.

### Allocation and allocation timing

- Prefer `_else` APIs to avoid eager allocation (`ok_or_else`, `map_or_else`, etc.).
- Keep iterator chains lazy; allocate only when required by terminal ops.
- Do not collect and allocate only to throw away data.

### Iterator vs loop

- Use iterator chains for data transformation and composition.
- Use `for` for early exits and side-effect-heavy or control-heavy loops.
- Require readable formatting; avoid long unreadable chains.

### Lints and static checks

- Run and fix warnings from:
  - `cargo clippy --all-targets --all-feature --locked -- -D warnings`
- Do not globally silence useful lints.
- Prefer `#[expect(clippy::...)]` with rationale instead of `#[allow(...)]` unless fully justified.

### Error discipline

- Libraries: prefer typed errors (`thiserror` and `#[from]` conversions).
- Binaries: `anyhow` acceptable, but keep context rich and actionable.
- Test both success and error behavior.

### Tests as behavior docs

- One behavior per test.
- One core assertion per test where possible.
- Names should be descriptive sentence-like statements.
- Prefer unit tests for internals, integration tests for public behavior.
- Use snapshot tests only for complex, stable structured outputs.

### Documentation and comments

- Use `///`/`//!` for API behavior and constraints.
- Use `//` for `why`, safety rationale, platform constraints, and assumptions.
- Remove stale comments; prefer smaller functions over narrative comments.
- Link TODOs to issues instead of leaving bare `TODO:`.

### Pointers and concurrency

- Prefer `&`/`&mut` before any heap pointer.
- Use `Arc` for cross-thread shared ownership; `Rc` for single-threaded.
- Use `Box` for recursive/heap allocation needs.
- Review raw pointer usage as unsafe boundaries with explicit invariants.

## Quick rejection triggers

- Unnecessary clones in hot paths.
- Unjustified `allow(clippy::...)`.
- Silent recovery from `Err` that discards root cause.
- Copying large types by value without a proof of intent.
- Comments that simply restate what code already expresses.
- TODOs without ownership/context.

