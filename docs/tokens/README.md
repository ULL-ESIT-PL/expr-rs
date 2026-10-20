# Understanding the Tokenizer

A parser does not usually work directly with the characters in the input. It is easier to first group them into meaningful pieces called **tokens**.

For example, the expression:

```text
2 * x + 19
```

can be converted into the sequence of tokens:

```text
Float(2.0), Star, Symbol("x"), Plus, Float(19.0)
```

This first step is called **tokenization** or **lexical analysis**. The parser then reads the tokens and builds a syntax tree. The task of the tokenizer is to recognize the pieces of the language. The file [/src/tokenizer.rs](/src/tokenizer.rs) contains the implementation of the tokenizer.

## 1. The `Token` enum

The file starts by defining the kinds of tokens that this tokenizer can produce:

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Error,
    Float(f64),
    Symbol(String),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Open,
    Close,
}
```

An `enum` lets us define a type that can have one of several variants.

Some variants carry extra data:

- `Float(f64)` stores a numeric value, such as `2.0`.
- `Symbol(String)` stores a variable name, such as `"x"`.

The other variants represent punctuation or operators. They do not need to store data: `Plus` represents `+`, `Open` represents `(`, and so on.

The `derive` attribute asks Rust to generate implementations for some useful traits:

- `Debug` allows values to be printed with `{:?}`, which is useful for debugging.
- `PartialEq` allows tokens to be compared in tests.
- `Clone` allows a token to be copied by cloning it.

For example:

```rust
assert_eq!(Token::Plus, Token::Plus);
```

## 2. Making tokens printable

The `Display` implementation defines how a token appears when formatted for a user:

```rust
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Plus => write!(f, "+"),
            // Other variants...
            Token::Error => write!(f, "ERROR"),
        }
    }
}
```

The `match` expression checks which variant `self` contains and chooses what to write.

For example, a `Token::Plus` is displayed as `+`, while a `Token::Float(2.0)` is displayed as `2`.

`Display` is useful when an error message needs to show the token that caused a problem.

## 3. The `Tokenizer` struct

The tokenizer stores the characters that remain to be read:

```rust
#[derive(Clone)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}
```

`Chars<'a>` is Rust's iterator over the characters of a string. The lifetime `'a` indicates that this iterator borrows text that must remain valid while the tokenizer uses it.

The tokenizer does not copy the entire input string. It keeps an iterator over the borrowed string instead.

The `Clone` derive is important because the tokenizer sometimes needs to inspect the next character without consuming it.

## 4. Creating and inspecting a tokenizer

The `new` function creates a tokenizer from an input string:

```rust
pub fn new(text: &'a str) -> Tokenizer<'a> {
    Tokenizer {
        chars: text.chars(),
    }
}
```

The call to `text.chars()` creates a character iterator over the input.

The `peek` method looks at the next token without advancing the original tokenizer:

```rust
pub fn peek(&self) -> Option<Token> {
    self.clone().next()
}
```

It clones the tokenizer, then asks the clone for its next token. Since the original was not changed, the same token can still be read later.

The return type is `Option<Token>`:

- `Some(token)` means there is another token.
- `None` means the input has ended.

## 5. Small helper methods

The tokenizer uses several private helper methods to move through the input.

### `skip`

```rust
fn skip(&mut self, count: usize) {
    for _ in 0..count {
        self.chars.next();
    }
}
```

This consumes up to `count` characters from the iterator. Here, `_` means that the loop variable exists but is not used.

### `skip_while`

```rust
fn skip_while<P>(&mut self, pred: P)
where
    P: Copy + FnMut(char) -> bool,
{
    while self.chars.clone().next().map_or(false, pred) {
        self.chars.next();
    }
}
```

This consumes characters as long as the predicate `pred` returns `true`.

For example, the tokenizer calls it with:

```rust
self.skip_while(|ch| ch.is_whitespace());
```

That skips spaces, tabs, and other whitespace characters.

The predicate is a closure: a small function written between `|` characters. Its parameter is a `char`, and it returns a `bool`.

The tokenizer clones the character iterator to inspect the next character. It consumes from the original iterator only when the predicate matches.

### `take_while`

```rust
fn take_while<P>(&mut self, pred: P) -> &'a str
where
    P: Copy + FnMut(char) -> bool,
{
    let start = self.chars.as_str();
    self.skip_while(pred);
    &start[..start.len() - self.chars.as_str().len()]
}
```

This returns the part of the input consumed while the predicate remained true.

For example, when reading `123.5`, it can collect the characters `123.5` before converting them into a number.

The method uses `Chars::as_str()` to get the unconsumed part of the original input. Subtracting the remaining length from the starting length tells it how many bytes were consumed.

