# Understanding the Parser

This tutorial explains how the recursive-descent parser in
[`src/parser.rs`](../../src/parser.rs) turns a sequence of tokens into an
expression tree. It assumes that you are new to Rust, but familiar with
arithmetic expressions and functions.

The tokenizer is described separately in
[`docs/tokens/README.md`](../tokens/README.md), and the tree type is defined in
[`src/tree.rs`](../../src/tree.rs).

## 1. The parser's job

The tokenizer reads characters and produces tokens. For example:

```text
2 * x + 19
```

becomes something like:

```text
Float(2.0), Star, Symbol("x"), Plus, Float(19.0)
```

The parser reads those tokens and builds an abstract syntax tree (AST). For
this expression, the structure is equivalent to:

```text
Add
├── Mul
│   ├── Float(2.0)
│   └── Var("x")
└── Float(19.0)
```

The tree records the meaning and grouping of the expression, rather than its
original spacing or exact spelling.

## 2. The grammar

The parser's grammar is written in comments above `parse`:

```text
expr   ::= term (("+" | "-") term)*
term   ::= factor (("*" | "/") factor)*
factor ::= ("-" | "+")? (number | variable | "(" expr ")")
```

Read `*` after a group as “zero or more repetitions,” and `?` as “optional.”
In other words:

- An `expr` is one `term`, followed by zero or more additions or subtractions.
- A `term` is one `factor`, followed by zero or more multiplications or
  divisions.
- A `factor` is an optional unary sign followed by a number, variable, or
  parenthesized expression.

This division into grammar rules gives multiplication and division higher
precedence than addition and subtraction. For example, `2 + 3 * 4` is parsed as
`2 + (3 * 4)`, not `(2 + 3) * 4`.

The grammar is designed to be parsed by looking ahead at the next token. It
does not need to guess between multiple possible parses.

## 3. Recursive descent: one function per rule

The implementation follows the grammar with three functions:

| Grammar rule | Parser function |
| --- | --- |
| `expr` | `expr_rule` |
| `term` | `term_rule` |
| `factor` | `factor_rule` |

Each function parses its own kind of expression and calls the function for the
next smaller rule. This is called **recursive descent**.

The calls normally go down through `expr_rule`, `term_rule`, and
`factor_rule`. When `factor_rule` finds an opening parenthesis, it calls
`expr_rule` again to parse the expression inside the parentheses. That call
back to an earlier rule is the recursion.

## 4. Starting with `parse`

The public entry point is:

```rust
pub fn parse(text: &str) -> Result<ExprTree> {
    let mut tokens = Tokenizer::new(text);
    let tree = expr_rule(&mut tokens);
    match tokens.next() {
        None => tree,
        Some(tok) => Err(Error::UnexpectedToken {
            token: tok,
            rule: "expr",
            expect: "end of input",
        }),
    }
}
```

It creates a `Tokenizer`, then asks `expr_rule` to parse a complete expression.
The tokenizer is passed as `&mut`: the parser functions share and advance the
same iterator rather than each making a copy.

After parsing, `parse` checks that there are no tokens left. This matters
because otherwise a valid prefix could be mistaken for a valid complete
expression. For example, `10 20` starts with a valid expression (`10`), but
leaves the `20` token unconsumed, so the input is rejected.

The return type is `Result<ExprTree>`. It is an alias for
`std::result::Result<ExprTree, parser::Error>`, so the function returns either
an AST (`Ok(tree)`) or a parser error (`Err(error)`).

## 5. Parsing addition and subtraction with `expr_rule`

The first part of `expr_rule` parses a term:

```rust
let mut tree = term_rule(tokens)?;
```

The `?` operator means: if `term_rule` returns an error, return that error
from `expr_rule` immediately. Otherwise, unwrap the successful value and
continue. The `mut` keyword is needed because the tree will be extended as more
operators are read.

Next, the function checks whether the next token is `+` or `-`:

```rust
while let Some(Token::Plus) | Some(Token::Minus) = tokens.peek() {
    // ...
}
```

`peek()` looks at the next token without consuming it. The `while let` loop
continues only while that token matches one of the two patterns.

Inside the loop, the parser consumes the operator with `tokens.next()`, parses
the right-hand side as another term, and creates a tree node:

```rust
tree = ExprTree::Add(Box::new(tree), Box::new(rhs));
```

For subtraction it creates `ExprTree::Sub` instead. `Box` stores each child
expression behind a pointer; this allows the recursive `ExprTree` enum to have
a finite size.

The existing tree becomes the left-hand child each time through the loop. As a
result, `10 - 3 - 2` is grouped as `(10 - 3) - 2`. This is **left
associativity**.

## 6. Parsing multiplication and division with `term_rule`

`term_rule` follows the same pattern as `expr_rule`, but works with factors and
looks for `*` or `/`:

