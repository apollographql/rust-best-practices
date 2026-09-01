# Chapter 2 - Clippy and Linting Discipline

Be sure to have `cargo clippy` installed with your rust compiler, run `cargo clippy -V` in your terminal for a rust project and you should get something like this `clippy 0.1.86 (05f9846f89 2025-03-31)`. If terminal fails to show a clippy version, please run the following code `rustup update && rustup component add clippy`.

Clippy documentation can be found [here](https://doc.rust-lang.org/clippy/usage.html).

## 2.1 Why care about linting?

Rust compiler is a powerful tool that catches many mistakes. However, some more in-depth analysis require extra tools, that is where `cargo clippy` comes into play. Clippy checks for:
* Performance pitfalls.
* Style issues.
* Redundant code.
* Potential bugs.
* Non-idiomatic Rust.

**Only allow the lint in the minimal scope**
1. Just over the concerned line.
2. If it is not enough, over brackets / function.
3. If it is not enough, on file scope (eg. `#![expect(clippy::lint_name, reason = "<your_reason>")])`).
4. Lastly, in the Cargo.toml if you do not want to follow the lint at all.

## 2.2 Always run `cargo clippy`

Add the following to your daily workflow:

```shell
$ cargo clippy --all-targets --all-features --locked --tests -- -D warnings
```

* `--all-targets`: checks library, tests, benches and examples.
* `--all-features`: checks code with all features enabled; it does not auto-solve conflicting features.
* `--locked`: Requires `Cargo.lock` to be up-to-date, can be solved with `$ cargo update`.
* `--tests`: also check tests.
* `-D warnings`: treats warnings as errors.

Potential additional elements to add:

* `-- -W clippy::pedantic`: lints which are rather strict or have occasional false positives.
* `-- -W clippy::nursery`: Optionally can be added to check for new lints that are still under development.
* ❗ Add this to your Makefile, Justfile, xtask or CI Pipeline.

> Example at ApolloGraphQL
>
> In the `Router` project there is a `xtask` configured for linting that can be executed with `cargo xtask lint`.

## 2.3 Important Clippy Lints to Respect

| Lint Name | Why | Link |
| --------- | ----| -----|
| `redundant_clone` | Detects unnecessary `clones`, has performance impact | [link (nursery + perf)](https://rust-lang.github.io/rust-clippy/master/#redundant_clone) |
| `needless_borrow` group | Removes redundant `&` borrowing | [link (style)](https://rust-lang.github.io/rust-clippy/master/#needless_borrow) |
| `map_unwrap_or` / `map_or` | Simplifies nested `Option/Result` handling | [`map_unwrap_or`](https://rust-lang.github.io/rust-clippy/master/#map_unwrap_or) [`unnecessary_map_or`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_map_or) [`unnecessary_result_map_or_else`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_result_map_or_else) |
| `manual_ok_or` | Suggest using `.ok_or_else` instead of `match` | [link (style)](https://rust-lang.github.io/rust-clippy/master/#manual_ok_or) |
| `large_enum_variant` | Warns if an enum has very large variant which is bad for memory. Suggests `Boxing` it | [link (perf)](https://rust-lang.github.io/rust-clippy/master/#large_enum_variant) |
| `unnecessary_wraps` | If your function always returns `Some` or `Ok`, you don't need `Option`/`Result` | [link (pedantic)](https://rust-lang.github.io/rust-clippy/master/#unnecessary_wraps) |
| `clone_on_copy` | Catches accidental `.clone()` on `Copy` types like `u32` and `bool` | [link (complexity)](https://rust-lang.github.io/rust-clippy/master/#clone_on_copy) |
| `needless_collect` | Prevents collecting and allocating an iterator, when allocation is not needed | [link (nursery)](https://rust-lang.github.io/rust-clippy/master/#needless_collect) |

## 2.4 Interesting lints which may interest you

| Lint Name | Why | Group & link |
| --------- | --- | ---- |
| `allow_attributes` | Enforce using `#[expect(...)]` which will check if it is still used | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#allow_attributes) |
| `allow_attribute_without_reason` | Enforce using `#[allow(..., reason = "...")]` to precise why you authorize it | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#allow_attribute_without_reason) |
| `assertion_on_result_states` | Rather `r.unwrap/unwrap_err()` than `assert!(r.is_ok/is_err())` | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#assertions_on_result_states) |
| `branches_sharing_code` | Don’t Repeat Yourself | [nursery](https://rust-lang.github.io/rust-clippy/stable/index.html#branches_sharing_code) |
| `collapse_else_if` | `else { if { ... }}` => `else if {}` | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#collapsible_else_if) |
| `definition_in_module_root` | Avoid `mod.rs` containing code, to avoid having too much files `mod.rs` opened to develop | [restriction](https://rust-lang.github.io/rust-clippy/master/index.html#definition_in_module_root) |
| `duration_suboptimal_units` | `Duration::from_millis(10_000)` => `Duration::from_secs(10)` | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#duration_suboptimal_units) |
| `expect_used`, `panic` & `unwrap_used` | Avoid production code to panic (can be authorized in tests: [expect](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-expect-in-tests), [panic](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-panic-in-tests) & [unwrap](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-unwrap-in-tests)) | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#expect_used), [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#panic) & [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#unwrap_used) |
| `ignore_without_reason` | Enforce giving a reason to ignore a test: `#[ignore = "..."]` | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#ignore_without_reason) |
| `indexing_slicing` | Avoid panic if the index does not exist [(can be authorized in tests)](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-indexing-slicing-in-tests) | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#ignore_without_reason) |
| `manual_assert` & `manual_assert_eq` | Rather `assert!()` & `assert_eq!()` rather than `panic!()` if it can be done | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#manual_assert) & [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#manual_assert_eq) |
| `manual_let_else` | Rather `let Some(v) = w else { return };` | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#manual_let_else) |
| `match_same_arms` | Warn if 2 arms of a match are the same => group them | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#manual_let_else) |
| `needless_for_each` | Indicate when use a `for` loop | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#needless_for_each) |
| `print_stderr` & `print_stdout` | Rather log tools than `println!()` ([can be authorize in tests](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-print-in-tests)) | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#print_stderr) & [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#print_stdout) |
| `redundant_clone` | Remove useless `.clone()` | [nursery](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-indexing-slicing-in-tests) |
| `redundant_else` | Remove useless `else {}` (end of function) | [pedantic](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#allow-indexing-slicing-in-tests) |
| `too_many_lines` | Avoid function > 100 lines [(can be configured)](https://doc.rust-lang.org/stable/clippy/lint_configuration.html#too-many-lines-threshold) | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#too_many_lines) |
| `unimplemented` & `unreachable` | Avoid panic in production | [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#unimplemented) & [restriction](https://rust-lang.github.io/rust-clippy/stable/index.html#unreachable) |
| `unused_async` | Remove `async` when not used | [pedantic](https://rust-lang.github.io/rust-clippy/stable/index.html#unused_async) |

## 2.5 Fix warnings, don't silence them!

**NEVER** just `#[allow(clippy::lint_something)]` unless:

* You **truly understand** why the warning happens and you have a reason why it is better that way.
* You **document** why it is being ignored (cf. lint [allow_attribute_without_reason](https://rust-lang.github.io/rust-clippy/stable/index.html#allow_attribute_without_reason))
* ❗ Don't use `allow`, but `expect`, it will give a warning in case the lint is not true anymore, `#[expect(clippy::lint_something)]`. (cf. lint [allow_attributes](https://rust-lang.github.io/rust-clippy/stable/index.html#allow_attributes))

### Example:

```rust
#[expect(clippy::large_enum_variant, reason = "Faster matching is preferred over size efficiency")]
enum Message {
    Code(u8),
    Content([u8; 1024]),
}
```

> The fix would be:
>
> ```rust
> enum Message {
>     Code(u8),
>     Content(Box<[u8; 1024]>),
> }
> ```

### Handling false positives

Sometimes Clippy complains even when your code is correct, in those cases there are two solutions:
1. Try to refactor the code, so it improves the warning.
2. **Locally** override the lint with `#[expect(clippy::lint_name, reason = "<your_reason>")]` commenting with the reason.
3. Avoid global overrides, unless it is core crate issue, a good example of this is the Bevy Engine that has a set of lints that should be allowed by default.

## 2.6 Configure workspace/package lints

In your `Cargo.toml` file it is possible to determine which lints and their priorities over each other. In case of 2 or more conflicting lints, the higher priority one will be chosen. Example configuration for a package:

```toml
[lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = 3 }
manual_while_let_some = { level = "deny", priority = 4 }
redundant_clone = { level = "deny", priority = 9 }
```

And for a workspace:

```toml
[workspace.lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = 3 }
manual_while_let_some = { level = "deny", priority = 4 }
redundant_clone = { level = "deny", priority = 9 }
```