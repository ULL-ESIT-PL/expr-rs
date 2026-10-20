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

A sample application that just evaluates the expression with optional
assignments are provided with the package in [src/bin/expr.rs](src/bin/expr.rs). To execute it, run the following command:

```bash
cargo run --bin expr -- '2 * x + 19' x=12
```

