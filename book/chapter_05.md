# Chapter 5 — Testing

**Test contracts, not implementation echoes.** Public behavior: readiness, effects, failure, teardown. Unit → pure policy; integration (`tests/`) → boundaries; doctests → usable API. One coherent behavior per test; setup and expectation visible; shared domain fixtures fine, buried action not.

**A failing test is evidence.** Don't weaken assertions, loosen tolerances, delete, `#[ignore]`, or accept snapshots to hide a regression; fix the code or report the conflict. Intended contract changes update tests with the reason stated. A new test must fail without the fix; `cargo mutants` checks that tests notice changes.

## Properties and oracles

Default to `proptest` when a law or independent reference exists:

- **Round trip:** generate domain values; `decode(encode(v)) == v` through the real pipeline ([example](../examples/parse-dont-validate/src/main.rs)). Define equivalence; bytes → parse → bytes canonicalizes rather than preserves. Round trip ≠ compatibility: persisted or public formats need golden fixtures from past versions.
- **Differential:** slow, obviously correct, independent reference vs the optimized path; share no production assumptions.
- **State machine:** `proptest-state-machine`: valid action sequences against a reference model; check transitions, conservation, eventual cleanup.
- **Metamorphic:** idempotence, commutativity, invariance under transformation; state why the law holds before encoding it.

Generators construct valid values (`prop_compose!`, `prop_oneof!`, `Arbitrary`) rather than filter (`prop_assume!`); keep structural and boundary variation; commit `proptest-regressions/`. Case count can't rescue a weak oracle. Parsers of untrusted bytes: add `cargo fuzz`.

## Async, concurrency, protocols

- Synchronize via readiness signals, channels, `Barrier`/`Notify`, or controlled time (`#[tokio::test(start_paused = true)]`, `time::advance`). Sleep isn't ordering; timeouts bound hangs, they don't prove completion.
- Assert a resource exists before asserting its release. Join tasks and await teardown so panics surface.
- Atomics/lock-free: `loom` for interleavings, `miri` for UB.
- Drive real lifecycles (`tower::ServiceExt::oneshot`, a bound test server) instead of calling handlers around negotiation. Established protocol impls in fixtures (`wiremock`); hand-rolled parsers are debt in tests too. Assert observable effects and safety guards, not callback counts.

## Snapshots and selection

- `insta` for large, stable structured output; redact only irrelevant nondeterminism; review snapshot diffs as assertions. Direct asserts for small values and semantic properties.
- Select by risk: success, rejected input, partial failure, cancellation, compatibility. Test seams follow external effects and controllability: time, randomness, network. Prefer a concrete fake, closure, explicit input, or in-process impl; a trait must still earn its complexity. Perf claims need measurement ([performance](chapter_03.md)).
