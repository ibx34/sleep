use crate::{
  lexer::{Atom, Error, ErrorKind, Lexer, LineInfo},
  tokens::{TPos, ToT, Token},
};

#[test]
pub fn default() {
  let mut lexer = Lexer {
    source: &vec!['[', ']'],
    idx: 0,
    current: ' ',
    results: vec![],
    line_info: LineInfo {
      last_line: 1,
      current_line: 1,
      last_spaces: 0,
    },
  };

  assert_eq!(lexer.current(), Some(&'['));
  lexer.advance(None);
  assert_eq!(lexer.current(), Some(&']'));
}

#[test]
pub fn current_checked() {
  let mut lexer = Lexer {
    source: &vec!['[', ']'],
    idx: 0,
    current: ' ',
    results: vec![],
    line_info: LineInfo {
      last_line: 1,
      current_line: 1,
      last_spaces: 0,
    },
  };

  assert_eq!(lexer.current(), Some(&'['));
  lexer.advance(None);
  assert_eq!(lexer.current(), Some(&']'));

  // Error should happen here
  lexer.advance(None);
  assert_eq!(lexer.current_checked().unwrap_err(), Error {
    error_kind: ErrorKind::FailedToFindCharacter,
    string_pos: 2
  })
}

#[test]
pub fn eof() {
  let lexer = Lexer {
    source: &vec!['[', ']'],
    idx: 0,
    current: ' ',
    results: vec![],
    line_info: LineInfo {
      last_line: 1,
      current_line: 1,
      last_spaces: 0,
    },
  };

  assert_eq!(lexer.eof(Some(2)), true)
}

#[test]
pub fn lexer_lex() {
  let mut lexer = Lexer {
    source: &vec!['#', 's', 'd', 'b', 's', 'b', 's', '\n', '('],
    idx: 0,
    current: ' ',
    results: vec![],
    line_info: LineInfo {
      last_line: 1,
      current_line: 1,
      last_spaces: 0,
    },
  };

  lexer.lex();

  assert_eq!(
    lexer.next(),
    &Some(Atom::Token(Token {
      ty: ToT::LeftParen,
      position: TPos { index: 8, line: 2 }
    }))
  )
}

#[test]
pub fn lexer_keyword() {
  let mut lexer = Lexer {
    source: &vec!['p', 'r', 'i', 'n', 't'],
    idx: 0,
    current: ' ',
    results: vec![],
    line_info: LineInfo {
      last_line: 1,
      current_line: 1,
      last_spaces: 0,
    },
  };

  assert_eq!(
    lexer.next(),
    &Some(Atom::Token(Token {
      ty: ToT::Print,
      position: TPos { index: 5, line: 1 }
    }))
  )
}
