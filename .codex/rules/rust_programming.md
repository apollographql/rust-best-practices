---
description: "Rust programming rules derived from book best practices for consistent review and implementation."
globs:
  - "**/*.rs"
alwaysApply: true
---

# Codex Programming Rules

Apply these rules when writing Rust in this repo.

## Rule 1: Borrow by default

- Default to borrowing (`&T`, `&mut T`) and avoid cloning.
- Use owned inputs only when semantics or API require it.
- Public APIs should prefer `&str` / `&[T]` over `String` / `Vec<T>` unless ownership is needed.
- Prefer `if`/`else if`/`else` over early-return-heavy control flow when that improves readability.

## Rule 2: Copy/value semantics intentionally

- Pass small `Copy` types by value.
- Avoid passing large structs/heap-backed values by value in public APIs.
- Do not mark large complex types as `Copy`.

## Rule 3: Option/Result control flow

- Prefer `let PATTERN = EXPR else { ... }` for early expected exits.
- Use `if let ... else` when extra logic is required in both branches.
- Use `?` for normal propagation.
- Keep `unwrap`/`expect` restricted to tests or impossibility proofs.
- Avoid silent recovery from `Err`; preserve root-cause visibility where recovery is needed.
- In production fallible paths, return `Result` with meaningful context instead of aborting.
- If `expect` is used, include invariant-focused messages.

## Rule 4: Prevent eager allocation

- Use `_else` methods for costly defaults.
- Defer conversions and allocations until consumed.
- Pass iterators through pipelines instead of collecting then reprocessing.
- Use `_or_else` to delay expensive default creation.

## Rule 5: Iterator and loop policy

- Prefer iterator chains for pure transformations.
- Use `for` when requiring `break`, `continue`, `return`, or complex side effects.
- Keep transform chains readable and line-broken.

## Rule 6: Imports and style

- Keep import grouping: std/core/alloc, external crates, workspace, `super::`, `crate::`.
- Use stable rustfmt ordering where available.
- Keep side effects at system boundaries; keep transformation logic side-effect free.
- Keep transform chains readable and line-broken.
- Prefer functional composition for pure logic; use mutable/shared state only at boundaries.

## Rule 7: Clippy-first workflow

- Use: `cargo clippy --all-targets --all-feature --locked -- -D warnings`.
- Treat clippy warnings as actionable.
- Avoid blanket suppression; if necessary, prefer documented local `#[expect(...)]`.

## Rule 8: Performance only after measurement

- Don't optimize by intuition.
- Measure with release builds, profiling, and benchmarks.
- Prefer measured alternatives when optimization claim is made.
- Performance changes should be backed by evidence.

## Rule 9: Error strategy

- Library crates: prefer typed errors (`thiserror`) and crate-level variants.
- Binaries: `anyhow` acceptable for ergonomics, with consistent context.
- Preserve error context where callers need it.
- Avoid silent recovery from `Err` unless intentionally handled.
- Prefer explicit boundary conversions for typed errors.

## Rule 10: Public API docs

- Document public items with `///` and include:
  - what it does,
  - parameters and outputs,
  - errors, panics, and safety notes when relevant,
  - examples.
- Use `//!` for module/crate-level purpose.
- Keep comments for intent, assumptions, constraints, or safety rationale; remove stale comments.

## Rule 11: Testing quality

- One test should validate one behavior.
- Use descriptive sentence-like names.
- Use both unit and integration coverage for behavioral and contract verification.
- Use snapshot testing for complex structured outputs only.
- Required quality gates:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
- Pre-merge checklist:
  - intentional ownership/data-flow review
  - explicit and aligned error handling
  - measured performance claim verification
  - documentation reflects runtime behavior

## Rule 12: Dispatch choice

- Prefer static dispatch (`impl Trait` / generic bounds) when types are known and performance matters.
- Use dynamic dispatch (`dyn Trait`) for plugin-style/runtime polymorphism needs.
- Delay boxing until required by design boundaries.

## Rule 13: Type-state when valuable

- Use type-state when illegal states should be compile-time errors.
- Skip type-state for simple enums/flags where runtime checks are clearer.

## Rule 14: Comments and TODO policy

- Keep comments for intent, constraints, safety, or architecture rationale.
- Remove stale comments and avoid obvious-restatement comments.
- Convert TODOs into issue references: `// TODO(issue #NNN): ...`
- Do not add comments for explicit code changes unless required for intent or safety.
- Avoid unjustified `#[allow]`; prefer local `#[expect(...)]` only when explicitly needed.

## Rule 15: Pointer and concurrency model

- Prefer references over pointers.
- Use `Arc`/`Rc` only by threading requirement.
- Use `Cell`/`RefCell` only when borrow rules must be relaxed.
- Treat raw pointer usage as unsafe API surface requiring explicit invariants.
- Default to borrowing (`&T`, `&mut T`) and avoid cloning unless ownership is required.
- Use small `Copy` types by value and pass heap-backed or large types by reference.
- Prefer `From`/`Into` over manual bit-level conversions.

## Rule 16: Workflow and governance

- Never "clean up", normalize, or replace in-progress experiments unless explicitly asked.
- Never modify git/jj history.
- Never modify anything outside this repo folder.
- Verify `jj status` before starting work; if unavailable, use `git status --short --branch`.
- Review the current branch before merge or commit decisions, focusing on behavior regressions, performance risk, and test coverage gaps.
- Prefer explicitness over magic; extract identifier/key strings into named constants.
- Add the following pre-merge review checks:
  - intentional ownership/data-flow review
  - explicit and aligned error handling
  - measured performance claim verification
  - documentation reflecting runtime behavior
- Keep code paths simple and explicit before adding clever abstractions.
- Never introduce new `panic!`, `unwrap`, or `expect` in shipping code unless explicitly approved.
- Never modify git/jj history.
- Never modify anything outside this folder.

Apply the same review references:
- https://github.com/apollographql/rust-best-practices
- `.codex/skills/rust-code-review/agents/openai.yaml`.

## Extra rules

- Follow the latest standards in https://github.com/apollographql/rust-best-practices
- Avoid:
  - unnecessary cloning in hot paths
  - unjustified `#[allow]`
  - missing context on errors
  - copying large values by value without justification
  - TODOs without ownership/context
