# expr-rs: recursive descent expression parser

An implementation of an simple expression parser with a focus on being
fast, compact, simple to embed, and with a minimum of
dependencies. The parser reads a string to build an expression
tree. The tree can then be evaluated, inspected, or manipulated.

## Grammar

The grammar of expressions is straightforward and (currently) quite
basic:

```ebnf
Expr = Term, { ("+" | "-"), Term }
Term = Factor, { ("*" | "/"), Factor}
Factor = [ "+" | "-" ], ( Number | Variable | "(", Expr, ")" )
```

## Example

The parser is implemented as a library to ensure that it can easily be
embedded into other applications and only has dependencies on the
standard library to be as lightweight as possible. A typical usage
can be found in the [examples/evaluate.rs](examples/evaluate.rs) file:

```rust
use expr::parse;
use std::collections::HashMap;

fn main() {
    let expression = "2 * x + 19";
    let tree = parse(expression).unwrap();

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 12.0);

    let value = tree.eval(&vars).unwrap();
    println!("{} = {}", expression, value);
}
```

To execute the example:

```bash
➜  mkindahl-expr-rs git:(learning) ✗ cargo run --example evaluate
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/evaluate`
2 * x + 19 = 43
```

## Sample Application `src/bin/expr.rs`

A sample application that just evaluates the expression with optional
assignments are provided with the package in [src/bin/expr.rs](src/bin/expr.rs). To execute it, run the following command:

```bash
cargo run --bin expr -- '2 * x + 19' x=12
```

By default `cargo run` will build and execute the default binary, which is `src/main.rs`. The `--bin expr` option tells Cargo to build and run the `expr` binary instead. The `--` option separates the arguments to the binary from the arguments to Cargo itself. The folder `src/bin/` is also scanned automatically for binary targets. If the code is in `src/bin/expr.rs` or `src/bin/expr/main.rs`, Cargo will recognize it as a binary target named `expr`. The first argument after `--` is the expression to evaluate, and any subsequent arguments are variable assignments in the form `<variable>=<value>`.

That will output:

```
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/expr '2*x+19' x=12`
43
```

Other examples:

```bash
➜  mkindahl-expr-rs git:(learning) cargo run --bin expr -- '-x' x=12   
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr -x x=12`
-12
➜  mkindahl-expr-rs git:(learning) cargo run --bin expr -- 'y*x' x=4 y=8
   Compiling expr v0.1.0 (/Users/casianorodriguezleon/campus-virtual/2627/learning/rust/recdescent-rust/recdescent/mkindahl-expr-rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/expr 'y*x' x=4 y=8`
32
```

The core of the [src/bin/expr.rs](/src/bin/expr.rs) file is just a few lines of code that parse the command line arguments, call the library's `eval` function, and print the result. The library handles all parsing and evaluation.

```rs 
use expr::eval;  // The library provides a single function `eval` 
use std::collections::HashMap; // Use a HashMap to provide variable values
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: expr <expression> [ <variable>=<value> ... ]");
    } else {
        let expr = &args[1];
        let mut map = HashMap::new();
        for assign in &args[2..] {
            let parts: Vec<&str> = assign.splitn(2, '=').collect();
            map.insert(parts[0].to_string(), parts[1].parse::<f64>()?);
        }
        println!("{}", eval(expr.as_str(), &map)?);
    }
    Ok(())
}
```

## The Tokenizer

See [docs/tokens/README.md](docs/tokens/README.md) for a description of the tokenizer at [src/tokens.rs](src/tokens.rs) and how it works.

## The Parser

See [docs/parser/README.md](docs/parser/README.md) for a description of the parser at [src/parser.rs](src/parser.rs) and how it works.

## The Abstract Syntax Tree

See [docs/tree/README.md](/docs/tree/README.md) for a description of the abstract syntax tree at [src/tree.rs](src/tree.rs) and how trees are evaluated.

## The root library src/lib.rs

The file [src/lib.rs](src/lib.rs) is the root of the `expr` library crate. It
connects the tokenizer, parser, and expression tree, and defines the API that
other Rust code can use.

It declares the `parser`, `tokens`, and `tree` modules as public. It also
re-exports `parse` from the parser module, so a library user can write
`expr::parse(...)` instead of `expr::parser::parse(...)`.

The library provides two ways to work with expressions:

- `parse(expression)` parses the input and returns an `ExprTree`, which can be
  inspected, manipulated, or evaluated.
- `eval(expression, variables)` parses and evaluates an expression in one
  step, using a `HashMap<String, f64>` to supply variable values.

The root `Error` enum brings parser errors and evaluation errors together.
Consequently, `eval` returns the library's `Result<f64>` and can report either
kind of failure.

For example, a program using the library can evaluate an expression directly:

```rust
use expr::eval;
use std::collections::HashMap;

let mut variables = HashMap::new();
variables.insert("x".to_string(), 12.0);

let value = eval("2 * x + 19", &variables).unwrap();
assert_eq!(value, 43.0);
```

The library root is not the executable's entry point. The command-line
application has its own `main` function in [src/bin/expr.rs](src/bin/expr.rs)
and uses the API exposed by this library.


## Authorship

This is a fork of [mkindahl/expr-rs](https://github.com/mkindahl/expr-rs).