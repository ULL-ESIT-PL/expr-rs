// Execute with:
// cargo run  --example evaluate
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