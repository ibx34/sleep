pub struct Lexer<'a> {
    pub source: &'a [char],
    pub index: usize,
}

impl<'a> Lexer<'a> {
    pub fn build(source: &'a [char]) -> Lexer {
        Lexer {
            source,
            index: 0
        }
    }
    
    pub fn check_eof(&self, index: Option<usize>) -> bool {
        index.unwrap_or(self.index) > self.source.len()
    }

    pub fn advance_by(&mut self, amount: usize) -> bool { 
        let possible_next_index = self.index + amount;
        
        if self.check_eof(Some(possible_next_index)) {
            return false;
        }

        self.index = possible_next_index;
        true
    }

    pub fn advance(&mut self) -> bool { 
        let possible_next_index = self.index + 1;
        
        if self.check_eof(Some(possible_next_index)) {
            return false;
        }

        self.index = possible_next_index;
        true
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.index + 1).copied()
    }

    pub fn current_char(&self) -> Option<char> {
        self.source.get(self.index).copied()
    }
} 