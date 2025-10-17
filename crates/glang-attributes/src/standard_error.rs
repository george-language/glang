use crate::position::Position;
use simply_colored::*;
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub struct StandardError {
    pub text: String,
    pub pos_start: Rc<Position>,
    pub pos_end: Rc<Position>,
    pub help: Option<String>,
    pub error_propagates: bool,
}

impl StandardError {
    pub fn new(
        text: &str,
        pos_start: Rc<Position>,
        pos_end: Rc<Position>,
        help: Option<&str>,
    ) -> Self {
        Self {
            text: text.to_string(),
            pos_start,
            pos_end,
            help: help.map(|h| h.to_string()),
            error_propagates: false,
        }
    }

    pub fn format_code_as_messup(
        &self,
        text: &str,
        pos_start: &Position,
        pos_end: &Position,
    ) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        for i in pos_start.line_num..=pos_end.line_num {
            if let Some(line) = lines.get(i as usize) {
                result.push_str(&format!(" {BOLD}{:>3} |{RESET} {}\n", i + 1, line));

                let line_len = line.len();
                let col_start = pos_start.column_num.clamp(0, line_len as isize) as usize;
                let col_end = pos_end.column_num.clamp(0, line_len as isize) as usize;
                let arrow_len = (col_end.saturating_sub(col_start)).max(1);
                let mut arrow_line = " ".repeat(col_start) + &"^".repeat(arrow_len);

                if let Some(msg) = &self.help {
                    arrow_line.push_str(&format!(" {DIM_GREEN}help:{RESET} {}", msg));
                }

                result.push_str(&format!("     {BOLD}|{RESET} {}", arrow_line));
            }
        }

        result.replace('\t', "")
    }
}

impl Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        output.push_str(&format!(
            "{BOLD}{DIM_RED}error:{RESET} {}\n    {BOLD}-->{RESET} {}:{}:{}\n",
            self.text,
            self.pos_start.filename.to_string_lossy(),
            self.pos_start.line_num + 1,
            self.pos_start.column_num + 1
        ));

        output.push_str(&format!(
            "     {BOLD}|{RESET}\n{}\n",
            self.format_code_as_messup(
                &self.pos_start.file_contents,
                &self.pos_start,
                &self.pos_end
            )
        ));

        write!(f, "{output}")
    }
}
