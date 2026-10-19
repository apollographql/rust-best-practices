# Testing

**Test contracts, not implementation echoes.** Readiness, effects, failure, teardown. Unit → pure policy; integration (`tests/`) → boundaries; doctests → usable API. One coherent behavior; visible setup/action/expectation; shared domain fixtures, not buried actions.

**A failing test is evidence.** Don't weaken assertions, loosen tolerances, delete, `#[ignore]`, or accept snapshots to hide a regression; fix or report. Intended contract changes update tests with the reason. Regression test: fails without the fix. Mutation testing (`cargo mutants`): does the suite detect changed behavior?

## Properties and oracles

Property tools: `proptest` for controlled strategies/shrinking; `quickcheck` for type-driven generation. Test laws or independent references:

- **Round-trip laws:** `decode(encode(v)) == v` for generated domain values through the real pipeline. Define equivalence; parse → emit may canonicalize, not preserve bytes. Round trip ≠ compatibility: public/persisted formats need past-version golden fixtures.
- **Differential testing:** optimized/custom code vs a slow, simple, independent reference. Oracle independence: share contract, not implementation logic or unverified assumptions. Shared-parser round trips aren't independent validation.
- **Model-based testing:** `proptest-state-machine`: action sequences vs a reference model; check transitions, conservation, rejected actions, eventual cleanup.
- **Metamorphic:** idempotence, commutativity, invariance under transformation; state why the law holds before encoding it.

Generators: construct valid values (e.g. `prop_compose!`, `prop_oneof!`) over heavy filtering (`prop_assume!`); vary boundaries, preserve shrinking; commit `proptest-regressions/`. Case count can't rescue a weak oracle. Untrusted-byte parsers: `cargo fuzz`.

## Async, concurrency, protocols

- Synchronize via readiness signals, channels, `Barrier`/`Notify`, or controlled time (`#[tokio::test(start_paused = true)]`, `time::advance`). Sleep isn't ordering; timeouts bound hangs, they don't prove completion.
- Prevent vacuous success: assert creation before release. Join tasks and await teardown so panics surface.
- Atomics/lock-free: `loom` for interleavings, `miri` for UB.
- Drive real lifecycles (`tower::ServiceExt::oneshot`, bound test server), not handlers bypassing negotiation. HTTP fixtures: `wiremock` or `httpmock` servers over hand-rolled protocol. Assert effects and safety guards, not callback counts.

## Snapshots and selection

- Snapshots: `insta` for structures, `snapbox` for CLI/files; redact irrelevant nondeterminism, review diffs as assertions. Direct asserts for small values/properties.
- Select by risk: success, rejected input, partial failure, cancellation, compatibility. Test seams follow external effects and controllability: time, randomness, network. Prefer a concrete fake, closure, explicit input, or in-process impl; a trait must still earn its complexity. Perf claims need measurement ([performance](performance.md)).
