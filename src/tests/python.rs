use crate::{
  lexer::{Atom, Lexer},
  tests::utils::read_python_test,
  tokens::{TPos, ToT, Token},
};

#[test]
fn py_print_test() {
  let mut lexer = Lexer {
    source: &read_python_test("print").chars().collect(),
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
  let mut lexer = Lexer {
    source: &read_python_test("indent").chars().collect(),
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

  lexer.lex();
  lexer.advance(Some(2));

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 20, line: 2, indent: 1 }
    }))
  );
}

#[test]
fn py_indent_test_2() {
  let mut lexer = Lexer {
    source: &read_python_test("indent_2").chars().collect(),
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

  lexer.lex();
  lexer.advance(Some(2));

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Def,
      position: TPos { index: 18, line: 2, indent: 1 }
    }))
  );

  lexer.lex();
  lexer.advance(Some(2));

  assert_eq!(
    lexer.lex(),
    Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 39, line: 3, indent: 2 }
    }))
  );
}