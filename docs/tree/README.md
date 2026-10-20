# The Abstract Syntax Tree

Parsing an expression produces an abstract syntax tree (AST), represented by
the public `ExprTree` enum in [/src/tree.rs](/src/tree.rs). Each variant represents one kind of
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


## Evaluation

The `eval` method walks the tree recursively and computes a value. It takes
ownership of the tree (`self`) and borrows a map of variable values. The map
associates each variable name with an `f64`:

```rs
impl ExprTree {
    pub fn eval(self, map: &HashMap<String, f64>) -> Result<f64> {
        match self {
            ExprTree::Float(num) => Ok(num),
            ExprTree::Var(name) => map
                .get(&name)
                .ok_or_else(|| Error::NoValue(name.clone()))
                .map(Clone::clone),
            ExprTree::Neg(expr) => Ok(-expr.eval(map)?),
            ExprTree::Add(lhs, rhs) => Ok(lhs.eval(map)? + rhs.eval(map)?),
            ExprTree::Sub(lhs, rhs) => Ok(lhs.eval(map)? - rhs.eval(map)?),
            ExprTree::Mul(lhs, rhs) => Ok(lhs.eval(map)? * rhs.eval(map)?),
            ExprTree::Div(lhs, rhs) => Ok(lhs.eval(map)? / rhs.eval(map)?),
        }
    }
}
```

The `match` handles every possible kind of node:

- A `Float` already contains its value, so evaluation returns it as `Ok(num)`.
- A `Var` looks up its name in `map`. If the name exists, the stored value is
  returned. If it is missing, evaluation returns `Error::NoValue` with that
  variable's name.
- A `Neg` evaluates its child and negates the resulting number.
- `Add`, `Sub`, `Mul`, and `Div` evaluate their left and right child trees,
  then apply the corresponding arithmetic operation.

The return type is `Result<f64>`, so evaluation can either succeed with a
number (`Ok(value)`) or fail with an evaluation error (`Err(error)`). The `?`
operator is used in the recursive cases: if evaluating a child returns an
error, `?` immediately returns that error from the current call. For example,
if a variable anywhere in the tree is missing from the map, the error travels
back up through its parent nodes to the caller.

Here is a complete example:

```rust
use expr::parse;
use std::collections::HashMap;

let tree = parse("2 * x + 19").unwrap();
let mut variables = HashMap::new();
variables.insert("x".to_string(), 12.0);

let value = tree.eval(&variables).unwrap();
assert_eq!(value, 43.0);
```

Calling `eval` consumes `tree`, because its receiver is `self` rather than
`&self`. After evaluation, that tree value cannot be used again. Also, division
by zero is not reported as an `ExprTree` evaluation error: the operation uses
Rust's `f64` arithmetic, which produces floating-point infinity or NaN as
appropriate.

```bash
➜  mkindahl-expr-rs git:(learning) cargo run --bin expr -- '2/(x-9)' x=9 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/expr '2/(x-9)' x=9`
inf
➜  mkindahl-expr-rs git:(learning) cargo run --bin expr -- '0/(x-9)' x=9
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr '0/(x-9)' x=9`
NaN
```
