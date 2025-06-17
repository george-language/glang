use crate::lexing::position::Position;
use simply_colored::*;
use std::fmt::Display;

#[derive(Clone)]
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

        let text_len = text.len();

        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        for i in pos_start.line_num..=pos_end.line_num {
            if let Some(line) = lines.get(i as usize) {
                result.push_str(line);
                result.push('\n');

                let col_start = if i == pos_start.line_num {
                    pos_start.column_num
                } else {
                    0
                };

                let col_end = if i == pos_end.line_num {
                    pos_end.column_num as usize
                } else {
                    line.len()
                };

                let arrow_line = " ".repeat(col_start as usize)
                    + &"^".repeat(col_end.saturating_sub(col_start as usize));
                result.push_str(&arrow_line);
                result.push('\n');
            }
        }

        result.replace('\t', "")
    }
}

impl Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(
            format!(
                "{BG_RED}{BOLD}error: {}{RESET}\nwhere: \n|     line {},\n|     column {},\n|     in {}",
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
                    self.pos_start.clone(),
                    self.pos_end.clone(),
                )
            )
            .as_str(),
        );

        if let Some(msg) = self.help.clone() {
            output.push_str(format!("\n{BG_GREEN}help: {}", msg).as_str());
        }

        write!(f, "{}{RESET}", output)
    }
}
