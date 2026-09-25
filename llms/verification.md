# Verification

**Tools own mechanics.** Before choosing commands, read `rust-toolchain(.toml)`, `rust-version`, `edition`, `[lints]`, `clippy.toml`, feature definitions, CI config. Use the pinned toolchain and locked deps; no incidental upgrades.

This workspace:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

- Inner loop: `cargo check`/`clippy` after each coherent edit. The compiler is the cheapest oracle; read notes and `help:` lines.
- Elsewhere mirror CI's feature/target matrix. `--all-features` breaks on mutually exclusive features and skips `--no-default-features`; `cargo hack --each-feature`/`--feature-powerset` covers combinations.
- Include doctests (`cargo nextest` skips them; run `cargo test --doc`) and integration tests relevant to the change.
- Where applicable: `cargo semver-checks` for library public API; `cargo deny`/`cargo audit` for deps; `cargo +nightly miri test` for unsafe; `cargo mutants` to test the tests.

Warnings: fix the cause. Narrow `#[expect(lint, reason = "…")]` over `#[allow]`; no blanket suppression. Review mechanical fixes (`clippy --fix`, suggestions) that touch ownership, public API, or perf. Enduring lint policy lives in `[workspace.lints]`, not prose.

Green gates ≠ correct behavior. Report evidence and limits: checks run, failures, unverified claims; distinguish regressions from pre-existing failures. Review behavior, invariants, dep fit, oracles—not compiler diagnostics.