There is also a `take` helper that consumes a fixed number of characters. It is currently marked `#[allow(dead_code)]` because nothing in this file uses it.

## 6. Why `Tokenizer` implements `Iterator`

The central method is `next`:

```rust
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // ...
    }
}
```

By implementing the standard `Iterator` trait, `Tokenizer` can be used like other Rust iterators. For example:

```rust
let tokens: Vec<Token> = Tokenizer::new("2 + 3").collect();
```

`type Item = Token` says that each successful call to `next` produces a `Token`. The `Option` return type allows `next` to signal that the input has ended.

## 7. How `next` reads one token

First, `next` skips whitespace:

```rust
self.skip_while(|ch| ch.is_whitespace());
```

Then it inspects the next character without consuming it:

```rust
match self.chars.clone().next() {
    // ...
}
```

The `match` handles each possible kind of character.

### Numbers

```rust
Some(ch) if ch.is_digit(10) => self
    .take_while(|ch| ch.is_digit(10) || ch == '.')
    .parse::<f64>()
    .map(Token::Float)
    .ok(),
```

If the next character is a digit, `take_while` gathers digits and decimal points. The collected text is parsed as an `f64`, Rust's 64-bit floating-point type.

If parsing succeeds, `.map(Token::Float)` wraps the value in a `Token::Float`. The final `.ok()` converts the parse result into an `Option`: a successful parse becomes `Some(token)`, and a failed parse becomes `None`.

### Variable names

```rust
Some(ch) if ch.is_alphabetic() => {
    let name = self.take_while(|c| c.is_alphabetic() || c == '_' || c.is_digit(10));
    Some(Token::Symbol(name.to_string()))
}
```

A variable must start with a letter. After the first letter, its name may contain letters, underscores, or digits.

The collected text is a string slice borrowed from the input. `.to_string()` creates an owned `String`, which is stored in the token.

For example, `total_2` becomes:

```rust
Token::Symbol("total_2".to_string())
```

### Operators and parentheses

Each operator branch consumes one character and returns its token:

```rust
Some('+') => {
    self.skip(1);
    Some(Token::Plus)
}
```

The other branches work the same way for `-`, `*`, `/`, `^`, `(`, and `)`.

The tokenizer produces `Token::Minus` for `-`; it does not decide whether the minus is a subtraction operator or a unary negative sign. That decision belongs to the parser, which has more context.

### Invalid characters and end of input

```rust
Some(_) => {
    self.skip(1);
    Some(Token::Error)
}
None => None,
```

`Some(_)` means that a character exists, but none of the earlier branches recognized it. The tokenizer consumes that character and returns `Token::Error`.

Consuming the character matters: it ensures the tokenizer makes progress instead of returning the same error forever.

`None` means there are no characters left, so the iterator returns `None` to signal the end of input.

## 8. How the tests check tokenization

The helper function converts all tokens into a vector and compares them with the expected result:

```rust
fn assert_tokens(text: &str, expected: Vec<Token>) {
    let tokenizer = Tokenizer::new(text);
    let tokens: Vec<Token> = tokenizer.collect();
    assert_eq!(tokens, expected);
}
```

The tests then describe expected behavior. For example:

```rust
assert_tokens("1@", vec![Float(1.0), Error]);
```

This checks that the valid number is recognized and the unknown character is reported as an error token.

The tests are also useful examples of how to use `collect` and how to compare vectors of enum values.

## 9. Following an example by hand

For the input:

```text
2 * x + 19
```

the tokenizer proceeds roughly as follows:

1. Reads `2`, collects it, and returns `Float(2.0)`.
2. Skips the space, reads `*`, and returns `Star`.
3. Skips the space, reads `x`, and returns `Symbol("x")`.
4. Skips the space, reads `+`, and returns `Plus`.
5. Skips the space, reads `19`, and returns `Float(19.0)`.
6. On the next call, finds no characters and returns `None`.

The parser can then use this token sequence to build an expression tree.

## 10. Things to notice while learning

- The tokenizer is an iterator, so it fits naturally into Rust's iterator tools.
- `Option` represents the possibility that there is no next token.
- `enum` represents the different kinds of token.
- `match` selects behavior based on a value's variant.
- Closures let helper methods receive rules such as “is this character whitespace?”
- The tokenizer only recognizes the pieces of the language; the parser decides how those pieces fit together.

One detail worth noticing: `Power` is recognized by the tokenizer, but the parser currently does not implement the `^` operator. Tokenization and parsing are separate steps, so recognizing a token does not automatically mean the grammar supports it. Also, the module's opening comment mentions the shunting-yard algorithm, but the parser in this project uses recursive descent; that comment may be worth updating.