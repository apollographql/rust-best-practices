# Chapter 9 — Resources and Concurrency

## Ownership before synchronization

- Name the owner of every resource and task; distinguish process, connection, and request lifetimes. Borrow for access; `Box` for owned indirection; `Rc`/`Arc` only for actual shared ownership. Refcounting extends lifetime; it defines neither shutdown nor thread safety.
- Share memory by communicating when state has one natural owner: local mutation, task-owned state + channels (actor), or results returned via `JoinHandle`/`JoinSet` before `Arc<Mutex<_>>`. For genuine sharing, pick mutex/`RwLock`/atomics/channel/owning task by access pattern and contention. Related state transitions share one critical section.
- Never hold a sync guard (`std`/`parking_lot`) across `.await`; keep sync critical sections short, with no external calls. `std::sync::Mutex` is right in async for those sections. `tokio::sync::Mutex` guards may span `.await` by design: use one only when the critical section must, and weigh contention, lock order, and cancellation mid-section.
- `Send`/`Sync` follow contents; check bounds, don't assume a wrapper confers them. `Cell`/`RefCell` are single-threaded; `RefCell` panics on conflicting borrows.

## Initialization

Ordinary construction when data is available. `OnceLock`/`LazyLock`/`OnceCell` (std) only for genuinely deferred or shared init; check MSRV; `once_cell`/`lazy_static` are legacy for new code. No partially initialized objects ([example](../examples/once_cell/src/main.rs)).

## Structured concurrency

- Every spawned task has an owner that observes its outcome. `JoinSet`/`JoinHandle` yield results and surface panics as `JoinError`. `tokio_util::task::TaskTracker` only waits for completion: pair it with an explicit outcome policy (tasks report errors themselves) or retained handles. Dropping a `JoinHandle` detaches, it doesn't cancel. Fire-and-forget needs a stated reason.
- Cancellation is a state transition: stop admission → signal (`tokio_util::sync::CancellationToken`) → in-flight policy (drain, abort, deadline) → observe completion. A signal proves nothing about cleanup. [Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown).
- **Cancel safety:** any `.await` may be the last. In `select!` loops use branches documented as cancel-safe, or pin the future outside the loop. State what has committed when a future is dropped.
- Blocking I/O or CPU work goes off the runtime: `spawn_blocking`, `rayon`, a dedicated thread.
- RAII covers synchronous release. There is no async `Drop`: async cleanup needs an explicit `shutdown().await` path and an owner that awaits it.
- Bound queues and concurrency (`mpsc::channel(n)`, `Semaphore`, `buffer_unordered(n)`); backpressure and overload behavior are contract.

## Unsafe boundaries

- Safe abstraction first (`bytemuck`, `zerocopy`, std APIs). Otherwise a minimal module whose safe API is **sound**: no sequence of safe calls can cause UB. Audit every construction and mutation path; a marker trait or comment establishes nothing.
- FFI: `bindgen`/`cxx`; explicit ownership, aliasing, and teardown across the boundary. Panics unwinding out of `extern "C"` abort; use `catch_unwind` or `extern "C-unwind"` deliberately.
- Verify with `cargo +nightly miri test`; sanitizers for FFI.
