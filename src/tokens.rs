#[derive(Debug, PartialEq, Eq)]
pub enum ToT {
  Hashtag,
  LeftParen,
  Ident,
}

impl From<char> for ToT {
  fn from(c: char) -> Self {
    match c {
      '#' => Self::Hashtag,
      '(' => Self::LeftParen,
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
