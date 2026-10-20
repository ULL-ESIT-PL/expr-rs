# expr-rs: recursive decent expression parser

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

Parsing an expression produces an abstract syntax tree (AST), represented by
the public `ExprTree` enum in [src/tree.rs](src/tree.rs). Each variant represents one kind of
expression:

| Variant                             | Meaning |
| ---                                 | --- |
| `Float(f64)`                        | A numeric literal |
| `Var(String)`                       | A variable name |
| `Neg(Box<ExprTree>)`                | Unary negation |
| `Add(Box<ExprTree>, Box<ExprTree>)` | Addition |
| `Sub(Box<ExprTree>, Box<ExprTree>)` | Subtraction |
| `Mul(Box<ExprTree>, Box<ExprTree>)` | Multiplication |
| `Div(Box<ExprTree>, Box<ExprTree>)` | Division |

For example, parsing `2 * (x - 9)` produces a tree equivalent to:

```text
Mul
├── Float(2.0)
└── Sub
    ├── Var("x")
    └── Float(9.0)
```

The parentheses do not need their own AST node: they determine how the
expression is grouped, which is reflected by the tree's structure. The parser
also encodes operator precedence in the tree, so multiplication and division
form subtrees beneath addition and subtraction when appropriate.

`Box<ExprTree>` provides indirection for child expressions. This lets the
recursive enum have a finite size while still representing trees of arbitrary
depth. Once parsed, an `ExprTree` can be evaluated with `eval`, inspected, or
manipulated.

## Authorship

This is a fork of [mkindahl/expr-rs](https://github.com/mkindahl/expr-rs).