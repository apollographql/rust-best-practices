# Errors

**Errors are domain contract.** Callers branch → typed enum (`thiserror`; `snafu` for context selectors). Application reports → `anyhow` or configurable `eyre`; attach operation context. Choose per caller, not per module. No `String`/`Box<dyn Error>` library errors.

- Preserve `source()` chains. Context names the failed operation and safe identifiers: no secrets, no flattening to strings. Log or return, not both; logging at every layer duplicates reports.
- `#[from]` only when the meaning holds at every `?` site. The same `io::Error` can mean different things at different sites; map explicitly.
- **Absence ≠ failure.** `Option` when absence is valid; `.ok_or(…)` where the domain requires presence. `.ok()`, `unwrap_or_default()`, `let _ =`, skips, and retries are policy decisions, never propagation shortcuts.
- **Exception safety:** failure leaves state unchanged or valid for stated recovery; say which. Identify committed effects and cleanup owners. Retry ≠ rollback: partial effects may duplicate; use idempotency, deduplication, or reconciliation as needed. Cancellation and task panics (`JoinError`) are failure paths too.
- **Panics are for broken invariants**, not malformed input or unavailable resources. Panic paths include `unwrap`, `expect`, `todo!`, `unimplemented!`, `unreachable!`, slice indexing. Prefer a type that removes the impossible branch over an assertion. `#[must_use]` on values callers must not drop. Tests: `expect("<invariant>")`.
- Test variants and observable recovery (`assert!(matches!(err, E::X { .. }))`). Assert message text only when wording is contract.
