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
	Dash,
	Quote,
	ArrowRight,
	ArrowLeft,
	MoreThan,
	LessThan,
	Comma,
	/// Token: Indent  
	Indent,
	/// Token: Dedent  
	Dedent,
	/// Token: Newline
	Newline,
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
			'-' => Self::Dash,
			'>' => Self::MoreThan,
			'<' => Self::LessThan,
			'"' => Self::Quote,
			'\n' => Self::Newline,
			_ => Self::Unkown,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct TPos {
	pub index: usize,
	pub line: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
	pub ty: ToT,
	pub position: TPos,
}
