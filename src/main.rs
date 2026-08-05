mod syntax;

use crate::syntax::parsing::ast;
use std::str::FromStr;

fn main() {
    let filename = std::env::args().next().expect("provide js file");
    let content = std::fs::read_to_string(filename).expect("failed to read js file");
    let ast = ast::Ast::from_str(&content).unwrap();
    println!("{ast:?}");
}
