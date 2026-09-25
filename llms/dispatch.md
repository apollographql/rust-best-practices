# Abstraction and Dispatch

**Domain boundary before dispatch mechanism.** Nothing varies → concrete type. Closed set → enum + exhaustive match. Open family or downstream extension → trait. No trait per struct or per hypothetical second impl; for test seams see [testing](testing.md).

- Generics/`impl Trait`: keep concrete types, enable inlining; cost is code size, compile time, generic propagation.
- `dyn Trait`: runtime heterogeneity, type erasure, stopping generic propagation; cost is indirection and lost inlining. Check [dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) against the supported compiler.
- Ownership is independent of dispatch: `&dyn T` borrows, `Box<dyn T>` owns, `Arc<dyn T + Send + Sync>` shares. `dyn` needs no allocation.
- Bounds from actual use; no reflexive `Send + Sync + 'static` or `Clone + Debug` on every parameter.
- `async fn` in traits: native for static dispatch; `dyn` use needs boxing (`async-trait`) and `Send` bounds may need `trait-variant`. Check MSRV.
- Public surface: seal traits not meant for downstream impls; `#[non_exhaustive]` on public types expected to grow. A trait object is not a stable plugin ABI.
- Information hiding: expose the smallest contract consumers need; use installed extension points over parallel adapters.
