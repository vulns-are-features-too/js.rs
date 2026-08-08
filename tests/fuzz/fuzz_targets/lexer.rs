#![no_main]

use std::str::from_utf8;

use libfuzzer_sys::fuzz_target;
use syntax::lexing::{lexer::lex, tokens::Token};

fuzz_target!(|data: &[u8]| {
    let input = data
        .iter()
        .map(|x| x & (u8::MAX >> 1)) // all <=127 for UTF8
        .collect::<Vec<u8>>();
    let input = from_utf8(&input).expect("Not UTF8");
    let mut last_token = None;
    for token in lex(input) {
        last_token = Some(token);
    }
    assert_eq!(Some(Token::Eof), last_token);
});
