#[derive(Debug, PartialEq, Eq)]
pub enum ToT {
  Hashtag,
  LeftParen,
  RightParen,
  Ident,
  /// Keyword: Print: print
  /// print(<value>)
  Print,
  /// Keyword: Def: def
  /// def <name>(...)
  Def,
  Unkown,
  Colon,
  SemiColon,
  Comma,
}

impl From<String> for ToT {
  fn from(c: String) -> Self {
    match c.as_str() {
      "print" => Self::Print,
      "def" => Self::Def,
      _ => Self::Ident,
    }
  }
}

impl From<&char> for ToT {
  fn from(c: &char) -> Self {
    match c {
      ')' => Self::RightParen,
      '(' => Self::LeftParen,
      ':' => Self::Colon,
      ';' => Self::SemiColon,
      ',' => Self::Comma,
      _ => Self::Unkown,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TPos {
  pub index: usize,
  pub line: usize,
  pub indent: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
  pub ty: ToT,
  pub position: TPos,
}
