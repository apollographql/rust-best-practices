# Performance

**Measure the workload, not the intuition.** Name the objective (latency p50/p99, throughput, peak/retained memory, startup, build time, binary size) and a baseline. Profile optimized builds with symbols on representative input; report variance and environment. No universal worth-it threshold.

Tools, if the project has none: `criterion`/`divan` benches; `samply`/`cargo flamegraph`/`perf` CPU profiles; `dhat`/`heaptrack` allocations; `tokio-console` async stalls; `cargo bloat`/`cargo llvm-lines` size and compile time.

Amdahl's law: unchanged work limits speedup. Profile first; algorithm/data flow before micro-optimization: repeated work, materialization, clones, serialization, contention, queues. Guard complex changes with a differential oracle ([testing](testing.md)).

- Ownership isn't allocation. Moving `String`/`Vec` is O(1); borrowing can prolong retention; `Arc` swaps deep copies for refcount traffic and lifetime coupling. Weigh allocation count, retained memory, locality, contention together.
- The largest payload drives enum size; discriminant, alignment, padding, and niches adjust it. Measure (`size_of`, `-Zprint-type-sizes`); box a rare large variant (`clippy::large_enum_variant`) when size and usage justify the indirection.
- Reuse buffers (`with_capacity`, `clear()` + refill) and stream when the contract permits; materialize when reuse, ownership, or locality wins.
- Monomorphization trades runtime for code size and compile time ([dispatch](dispatch.md)).

Re-measure end-to-end across objectives, not just the optimized metric. Keep the simplest version meeting the targets.
