# The Cargo.toml file

The file [Cargo.toml](../../Cargo.toml) governs the build of this crate. You can publish the crate to [crates.io](https://crates.io) with `cargo publish`. The documentation of the `cargo` command is available at [https://doc.rust-lang.org/cargo/guide/](https://doc.rust-lang.org/cargo/guide/).

The description of the syntax and semantics of the Cargo file can be found at [https://doc.rust-lang.org/cargo/reference/manifest.html](https://doc.rust-lang.org/cargo/reference/manifest.html).


## [package] section: Metadata about the package

- `name = "crguezl-expr"`: The name of the package. This is used when publishing the crate to crates.io.
- `version = "0.1.0"`: The version of the package. This follows semantic versioning (semver) rules. (`MAJOR.MINOR.PATCH`). Starts with and therfore the API is not yet stable.
- `authors = ["Mats Kindahl <mats.kindahl@gmail.com>", "Casiano Rodriguez-Leon <crguezl@ull.edu.es"]`: The authors of the package. This is a list of strings, each containing the name and email address of an author.
- `description = "Example of Recursive descent expression parser used for teaching."`: A short description of the package. This is used when publishing the crate to [https://crates.io](https://crates.io).
- `documentation = "https://docs.rs/crguezl-expr"`: The URL of the documentation for the package. This is used when publishing the crate to [https://crates.io](https://crates.io).
- `edition = "2024"`: The `edition` key is an optional key that affects which Rust Edition your package is compiled with. Setting the edition key in `[package]` will affect all targets/crates in the package, including test suites, benchmarks, binaries, examples, etc. Most manifests have the `edition` field filled in automatically by [cargo new](https://doc.rust-lang.org/cargo/commands/cargo-new.html) with the latest stable edition. 
If the `edition` field is not present in [Cargo.toml](../../Cargo.toml), then the 2015 edition is assumed for backwards compatibility. 



### What does not appear and why it matters

[[dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html): There are no dependencies declared in the Cargo.toml file, which means that the crate does not depend on any external crates. This is important because it means that the crate can be used without having to install any additional crates, and it also means that there are
no hay dependencias de runtime. 

[[[bin]]](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries): Not declared, but since there is a src/main.rs file, Cargo automatically detects it as a binary named expr.

[lib]: no se declara, pero si existe src/lib.rs, Cargo lo auto-detecta como biblioteca expr.

## Exercise: run cargo metadata

Run

```
cargo metadata | jless
```

Be sure to have `jless` installed. This command will print the metadata of the package in JSON format, which can be useful for debugging and understanding the package's dependencies and structure.

## Exercise: run cargo bench
