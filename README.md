# Rust Programming Best Practices Handbook
## Connectors dev team "From Practice to Practitione"

## Summary
- [Chapter 1 - Coding Style and Idioms](./book/chapter_01.md)
    - [Borrowing Over Cloning](./book/chapter_01.md#11-borrowing-over-cloning)
    - [When to pass by value?](./book/chapter_01.md#12-when-to-pass-by-value-copy-trait)
    - [Handling Option<T> and Result<T, E>](./book/chapter_01.md#13-handling-option-and-resultt-e)
    - [Prevent Early Allocation](./book/chapter_01.md#14-prevent-early-allocation)
    - [Iterator, `.iter` vs `for`](./book/chapter_01.md#15-iterator-iter-vs-for)
    - [Comments: Context, not Clutter](./book/chapter_01.md#16-comments-context-not-clutter)
- [Chapter 2 - Clippy and Linting Discipline](./book/chapter_02.md)
    - [Why care about linting?](./book/chapter_02.md#21-why-care-about-linting)
    - [Always run `cargo clippy`](./book/chapter_02.md#22-always-run-cargo-clippy)
    - [Important Clippy Lints to Respect](./book/chapter_02.md#23-important-clippy-lints-to-respect)
    - [Fix warnings, don't silence them!](./book/chapter_02.md#24-fix-warnings-dont-silence-them)
    - [Configure workspace/package lints](./book/chapter_02.md#25-configure-workspacepackage-lints)
- [Final Notes](./book/zz_final_notes.md)