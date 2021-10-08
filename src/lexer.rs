use crate::tokens::{TPos, ToT, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
  UnkownEoF,
  EoF,
  UnknownCharacter(char),
  FailedToFindCharacter,
}

#[derive(Debug, PartialEq, Eq)]
pub struct r#Error {
  pub error_kind: ErrorKind,
  pub string_pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
  Token(Token),
  Error(Error),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineInfo {
  pub last_line: usize,
  pub current_line: usize,
  pub last_spaces: usize,
}

pub struct Lexer<'a> {
  pub source: &'a Vec<char>,
  pub current: char,
  pub idx: usize,
  pub results: Vec<Option<Atom>>,
  pub line_info: LineInfo,
}

impl<'a> Lexer<'a> {
  pub fn lex(&mut self) {
    let spaces = self.determine_indent();

    if spaces > self.line_info.last_spaces && spaces != 0 && self.line_info.last_line != self.line_info.current_line {
      self.create_token(ToT::Indent);
      self.line_info.last_spaces = spaces;
    } else if spaces < self.line_info.last_spaces && spaces != 0 && self.line_info.last_line != self.line_info.current_line {
      if let Some(last) = self.latest() {
        match last {
          Atom::Token(token) => {
            if !token.position.line == self.line_info.current_line {
              self.create_token(ToT::Dedent);
            }
          }
          Atom::Error(_) => {}
        }
      }

      self.line_info.last_spaces = spaces;
    }

    let current_idx = self.idx;
    let c = match self.current() {
      Some(c) => c,
      None => return,
    };

    match c {
      &'\n' => {
        self.line_info.last_line = self.line_info.current_line;
        self.line_info.current_line += 1;
      }

      &'#' => {
        self.skip_comments();
      }

      &'(' | ')' => {
        self.create_token(c.into());
      }

      &':' => self.create_token(ToT::Colon),

      _ => {
        if c.is_alphabetic() || c == &'_' {
          while let Some(c) = self.peek(None) {
            if !c.is_ascii_alphabetic() && c != &'_' {
              break;
            }
            self.advance(None);
          }

          if let Some(keyword) = self.source.get(current_idx..self.idx + 1) {
            let actual_keyword = keyword.iter().collect::<String>();
            self.create_token(actual_keyword.into());
          }
        } else {
          self.create_error(ErrorKind::UnknownCharacter(*c));
        }
      }
    };
  }

  pub fn skip_comments(&mut self) {
    while !self.eof(None) {
      if self.current() == Some(&'\n') {
        break;
      }
      self.advance(None);
    }
  }

  pub fn create_token(&mut self, ty: ToT) {
    self.results.push(Some(Atom::Token(Token {
      ty,
      position: TPos {
        index: self.idx,
        line: self.line_info.current_line,
      },
    })));
  }

  pub fn create_error(&mut self, error: ErrorKind) {
    self.results.push(Some(Atom::Error(Error {
      error_kind: error,
      string_pos: self.idx,
    })));
  }

  pub fn lex_all(&mut self) {
    while !self.eof(None) {
      self.lex();
      self.advance(None);
    }
  }

  pub fn determine_indent(&mut self) -> usize {
    let mut spaces_encountered = 0;

    while !self.eof(None) {
      let cc = self.current();

      if cc.is_none() || cc != Some(&' ') && cc != Some(&'\t') {
        return spaces_encountered;
      }

      spaces_encountered += 1;
      self.advance(None);
    }

    spaces_encountered
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

  #[allow(clippy::should_implement_trait)]
  #[must_use = "If you would just like to lex, without getting the latest token, just run `.lex()`."]
  pub fn next(&mut self) -> &Option<Atom> {
    self.lex();
    self.latest()
  }

  pub fn latest(&self) -> &Option<Atom> { self.results.last().unwrap_or(&None) }

  pub fn peek(&self, amount: Option<usize>) -> Option<&'a char> { self.source.get(self.idx + amount.unwrap_or(1)) }

  pub fn current(&self) -> Option<&'a char> { self.source.get(self.idx) }

  pub fn current_unchecked(&self) -> char { self.source[self.idx] }

  pub fn current_checked(&self) -> Result<char, Error> {
    if self.idx > self.source.len() {
      return Err(Error {
        error_kind: ErrorKind::EoF,
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
