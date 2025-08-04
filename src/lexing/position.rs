#[derive(Debug, Clone)]
pub struct Position {
    pub index: isize,
    pub line_num: isize,
    pub column_num: isize,
    pub filename: String,
    pub file_contents: String,
}

impl Position {
    pub fn new(
        index: isize,
        line_num: isize,
        column_num: isize,
        filename: &str,
        file_contents: &str,
    ) -> Self {
        Self {
            index,
            line_num,
            column_num,
            filename: filename.to_string(),
            file_contents: file_contents.to_string(),
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) -> Self {
        self.index += 1;
        self.column_num += 1;

        if let Some(character) = current_char {
            if character == '\n' {
                self.line_num += 1;
                self.column_num = 0;
            }
        }

        self.clone()
    }
}
