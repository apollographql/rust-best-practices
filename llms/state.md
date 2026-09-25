# State and Invariants

## Parse, don't validate

Trust boundaries (CLI, HTTP, config, files, env, DB rows, FFI): parse raw input into domain types; the interior doesn't re-check type-carried invariants. `fn send(to: &Email)`, not `fn send(to: &str) // must be valid`. Avoid shotgun parsing (scattered checks) and boolean blindness (`is_valid(&x) -> bool` discards evidence).

- `TryFrom`, `FromStr`, smart constructors + private fields. Proof obligation: every construction, mutation, deserialization, and conversion preserves the invariant; privacy alone doesn't prove it.
- Deserialization: wire type + `#[serde(try_from = "Raw")]` (+ `into = "Raw"` to serialize) ([example](examples/parse-dont-validate/src/main.rs)). A plain `#[derive(Deserialize)]` on a domain type bypasses its constructor.
- Reuse constrained types: `NonZeroU16`, `SocketAddr`, `IpAddr`, `Duration`, `url::Url`, `uuid::Uuid`, `semver::Version`. Name by guarantee (`Port`, `TenantId`) over `ValidatedX` when a domain name exists.

## Make illegal states unrepresentable

- Algebraic data types: product = coexistence, sum = alternatives. State-space cardinality: `(Option<A>, Option<B>)` admits four presence combinations; exactly one valid → `enum { A(A), B(B) }`; both valid too → add `Both(A, B)`. Data lives in its variant.
- `Option`/`Result` when absence or failure is the concept. `bool` params/fields naming a domain choice → enum (`Mode::Strict` over `true` at call sites); plain predicates stay `bool`.
- Newtypes against primitive obsession when an invariant or semantic distinction makes substitution a mistake (IDs, units, validated strings); not for every primitive.
- List variants instead of `_ =>` when a new variant should force reconsidering the decision; `_` may deliberately group irrelevant alternatives. Downstream matches on a `#[non_exhaustive]` enum require `_`.
- Let state carry its data: `Connected { socket }`, not `state: State` + `socket: Option<Socket>`. Existing types may already be the state: an open `File` is the opened state ([open handle](examples/open-handle/src/main.rs)); a wrapper must add a real guarantee.

## Transitions consume their source

- `self`-consuming methods retire the prior state's capabilities. Concrete per-state types or an enum first; typestate generics (`Conn<S>`) only for composability or a guarantee concrete types can't give. Carry state data directly, never `PhantomData` + `Option` + `unreachable!`.
- Runtime-determined state (events, callbacks, shared ownership) → payload enum owned by one component; change state and its data together, under one lock or task.
- Consuming a value can't revoke clones, remote tokens, or other processes' views.
- Failed transitions: choose explicitly between returning the original (`Err((self, e))`), a recovery state, or releasing resources.
- Builders are an API choice, not a typestate showcase. Few required fields → constructor args ([required constructor](examples/required-constructor/src/main.rs)); substantial builders → `bon` or `typed-builder`.

## Authority boundary

- Parsing ≠ authorization. Complete mediation: authorize each access by actor, action, resource, current policy. TOCTOU: an open handle doesn't freeze contents; a checked path can change before use. Identify the source of truth and who can mutate it before promising freshness or availability.
- Distinguish peer claims, negotiated state, and locally supported behavior. Guard safety- and capability-critical boundaries explicitly even when a dependency overlaps, citing its contract; not a mandate to re-check every dependency.
