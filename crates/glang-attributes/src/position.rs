#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub line_num: usize,
    pub column_num: usize,
}

impl Position {
    pub fn new(index: usize, line_num: usize, column_num: usize) -> Self {
        Self {
            index,
            line_num,
            column_num,
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) -> Self {
        self.index += 1;
        self.column_num += 1;

        if let Some(character) = current_char
            && character == '\n'
        {
            self.line_num += 1;
            self.column_num = 0;
        }

        self.clone()
    }
}
