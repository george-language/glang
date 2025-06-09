use crate::lexing::position::Position;
use std::fmt::Display;

pub struct StandardError {
    pub text: String,
    pub pos_start: Position,
    pub pos_end: Position,
    pub help: Option<String>,
}

impl StandardError {
    pub fn new(text: String, pos_start: Position, pos_end: Position, help: Option<String>) -> Self {
        StandardError {
            text: text,
            pos_start: pos_start,
            pos_end: pos_end,
            help: help,
        }
    }

    pub fn format_code_as_messup(
        &self,
        text: &str,
        pos_start: Position,
        pos_end: Position,
    ) -> String {
        let mut result = String::new();

        let mut idx_start = text[..pos_start.index as usize]
            .rfind('\n')
            .map_or(0, |i| i + 1);
        let mut idx_end = text[idx_start..]
            .find('\n')
            .map_or(text.len(), |i| idx_start + i);

        let line_count = pos_end.line_num - pos_start.line_num + 1;

        for i in 0..line_count {
            let line = &text[idx_start..idx_end];

            let col_start = if i == 0 { pos_start.column_num } else { 0 };
            let col_end = if i == line_count - 1 {
                pos_end.column_num as usize
            } else {
                line.len().saturating_sub(1) as usize
            };

            result.push_str(line);
            result.push('\n');

            let arrow_line = " ".repeat(col_start as usize)
                + &"^".repeat(col_end.saturating_sub(col_start as usize));
            result.push_str(&arrow_line);

            result.push('\n');
            idx_start = idx_end + 1;
            idx_end = text[idx_start..]
                .find('\n')
                .map_or(text.len(), |i| idx_start + i);
        }

        result.replace('\t', "")
    }
}

impl Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(
            format!(
                "error: {}\nwhere: \n|     line {},\n|     column {},\n|     in {}",
                self.text,
                self.pos_start.line_num,
                self.pos_start.column_num,
                self.pos_start.filename
            )
            .as_str(),
        );

        // this will print the '^' indicating where the issue is
        output.push_str(
            format!(
                "\ncaused by:\n\n{}",
                self.format_code_as_messup(
                    self.pos_start.file_contents.as_str(),
                    self.pos_start.copy(),
                    self.pos_end.copy(),
                )
            )
            .as_str(),
        );

        if let Some(msg) = self.help.clone() {
            output.push_str(format!("\nhelp: {}", msg).as_str());
        }

        write!(f, "{}", output)
    }
}
