# AGENTS

This repository follows the guidance from [Rust Best Practices](https://github.com/apollographql/rust-best-practices)

## Scope

- Applies to all Rust files matching `**/*.rs`.
- Focus on correctness, intent clarity, and maintainability first, with performance changes based on evidence.
- This guidance applies to all Rust files matching `**/*.rs`.

## Core rules

- Never "clean up", replace, or normalize in-progress user experiments unless explicitly asked.
- Never modify git/jj history.
- Never modify anything outside this folder.
- Favor simplicity. Write the least complex code that solves the problem — clarity beats cleverness.
  - Prefer simple code over elegant code; prefer elegant code over complex code.
- Rust style should follow a functional-first approach (immutability, pure helpers, composition over shared mutable state).
- When user asks for explicit code change there is ABSOLUTELY no need to add comment.
- Keep transform chains readable and line-broken.
- Always verify jj status before starting work, if unavailable, use `git status --short --branch`. Ensure workspace not stale.
- Always review the current branch before merge or commit decisions, focusing on behavior regressions, performance risks, and test coverage gaps.
- Minimum Rust quality gates (must remain covered or be run explicitly):
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-features`
- Avoid magic strings. Extract string literals used as identifiers or keys into named constants defined at the top of the file or in a dedicated constants module.
- Prefer explicitness over magic

## Core Rust coding rules

- Default to borrowing (`&T` / `&mut T`) and avoid cloning unless ownership is required.
- Prefer small `Copy` types by value; pass heap-backed or large types by reference.
- Use explicit ownership in public APIs (`&str` / `&[T]` preferred over owned inputs unless needed).
- Prefer `?` for fallible flow and use `let PATTERN = EXPR else { ... }` or
  `if let ... else` for structured early-exit patterns.
- Prefer `if`/`else if`/`else` over `if` with early `return`.
- Use `_or_else` / lazy defaults and avoid eager allocation when not needed.
- Prefer iterator pipelines for pure transformations; use `for` loops when break/continue/early return or complex side effects are required.
- Keep import groups ordered: `std/core/alloc`, external crates, workspace, `super::`, `crate::`.
- Prefer references before pointers; use `Arc`/`Rc` only when ownership model requires it.
- Treat raw pointer/unsafe usage as bounded API surfaces with explicit invariants.
- Prefer `From`/`Into` trait conversions over manual bit-transform conversion implementations.
- Keep side effects at system boundaries (I/O, engine/FFI calls), and keep transformation logic side-effect free.
- Use `Cell`/`RefCell` only when borrow rules must be relaxed.

## Error and API quality

- Libraries should use typed errors (`thiserror`) with clear boundary conversions.
- Binaries may use `anyhow`, but maintain informative context.
- Avoid silent recovery from `Err`; keep root-cause visibility where feasible.
- Public items should be documented with `///`, including behavior, parameters, returns, and error/panic/safety notes.
- Keep comments purposeful (`why`, assumptions, constraints, safety); remove stale comments.
- Convert TODOs to issue-linked notes like `// TODO(issue #NNN): ...`.
- Add `///` doc notes for module-level behavior with `//!` when needed.

## Panic/Unwrap Policy

- Restrict `unwrap` / `expect` to tests or impossible-by-design cases with rationale.
- Ccode must never introduce new `panic!`, `unwrap`, or `expect` unless explicitly approved.
- Runtime fallible paths must return `Result` with meaningful error context instead of aborting, unless explicitly required by user.
- `unwrap`/`expect` are allowed in tests and short-lived prototype code only; `expect` messages must explain the invariant.
- Prototype-only panics/unwraps must be removed or isolated behind non-shipping cfg/feature gates before merge.
- Keep `expect` messages tied to explicit invariants when they are unavoidable.

## Testing and review

- One behavior per test, with descriptive names.
- Cover success and error behavior for behavioral paths.
- Add integration tests for public behavior and unit tests for internals when appropriate.
- Use snapshot tests only for complex, stable structured outputs.
- Required pre-merge quality gates:
  - Intentional ownership/data-flow review
  - Explicit and aligned error handling
  - `cargo fmt --all --check`
  - `cargo clippy --all-targets --all-features --locked -- -D warnings`
  - `cargo test --workspace --all-features`
  - Measured performance claims before acceptance
  - Documentation reflects runtime behavior
- Avoid unnecessary `#[allow]` and prefer explicit local `#[expect(...)]` only when justified.
- High-risk review triggers:
  - unnecessary cloning in hot paths
  - unjustified `#[allow]`/suppression
  - missing context on errors
  - copying large values by value without justification
  - TODOs without ownership/context

## Dispatch strategy

- Prefer static dispatch (`impl Trait` / generic bounds) when types are known and performance matters.
- Use dynamic dispatch (`dyn Trait`) for plugin-style/runtime polymorphism needs.
- Delay boxing until required by design boundaries.

## Type-state

- Use type-state patterns when illegal states can be prevented at compile time.
- Skip type-state when runtime checks are clearer for simple finite-state behavior.

## Reference

- Default review workflow references: [Rust Best Practices](https://github.com/apollographql/rust-best-practices) standards and
  `.codex/skills/rust-code-review/agents/openai.yaml`.
