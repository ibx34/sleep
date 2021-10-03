use crate::lexer::{Error, ErrorKind, Lexer};

#[test]
pub fn default() {
  let mut lexer = Lexer { source: &['[', ']'],
                          idx: 0,
                          current: ' ' };

  assert_eq!(lexer.current(), Some(&'['));
  lexer.advance(None);
  assert_eq!(lexer.current(), Some(&']'));
}

#[test]
pub fn current_checked() {
  let mut lexer = Lexer { source: &['[', ']'],
                          idx: 0,
                          current: ' ' };

  assert_eq!(lexer.current(), Some(&'['));
  lexer.advance(None);
  assert_eq!(lexer.current(), Some(&']'));

  // Error should happen here
  lexer.advance(None);
  assert_eq!(lexer.current_checked().unwrap_err(), Error { error_kind: ErrorKind::FailedToFindCharacter,
                                                           string_pos: 2 })
}
