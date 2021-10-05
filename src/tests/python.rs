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
    indent: 0,
  };

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 5, line: 1, indent: 0 }
    }))
  );

  lexer.advance(Some(2));

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 35, line: 4, indent: 0 }
    }))
  );
}

#[test]
fn py_indent_test() {
  let mut file = File::open("src/tests/python/indent.py").unwrap();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();

  let mut lexer = Lexer {
    source: &buffer.chars().collect(),
    current: ' ',
    idx: 0,
    line: 1,
    indent: 0,
  };

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Def,
      position: TPos { index: 3, line: 1, indent: 0 }
    }))
  );

  // Process the function name
  lexer.lex();
  // Skip function's ()
  lexer.advance(Some(2));
  // Eat the colon and indent the file
  lexer.lex();

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 20, line: 2, indent: 1 }
    }))
  );
}
