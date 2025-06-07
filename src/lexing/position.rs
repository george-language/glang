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
        filename: String,
        file_contents: String,
    ) -> Self {
        Position {
            index: index,
            line_num: line_num,
            column_num: column_num,
            filename: filename,
            file_contents: file_contents,
        }
    }

    pub fn advance(&mut self, current_char: char) -> Self {
        self.index += 1;
        self.column_num += 1;

        if current_char == '\n' {
            self.line_num += 1;
            self.column_num = 0;
        }

        Position::new(
            self.index,
            self.line_num,
            self.column_num,
            self.filename.clone(),
            self.file_contents.clone(),
        )
    }

    pub fn copy(&self) -> Position {
        Position::new(
            self.index,
            self.line_num,
            self.column_num,
            self.filename.clone(),
            self.file_contents.clone(),
        )
    }
}
