#[derive(Debug, PartialEq, Eq)]
pub enum ToT {
  Hashtag,
  LeftParen,
  RightParen,
  Ident,
  /// Keyword: Print: print
  /// print(<value>)
  Print,
}

impl From<String> for ToT {
  fn from(c: String) -> Self {
    match c.as_str() {
      "print" => Self::Print,
      _ => Self::Ident,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TPos {
  pub index: usize,
  pub line_: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
  pub ty: ToT,
  pub position: TPos,
}
