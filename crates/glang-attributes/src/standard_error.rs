use crate::Span;
use simply_colored::*;
use std::{fmt::Display, fs};

#[derive(Clone)]
pub struct StandardError {
    pub text: String,
    pub contents: Option<String>,
    pub span: Span,
    pub help: Option<String>,
    pub error_propagates: bool,
}

impl StandardError {
    pub fn new(text: &str, span: Span, help: Option<&str>) -> Self {
        Self {
            text: text.to_string(),
            contents: None,
            span,
            help: help.map(|h| h.to_string()),
            error_propagates: false,
        }
    }

    pub fn format_code_as_messup(&self, text: &str, span: &Span) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();

        for i in span.start.line_num..=span.end.line_num {
            if let Some(line) = lines.get((i - 1) as usize) {
                result.push_str(&format!(" {BOLD}{:>3} |{RESET} {}\n", i, line));

                let col_start = span.start.column_num.saturating_sub(1) + 1;
                let col_end = span.end.column_num.saturating_sub(1) + 1;
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

        let contents = if let Some(c) = &self.contents {
            c.to_owned()
        } else {
            match fs::read_to_string(&self.span.filename) {
                Ok(contents) => contents,
                Err(_) => String::new(),
            }
        };

        output.push_str(&format!(
            "{BOLD}{DIM_RED}error:{RESET} {}\n    {BOLD}-->{RESET} {}:{}:{}\n",
            self.text,
            self.span.filename.to_string_lossy(),
            self.span.start.line_num,
            self.span.start.column_num
        ));

        output.push_str(&format!(
            "     {BOLD}|{RESET}\n{}\n",
            self.format_code_as_messup(&contents, &self.span)
        ));

        write!(f, "{output}")
    }
}
