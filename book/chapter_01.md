# Chapter 1 — Idioms and Ecosystem

## Ownership in APIs

- Params: `&str`/`&[T]`/`&T` to view; owned `T` to retain. `impl Into<T>`/`AsRef` only where caller ergonomics justify the hidden conversion. `Cow` only for a real borrowed-or-owned contract.
- Derive `Copy` only when implicit duplication fits the domain. Not on stateful types (iterators, cursors, accumulators): copies fork state silently.

## Abstraction

- Functional core, imperative shell: pure decisions, effects at the edges.
- DRY is about knowledge. Extract a shared decision, a name, or a boundary. Line and occurrence counts are weak signals; similar code whose policies evolve independently stays separate.
- Wrong abstraction: when caller-specific flags accumulate, inline into callers, specialize, re-extract the real commonality. An enum param names a genuine choice; it can't rescue an incoherent helper.

## Declare intent; delegate mechanics

Inspect workspace deps and their extension points before writing machinery. Select for maintenance, adoption, MSRV/feature/license/deployment fit. A suitable installed dep beats a better-known replacement; familiarity isn't a criterion.

Defaults to evaluate, not impose:

- **CLI:** `clap` derive: `Parser`, payload `Subcommand` enums, `ValueEnum`, `value_parser`, `conflicts_with`/`requires`/`ArgGroup`, `env`. No manual argv scanning, no parallel help/validation. Flag names, defaults, env names are public API. Convert to domain types for cross-field invariants.
- **Formats:** `serde` + the format crate (`serde_json`, `toml`). `#[serde(try_from = "Raw")]` when deserialization alone doesn't establish invariants ([example](../examples/parse-dont-validate/src/main.rs)). Know what `default`, `flatten`, `untagged`, `deny_unknown_fields` actually do.
- **Protocols and data:** `url`, `bytes`, `tokio_util::codec` for framing, `http`/`hyper`/`reqwest`/`axum`/`tonic`, `jiff` or `chrono` for time. Whatever the workspace already uses wins.
- **Errors/tests:** [errors](chapter_04.md), [testing](chapter_05.md).

Derives and schemas remove mechanics, not policy: authorization, data authority, and application rules stay explicit.
