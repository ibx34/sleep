#[macro_use(read_python_file)]
extern crate sleep;

use sleep::lexer::*;

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

  println!("{:#?}", lexer.results);
}
