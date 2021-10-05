use std::{fs::File, io::Read};

use crate::{
  lexer::{Atom, Lexer},
  tokens::{TPos, ToT, Token},
};

#[test]
fn py_print_test() {
  let mut file = File::open("src/tests/python/print.py").unwrap();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();

  let mut lexer = Lexer {
    source: &buffer.chars().collect(),
    current: ' ',
    idx: 0,
    line: 1,
  };

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 5, line_: 1 }
    }))
  )
}
