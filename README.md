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

## Sample Application src/bin/expr.rs

A sample application that just evaluates the expression with optional
assignments are provided with the package in [src/bin/expr.rs](src/bin/expr.rs). To execute it, run the following command:

```bash
cargo run --bin expr -- '2 * x + 19' x=12
```

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
➜  mkindahl-expr-rs git:(learning) cargo run --bin expr -- '2*(x-9)' x=12
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/expr '2*(x-9)' x=12`
6
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