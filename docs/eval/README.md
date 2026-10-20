# Understanding `src/eval.rs`

This tutorial explains the evaluator currently present in
[`src/eval.rs`](../../src/eval.rs). It describes a stack-based way to evaluate
an expression: values are pushed onto a stack, and each operator removes its
operands and pushes the result.

> **Important:** this file does not match the evaluator used by the active
> library. The current library evaluates the AST using `ExprTree::eval` in
> [`src/tree.rs`](../../src/tree.rs), and `src/lib.rs` does not declare an
> `eval` module. The `Machine`, `Parser`, `Error::MissingOperand`, and `Result`
> names imported by this file are not defined in the current crate. As a
> result, `src/eval.rs` appears to be leftover or incomplete code and is not
> compiled as part of the library. Read this as an example of a stack-machine
> evaluator, not as the implementation behind the crate's current `eval`
> function.

## 1. The evaluator's general idea

An evaluator needs to perform the operations described by an expression. This
file delegates parsing to a `Parser` and implements a `Machine` interface that
the parser can call when it finds values and operators.

The `Evaluator` stores numbers in a `Vec<f64>`:

```rust
#[derive(Debug)]
struct Evaluator {
    stack: Vec<f64>,
}
```

A `Vec` is a growable list. Here it is used as a **stack**, where values are
added and removed from the end. The stack follows last-in, first-out order
(LIFO): the last value pushed is the first one popped.

For example, evaluating `2 + 3` in postfix order works like this:

```text
Read 2:  [2]
Read 3:  [2, 3]
Read +:  pop 3 and 2, add them, then push 5 -> [5]
```

This is one possible evaluation strategy. It differs from the current
implementation, which first constructs an AST and then recursively evaluates
that tree.

## 2. The `Machine` implementation

The evaluator implements a trait named `Machine`:

```rust
impl Machine for Evaluator {
    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }
    // Operator methods...
}
```

A trait defines methods that a type promises to provide. In this design, the
parser can use the `Machine` methods without needing to know the details of the
`Evaluator` type.

The `&mut self` receiver means these methods are allowed to change the
evaluator's stack. `push` places a parsed number on the stack. The operator
methods (`add`, `sub`, `mul`, `div`, and `exp`) use the shared `binary` helper
to perform their operation.

The `Machine` trait's definition is not present in the current source tree, so
the exact contract between it and `Parser` cannot be verified from this file
alone.

## 3. Applying a binary operator

All binary operators use this helper:

```rust
fn binary<F>(&mut self, f: F) -> Result<()>
where
    F: FnOnce(f64, f64) -> f64,
{
    let a = self.pop()?;
    let b = self.pop()?;
    let r = f(b, a);
    self.stack.push(r);
    Ok(())
}
```

The helper accepts a closure `f`, which is a function-like value describing
the operation to perform. For addition, the caller supplies `|a, b| a + b`;
for multiplication, it supplies `|a, b| a * b`.

The helper pops two values from the stack. The first value popped is called
`a`, and the second is called `b`. It then calls `f(b, a)`. That order is
important for subtraction, division, and exponentiation: the earlier stack
value is the left operand, and the later value is the right operand.

If the stack does not contain two values, `pop` returns an error and `?`
immediately returns that error from `binary`. Otherwise, the result is pushed
back onto the stack.

For example, the stack `[10.0, 4.0]` followed by subtraction becomes:

```text
pop a = 4.0
pop b = 10.0
calculate b - a = 10.0 - 4.0
push 6.0
```

## 4. Operator methods

The `Machine` methods select the operation passed to `binary`:

```rust
fn add(&mut self) -> Result<()> {
    self.binary(|a, b| a + b)
}

fn sub(&mut self) -> Result<()> {
    self.binary(|a, b| a - b)
}
```

Multiplication and division work in the same way. `exp` uses `f64::powf`:

```rust
fn exp(&mut self) -> Result<()> {
    self.binary(|a, b| a.powf(b))
}
```

So if the stack has a base followed by an exponent, the helper calls
`powf(base, exponent)`. Although this file supports an exponent operation, the
active recursive-descent parser and its AST currently do not support `^`.

These methods return `Result<()>`: they do not produce a value directly for
the caller, but they may fail if there are not enough operands on the stack.
The result of the arithmetic operation is stored back on the stack.

## 5. Removing a value and reporting missing operands

The private `pop` helper is:

```rust
fn pop(&mut self) -> Result<f64> {
    self.stack.pop().ok_or(Error::MissingOperand)
}
```

`Vec::pop()` returns an `Option<f64>` because the vector might be empty.
`ok_or(...)` turns that into a `Result`: a value becomes `Ok(value)`, while an
empty stack becomes `Err(Error::MissingOperand)`.

The `Error` type and the `MissingOperand` variant are not defined in the
current crate, another indication that this file is not connected to the
current implementation.

## 6. Creating and running an evaluation

`Evaluator::new` creates an evaluator with an empty stack:

```rust
pub fn new() -> Evaluator {
    Evaluator { stack: Vec::new() }
}
```

Its `eval` method creates a parser and asks it to parse the input while using
the evaluator as the machine:

```rust
pub fn eval(&mut self, expr: &str) -> Result<f64> {
    let result = {
        let mut parser = Parser::new(self);
        parser.parse(expr)
    };

    // Check parser result and stack...
}
```

The braces create a smaller scope for `parser`. The parser borrows `self`
mutably, so this scope ensures that the parser is dropped before the method
inspects `self.stack` again.

After parsing, the method checks two things:

1. Did parsing succeed?
2. Does the stack contain exactly one result?

If parsing succeeded but the stack length is not one, it reports
`MissingOperand`. If there is exactly one value, it pops and returns that
value. If parsing failed, it returns the parser's error.

The stack-length check catches cases where evaluation did not leave one
complete result. The exact malformed inputs that lead to each stack shape
depend on the missing `Parser` implementation.

## 7. The public convenience function

At the end of the file is a function that creates a fresh evaluator for each
expression:

```rust
pub fn eval(expr: &str) -> Result<f64> {
    Evaluator::new().eval(expr)
}
```

This means callers would not need to construct an `Evaluator` themselves.
They could call `eval("2 + 3")` and receive a `Result<f64>`. However, since
this module is not declared by the current `src/lib.rs`, this function is not
currently available as `expr::eval`.

The active library does have a function named `expr::eval`, but its signature
is different: it also accepts a map of variable values and evaluates the AST
produced by the recursive-descent parser.

## 8. How this differs from the current evaluator

The active evaluation path in this project is:

```text
expression text
    -> Tokenizer
    -> recursive-descent Parser
    -> ExprTree
    -> ExprTree::eval(variable_map)
    -> f64 or evaluation error
```

The `ExprTree::eval` method uses a `match` on AST nodes and recursively
evaluates their children. By contrast, `src/eval.rs` describes an evaluator
that receives parser callbacks and stores intermediate values on a stack.

Keeping this distinction clear is useful when learning: both approaches can
evaluate expressions, but they use different parser interfaces and different
intermediate representations. To understand the evaluator currently used by
the library, read the Evaluation section in
[`docs/tree/README.md`](../tree/README.md).
