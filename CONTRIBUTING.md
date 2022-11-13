# Rust Algorithms contribution guidelines

Thank you for contributing to this project! We expect all contributors to have read this file before submitting PRs.

## Structure

The structure of the project is the following:

```plain
rust-algorithms/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── CONTRIBUTING.md <-- This file
    ├── README.md
    ├── .github/
    │  └── workflows/
    │     └── basic.yml
    └── src/
        ├── lib.rs
        ├── ciphers/
        │  ├── mod.rs
        │  ├── ...
        │  └── ...
        ├── dynamic_programming/
        │  ├── ...
        │  ├── mod.rs
        │  └── ...
        ├── general/
        │  ├── mod.rs
        │  └── ...
        └── sorting/
            ├── ...
            ├── ...
            ├── mod.rs
            └── ...
```

The different classes of algorithms live under the `src` directory.
Under each class live the files that contain the algorithms and their appropriate tests.
Also, each class has a `mod.rs` file that exports the algorithm implementations.
This file should be updated when adding a new implementation.
`src/lib.rs` exports each class and should be updated if you add a new class.

## Pull Requests

To make changes to `rust-algorithms`, please submit a PR to the `main` branch.
We'll review them and either merge or request changes.
We have a basic CI setup that runs:

```rust
cargo check
cargo test
cargo fmt --all -- --check
```

## Issues

If you find a problem with an implementation, a typo, or you want to suggest improvements
to the project, file an issue.

## Testing

You should provide rigorous tests beside the algorithm implementations. PRs without
tests when appropriate won't be merged.

To run the tests:

```bash
cargo test
```

## Formatting

We adhere to the standard rules of formatting in rust.
Please, make sure that your changes follow them too by running:

```bash
cargo fmt
```

