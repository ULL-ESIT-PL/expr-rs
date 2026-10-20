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

See [docs/tokens/README.md](docs/tokens/README.md) for a description of the tokenizer and how it works.

## The Parser

See [docs/parser/README.md](docs/parser/README.md) for a description of the parser and how it works.

## The Abstract Syntax Tree

See [docs/tree/README.md](/docs/tree/README.md) for a description of the abstract syntax tree and how it can be evaluated and

## Authorship

This is a fork of [mkindahl/expr-rs](https://github.com/mkindahl/expr-rs).