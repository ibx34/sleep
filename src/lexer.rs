use crate::tokens::{TPos, ToT, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
  UnkownEoF,
  UnknownCharacter(char),
  FailedToFindCharacter,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
  pub error_kind: ErrorKind,
  pub string_pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
  Token(Token),
  Error(Error),
}

pub struct Lexer<'a> {
  pub source: &'a Vec<char>,
  pub current: char,
  pub idx: usize,
  pub line: usize,
  pub indent: usize,
}

impl<'a> Lexer<'a> {
  pub fn lex(&mut self) -> Option<Atom> {
    self.whitespace();
    while self.current() == Some(&'#') {
      let idx_before = self.idx;
      if self.eof(None) {
        return None;
      }

      self.skip_comments();
      if self.idx == idx_before {
        break;
      }
    }

    if self.peek(None) == Some(&'\n') {
      if self.peek(Some(2)) == Some(&' ') {
        self.advance(Some(3));

        if self.current() != Some(&' ') {
          self.advance(None);
        }
        self.indent += 1;
      }

      self.line += 1;
      self.advance(None);
    }

    self.whitespace();

    let current_idx = self.idx;
    let cc = match self.current() {
      Some(char) => char,
      None => return None,
    };

    Some(match cc {
      ':' => Atom::Token(Token {
        ty: ToT::Colon,
        position: TPos {
          index: self.idx,
          line: self.line,
          indent: self.indent,
        },
      }),
      '(' | ')' => Atom::Token(Token {
        ty: cc.into(),
        position: TPos {
          index: self.idx,
          line: self.line,
          indent: self.indent,
        },
      }),
      _ => {
        if cc.is_alphabetic() || cc.is_alphanumeric() {
          while !self.eof(None) {
            let character = self.current_unchecked();
            if !character.is_alphabetic() || !character.is_alphanumeric() {
              break;
            }
            self.advance(None);
          }

          if let Some(keyword) = self.source.get(current_idx..self.idx) {
            let actual_keyword = keyword.iter().collect::<String>();
            Atom::Token(Token {
              ty: actual_keyword.into(),
              position: TPos {
                index: self.idx,
                line: self.line,
                indent: self.indent,
              },
            })
          } else {
            Atom::Error(Error {
              error_kind: ErrorKind::UnknownCharacter(*cc),
              string_pos: self.idx,
            })
          }
        } else {
          Atom::Error(Error {
            error_kind: ErrorKind::UnknownCharacter(*cc),
            string_pos: self.idx,
          })
        }
      }
    })
  }

  pub fn whitespace(&mut self) {
    while !self.eof(None) {
      let character = match self.current() {
        Some(c) => c,
        None => return,
      };

      if character == &'\n' {
        self.line += 1;
      } else if character != &' ' {
        return;
      }

      self.advance(None);
    }
  }

  pub fn skip_comments(&mut self) {
    while !self.eof(None) {
      let character = match self.current() {
        Some(c) => c,
        None => return,
      };

      if character == &'\n' {
        self.line += 1;
        //self.advance(None);
        break;
      }

      self.advance(None);
    }

    self.advance(None);
  }

  pub fn eof(&self, amount: Option<usize>) -> bool {
    let idx = amount.unwrap_or(self.idx);

    if self.source.get(idx).is_some() {
      return false;
    }
    true
  }

  pub fn advance(&mut self, amount: Option<usize>) -> bool {
    let amount = amount.unwrap_or(1);

    if !self.eof(Some(amount)) {
      self.idx += amount;
      return true;
    }
    false
  }

  pub fn peek(&self, amount: Option<usize>) -> Option<&'a char> { self.source.get(self.idx + amount.unwrap_or(1)) }

  pub fn current(&self) -> Option<&'a char> { self.source.get(self.idx) }

  pub fn current_unchecked(&self) -> char { self.source[self.idx] }

  pub fn current_checked(&self) -> Result<char, Error> {
    if self.idx > self.source.len() {
      return Err(Error {
        error_kind: ErrorKind::UnkownEoF,
        string_pos: self.idx,
      });
    }

    let character = self.source.get(self.idx);
    if let Some(character) = character {
      Ok(*character)
    } else {
      Err(Error {
        error_kind: ErrorKind::FailedToFindCharacter,
        string_pos: self.idx,
      })
    }
  }
}
