# The Cargo.toml file

The file [Cargo.toml](../../Cargo.toml) governs the build of this crate. You can publish the crate to [crates.io](https://crates.io) with `cargo publish`. The documentation of the `cargo` command is available at [https://doc.rust-lang.org/cargo/guide/](https://doc.rust-lang.org/cargo/guide/).


## [package] section: Metadata about the package

- `name = "crguezl-expr"`: The name of the package. This is used when publishing the crate to crates.io.
- `version = "0.1.0"`: The version of the package. This follows semantic versioning (semver) rules. (`MAJOR.MINOR.PATCH`). Starts with and therfore the API is not yet stable.
- `authors = ["Mats Kindahl <mats.kindahl@gmail.com>", "Casiano Rodriguez-Leon <crguezl@ull.edu.es"]`: The authors of the package. This is a list of strings, each containing the name and email address of an author.
- `description = "Example of Recursive descent expression parser used for teaching."`: A short description of the package. This is used when publishing the crate to [https://crates.io](https://crates.io).
- `documentation = "https://docs.rs/crguezl-expr"`: The URL of the documentation for the package. This is used when publishing the crate to [https://crates.io](https://crates.io).



## Exercise: run cargo metadata

Run

```
cargo metadata | jless
```

Be sure to have `jless` installed. This command will print the metadata of the package in JSON format, which can be useful for debugging and understanding the package's dependencies and structure.

## Exercise: run cargo bench