1. Parse the first factor.
2. While the next token is `*` or `/`, consume the operator.
3. Parse the next factor.
4. Replace the current tree with a `Mul` or `Div` node.

Because `expr_rule` asks `term_rule` to parse each operand before handling
`+` or `-`, multiplication and division bind more tightly. For `2 + 3 * 4`,
the `term_rule` call consumes `3 * 4` and returns a `Mul` tree. `expr_rule`
then combines that tree with `2` using `Add`.

Like addition and subtraction, multiplication and division are left
associative: `12 / 3 / 2` is grouped as `(12 / 3) / 2`.

## 7. Parsing values, signs, and parentheses with `factor_rule`

`factor_rule` handles the simplest expressions. It first consumes one token:

```rust
let tok = tokens.next();
```

It then checks for an optional unary sign. A minus sign sets `negate` to
`true`; a plus sign leaves it `false`. In either case, the function consumes
the following token, which must be the value being signed.

The next `match` handles the value:

- `Token::Float(number)` becomes `ExprTree::Float(number)`.
- `Token::Symbol(name)` becomes `ExprTree::Var(name)`.
- `Token::Open` starts a parenthesized expression.
- Any other token is unexpected.
- `None` means the input ended before a factor was found.

For an opening parenthesis, the function calls `expr_rule(tokens)` to parse the
inner expression. It then consumes and checks for the closing `)` token. The
parentheses affect how the tree is grouped, but they do not need their own
AST node: the inner expression itself is returned.

Finally, if `negate` is true, the parsed value is wrapped in
`ExprTree::Neg(Box::new(result))`. Thus `-x` becomes a `Neg` node containing a
`Var("x")`.

## 8. Errors and `Result`

The parser defines two error cases:

```rust
pub enum Error {
    UnexpectedEndOfInput {
        rule: &'static str,
        expect: &'static str,
    },
    UnexpectedToken {
        token: Token,
        rule: &'static str,
        expect: &'static str,
    },
}
```

`UnexpectedEndOfInput` means the input ended before the parser found something
required, such as an operand after `+` or a closing parenthesis. `UnexpectedToken`
means a token was present, but was not valid at that point.

The `rule` and `expect` fields help explain where parsing failed and what was
expected. `Display` formats these errors as readable messages. Implementing
`std::error::Error` also lets this error work with Rust's standard error-handling
ecosystem.

The `?` operator makes error propagation concise. For example, if parsing the
right-hand term fails inside `expr_rule`, the error is returned to its caller;
it can eventually be returned from `parse`.

## 9. Reading and extending the tests

The tests at the end of `parser.rs` check both successful and unsuccessful
parses.

The `check` helper compares the result of parsing with an expected tree:

```rust
fn check(expr: &str, tree: ExprTree) {
    assert_eq!(parse(expr), Ok(tree));
}
```

For example, the test for `10 + 12 * 20 - 2` expects a `Sub` node whose left
child is an `Add`; that `Add` contains a `Mul` as its right child. This checks
precedence and left associativity together.

The invalid-input tests use `assert_matches!` to check the kind of error
returned, and sometimes inspect its fields. When changing the grammar, update
or add tests for both the resulting tree and invalid expressions.

## 10. A complete walkthrough

Consider `2 * (x - 9) + 1`:

1. `parse` creates the tokenizer and calls `expr_rule`.
2. `expr_rule` calls `term_rule` for the left side of the addition.
3. `term_rule` parses `2` as a factor, sees `*`, and parses another factor.
4. That factor sees `(`, so it calls `expr_rule` to parse `x - 9`.
5. The inner `expr_rule` returns a `Sub(Var("x"), Float(9.0))` tree. The
   factor consumes `)` and returns that subtree.
6. The outer `term_rule` combines `2` and the parenthesized subtree in a
   `Mul` node.
7. The outer `expr_rule` sees `+`, parses `1` as another term, and creates an
   `Add` node.
8. `parse` confirms that the input has no extra tokens and returns the tree.

The result is equivalent to:

```text
Add
├── Mul
│   ├── Float(2.0)
│   └── Sub
│       ├── Var("x")
│       └── Float(9.0)
└── Float(1.0)
```

## 11. Limits of this parser

The parser implements addition, subtraction, multiplication, division, unary
plus and minus, numbers, variables, and parentheses. The tokenizer also
recognizes `^` as `Token::Power`, but this parser has no grammar rule for
exponentiation, so `^` is not a supported operator here.

The tokenizer represents unknown characters as `Token::Error`; the parser
reports such a token as unexpected. The exact error message may depend on which
grammar rule encounters it.

When studying error handling, note that `parse` calls `tokens.next()` after
`expr_rule` returns, even if `expr_rule` returned an error. If malformed input
leaves another token behind, this final check can report that remaining token
instead of the earlier parsing error. This is an implementation detail to keep
in mind when interpreting error tests.
