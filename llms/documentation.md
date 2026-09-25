# Documentation

**Document the contract and the missing why.**

- Public items: behavior, invariants, and rustdoc sections `# Errors`, `# Panics`, `# Safety` (every `unsafe fn`), `# Examples` as compiled doctests. Add ownership, cancel-safety, and partial-effect semantics when they affect correct use. Enforce via `missing_docs`, `clippy::missing_errors_doc`/`missing_panics_doc` where the project enables them.
- Comments preserve intent, constraints, and rejected alternatives; don't narrate code or edits. Names can replace procedural commentary; extraction must earn its indirection.
- `unsafe`: `// SAFETY:` argues this operation's proof obligations hold: provenance, lifetime, alignment, initialization, aliasing, bounds, synchronization as relevant (`clippy::undocumented_unsafe_blocks`). Non-null alone doesn't establish pointer validity.
- One authoritative location per policy; link, don't copy. Update comments when their code's behavior changes. `TODO`s carry an issue reference and the blocking constraint.
- **Configuration is public API.** Names, nesting, defaults, env-var paths follow domain concepts, not plumbing. Document compatibility; renames need aliases (`#[serde(alias)]`) or a migration path.
