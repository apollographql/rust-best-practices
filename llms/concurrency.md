# Resources and Concurrency

## Ownership before synchronization

- Name the owner of every resource and task; distinguish process, connection, and request lifetimes. Borrow for access; `Box` for owned indirection; `Rc`/`Arc` only for actual shared ownership. Refcounting extends lifetime; it defines neither shutdown nor thread safety.
- Share memory by communicating when state has one natural owner: local mutation, task-owned state + channels (actor), or results returned via `JoinHandle`/`JoinSet` before `Arc<Mutex<_>>`. For genuine sharing, pick mutex/`RwLock`/atomics/channel/owning task by access pattern and contention. Related state transitions share one critical section.
- Sync guards (`std`/`parking_lot`): short critical sections, no `.await` or external calls. In async code, contention must also be low: acquisition blocks the worker. Otherwise consider an owning task or `tokio::sync::Mutex`; assess contention, deadlock/lock ordering, and cancellation mid-section.
- `Send`/`Sync` follow contents; check bounds, don't assume a wrapper confers them. `Cell`/`RefCell` are single-threaded; `RefCell` panics on conflicting borrows.

## Initialization

Ordinary construction first; std `OnceLock`/`LazyLock`/`OnceCell` for deferred/shared init, subject to MSRV and required APIs. Cached derivations require stable inputs or cache invalidation ([example](examples/deferred-init/src/main.rs)); no partially initialized objects.

## Structured concurrency

- Every spawned task has an owner that observes its outcome. `JoinSet`/`JoinHandle` yield results and surface panics as `JoinError`. `tokio_util::task::TaskTracker` only waits for completion: pair it with an explicit outcome policy (tasks report errors themselves) or retained handles. Dropping a `JoinHandle` detaches, it doesn't cancel. Fire-and-forget needs a stated reason.
- Shutdown protocol: stop admission → signal (`tokio_util::sync::CancellationToken`) → drain/abort/deadline → observe completion. Safety: preserve invariants; liveness: state how tasks terminate. Signal ≠ cleanup. [Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown).
- **Cancellation safety:** any `.await` may be the last. In `select!` loops use cancel-safe branches or retain/pin the future across iterations. Identify commit points: dropping a future doesn't roll back effects.
- Blocking I/O or CPU work goes off the runtime: `spawn_blocking`, `rayon`, a dedicated thread.
- RAII covers synchronous release. There is no async `Drop`: async cleanup needs an explicit `shutdown().await` path and an owner that awaits it.
- Backpressure/admission control: bound queued and in-flight work (`mpsc::channel(n)`, `Semaphore`, `buffer_unordered(n)`); define whether overload waits, rejects, or sheds work.

## Unsafe boundaries

- Safe abstraction first (`bytemuck`, `zerocopy`, std APIs). Otherwise a minimal module whose safe API is **sound**: no sequence of safe calls can cause UB. Audit every construction and mutation path; a marker trait or comment establishes nothing.
- FFI: `bindgen`/`cxx`; explicit ownership, aliasing, and teardown across the boundary. Panics unwinding out of `extern "C"` abort; use `catch_unwind` or `extern "C-unwind"` deliberately.
- Verify with `cargo +nightly miri test`; sanitizers for FFI.
