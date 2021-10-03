use crate::lexer;

#[test]
pub fn default() {
    let lexer = lexer::Lexer::build(
        &['o']
    );

    assert_eq!(lexer.current_char().unwrap(), 'o');
}