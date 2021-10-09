#[macro_use(read_python_file)]
extern crate sleep;

use sleep::{lexer::*, tokens::ToT};

fn main() {
	let mut lexer = Lexer {
		source: &read_python_file!("indent_comments").chars().collect(),
		current: ' ',
		idx: 0,
		results: vec![],
		line_info: LineInfo {
			last_line: 1,
			current_line: 1,
			last_spaces: 0,
		},
	};

	lexer.lex_all();

	// Lets pretty print:
	let atoms_iter = lexer.results.iter();
	for atom in atoms_iter.flatten() {
		match atom {
			Atom::Token(token) => println!(
				"{}(Type: {:?}, Position: {:?})",
				if token.ty == ToT::Indent {
					"+"
				} else if token.ty == ToT::Dedent {
					"-"
				} else {
					""
				},
				token.ty,
				token.position
			),
			Atom::Error(error) => println!("ERROR (Kind: {:?}, Position: {:?})", error.error_kind, error.position),
		}
	}
}
