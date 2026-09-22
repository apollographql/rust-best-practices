# Chapter 4 — Errors

**Errors are domain contract.** Callers branch on variants → typed enum via `thiserror`. Application reporting boundary → `anyhow` with `.context()`/`.with_context()`. Choose per caller need, not one enum per module by reflex. No `String`/`Box<dyn Error>` library errors.

- Preserve `source()` chains. Context names the failed operation and safe identifiers: no secrets, no flattening to strings. Log or return, not both; logging at every layer duplicates reports.
- `#[from]` only when the meaning holds at every `?` site. The same `io::Error` can mean different things at different sites; map explicitly.
- **Absence ≠ failure.** `Option` when absence is valid; `.ok_or(…)` where the domain requires presence. `.ok()`, `unwrap_or_default()`, `let _ =`, skips, and retries are policy decisions, never propagation shortcuts.
- **Recovery restores a valid state.** Define what committed, what's retryable, who cleans up. Retry ≠ rollback: retrying a partial effect duplicates it (require idempotency keys, deduplication, or reconciliation), and cancellation doesn't undo committed work. Cancellation and task panics (`JoinError`) are error paths too.
- **Panics are for broken invariants**, not malformed input or unavailable resources. Panic paths include `unwrap`, `expect`, `todo!`, `unimplemented!`, `unreachable!`, slice indexing. Prefer a type that removes the impossible branch over an assertion. `#[must_use]` on values callers must not drop. Tests: `expect("<invariant>")`.
- Test variants and observable recovery (`assert!(matches!(err, E::X { .. }))`). Assert message text only when wording is contract.
