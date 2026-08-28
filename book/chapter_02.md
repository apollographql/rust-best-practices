# Chapter 2 - Clippy and Linting Discipline

Be sure to have `cargo clippy` installed with your Rust toolchain. Run `cargo clippy -V` from a Rust project and you should get something like `clippy 0.1.86 (05f9846f89 2025-03-31)`. If the terminal fails to show a Clippy version, install it with `rustup component add clippy`.

Clippy documentation can be found [here](https://doc.rust-lang.org/clippy/usage.html).

## 2.1 Why care about linting?

The Rust compiler is a powerful tool that catches many mistakes. However, some more in-depth analyses require extra tooling, and that is where `cargo clippy` comes into play. Clippy checks for:
* Performance pitfalls.
* Style issues.
* Redundant code.
* Potential bugs.
* Non-idiomatic Rust.

## 2.2 Always run `cargo clippy`

Add the following to your daily workflow:

```shell
$ cargo clippy --all-targets --all-features --locked -- -D warnings
```

* `--all-targets`: checks library, binaries, tests, benches and examples.
* `--all-features`: activates all available features for the selected packages. It does not resolve mutually exclusive features; those should be avoided, checked with `compile_error!`, or tested with targeted `--features` combinations.
* `--locked`: fails if `Cargo.lock` is missing or Cargo would need to change it because dependency resolution changed.
* `-D warnings`: treats all Rust and Clippy warnings as errors.

Potential additional lint flags:

* `-- -W clippy::pedantic`: enables stricter lints that can have occasional false positives.
* `-- -W clippy::nursery`: checks new lints that are still under development.
* ❗ Add this to your Makefile, Justfile, xtask or CI Pipeline.

> Example at ApolloGraphQL
>
> In the `Router` project there is a `xtask` configured for linting that can be executed with `cargo xtask lint`. 

## 2.3 Important Clippy Lints to Respect

| Lint Name | Why | Link |
| --------- | ----| -----|
| `redundant_clone` | Detects unnecessary `clone()` calls that can have a performance impact | [link (nursery)](https://rust-lang.github.io/rust-clippy/master/#redundant_clone) |
| `needless_borrow` | Removes redundant `&` borrowing | [link (style)](https://rust-lang.github.io/rust-clippy/master/#needless_borrow) |
| `map_unwrap_or` / `unnecessary_map_or` | Simplifies verbose `Option`/`Result` combinator chains | [`map_unwrap_or`](https://rust-lang.github.io/rust-clippy/master/#map_unwrap_or) [`unnecessary_map_or`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_map_or) [`unnecessary_result_map_or_else`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_result_map_or_else) |
| `manual_ok_or` | Suggests using `.ok_or(...)` instead of manually reimplementing `Option::ok_or` | [link (style)](https://rust-lang.github.io/rust-clippy/master/#manual_ok_or) |
| `large_enum_variant` | Warns when one enum variant makes the whole enum large; consider boxing when the allocation tradeoff is acceptable | [link (perf)](https://rust-lang.github.io/rust-clippy/master/#large_enum_variant) |
| `unnecessary_wraps` | If your function always returns `Some` or `Ok`, you don't need `Option`/`Result` | [link (pedantic)](https://rust-lang.github.io/rust-clippy/master/#unnecessary_wraps) |
| `clone_on_copy` | Catches accidental `.clone()` on `Copy` types like `u32` and `bool` | [link (complexity)](https://rust-lang.github.io/rust-clippy/master/#clone_on_copy) |
| `needless_collect` | Prevents collecting and allocating an iterator, when allocation is not needed | [link (nursery)](https://rust-lang.github.io/rust-clippy/master/#needless_collect) |

## 2.4 Fix warnings, don't silence them!

**NEVER** just `#[allow(clippy::lint_something)]` unless:

* You **truly understand** why the warning happens and you have a reason why it is better that way.
* You **document** why it is being ignored.
* ❗ Prefer `expect` over `allow` when you intentionally keep the code as-is. It will warn if the lint no longer triggers: `#[expect(clippy::lint_something)]`.

### Example:

```rust
// Inline storage is intentional here to avoid heap allocation.
#[expect(clippy::large_enum_variant)]
enum Message {
    Code(u8),
    Content([u8; 1024]),
}
```

> If the lint is correct, fix the enum instead and remove the `#[expect]`:
> 
> ```rust
> enum Message {
>     Code(u8),
>     Content(Box<[u8; 1024]>),
> }
> ```

### Handling false positives

Sometimes Clippy complains even when your code is correct, in those cases there are three options:
1. Try to refactor the code, so it improves the warning.
2. **Locally** override the lint with `#[expect(clippy::lint_name)]` and a comment with the reason.
3. Avoid global overrides, unless it is core crate issue, a good example of this is the Bevy Engine that has a set of lints that should be allowed by default.

## 2.5 Configure workspace/package lints

In `Cargo.toml`, you can configure lint levels for the package. When a lint group and a more specific lint both apply, `priority` controls which level wins; higher priority values override lower ones. Example configuration for a package:

```toml
[lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
redundant_clone = { level = "deny", priority = 1 }
manual_while_let_some = { level = "deny", priority = 1 }
```

And for a workspace:

```toml
[workspace.lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
redundant_clone = { level = "deny", priority = 1 }
manual_while_let_some = { level = "deny", priority = 1 }
```

Each workspace member that should use the shared lint configuration must opt in:

```toml
[lints]
workspace = true
```
