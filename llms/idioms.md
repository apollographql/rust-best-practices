# Idioms and Ecosystem

## Ownership in APIs

- Params: `&str`/`&[T]`/`&T` to view; owned `T` to retain. `impl Into<T>`/`AsRef` only where caller ergonomics justify the hidden conversion. `Cow` only for a real borrowed-or-owned contract.
- Derive `Copy` only when implicit duplication fits the domain. Not on stateful types (iterators, cursors, accumulators): copies fork state silently.

## Abstraction

- Functional core, imperative shell: pure decisions, effects at the edges.
- DRY is about knowledge, not text. Extract shared policy; avoid change coupling between independently evolving decisions. Line/occurrence counts aren't the criterion.
- Wrong abstraction: when caller-specific flags accumulate, inline into callers, specialize, re-extract the real commonality. An enum param names a genuine choice; it can't rescue an incoherent helper.

## Declare intent; delegate mechanics

Inspect installed deps/extension points first. Select for maintenance, adoption, MSRV/feature/license/deployment fit; name recognition alone doesn't justify replacement.

Defaults to evaluate, not impose:

- **CLI:** `clap` for help/subcommands/constraints (`Parser`, payload `Subcommand`, `ValueEnum`, `ArgGroup`); `argh` for smaller derive; `pico-args` for minimal parsing without help. No parallel argv/help. Flags/defaults/env names are public API; parse cross-field invariants into domain types.
- **Formats:** `serde` + the format crate (`serde_json`, `toml`). `#[serde(try_from = "Raw")]` when deserialization alone doesn't establish invariants. Know what `default`, `flatten`, `untagged`, `deny_unknown_fields` actually do.
- **Protocols and data:** `url`, `bytes`, `tokio_util::codec` for framing, `http`/`hyper`/`reqwest`/`axum`/`tonic`, `jiff` or `chrono` for time. Prefer the existing stack when suitable.
- **Errors/tests:** [errors](errors.md), [testing](testing.md).

Derives and schemas remove mechanics, not policy: authorization, data authority, and application rules stay explicit.
