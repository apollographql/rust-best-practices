# Chapter 8 — Documentation

**Document the contract and the missing why.**

- Public items: behavior, invariants, and rustdoc sections `# Errors`, `# Panics`, `# Safety` (every `unsafe fn`), `# Examples` as compiled doctests. Add ownership, cancel-safety, and partial-effect semantics when they affect correct use. Enforce via `missing_docs`, `clippy::missing_errors_doc`/`missing_panics_doc` where the project enables them.
- Comments carry non-obvious decisions, external constraints, and why the obvious alternative is wrong. Don't narrate code or edits ("now uses X", "changed to Y"). A good name can replace a procedural comment; extraction still has to earn its indirection.
- `unsafe`: each block gets `// SAFETY:` arguing why this operation's preconditions hold: provenance, lifetime, alignment, initialization, aliasing, bounds, synchronization as relevant (`clippy::undocumented_unsafe_blocks`). "Pointer is non-null" is not an argument.
- One authoritative location per policy; link, don't copy. Update comments when their code's behavior changes. `TODO`s carry an issue reference and the blocking constraint.
- **Configuration is public API.** Names, nesting, defaults, env-var paths follow domain concepts, not plumbing. Document compatibility; renames need aliases (`#[serde(alias)]`) or a migration path.
