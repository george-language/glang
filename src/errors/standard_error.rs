use crate::lexing::position::Position;
use std::fmt::Display;

pub struct StandardError {
    pub text: String,
    pub position: Position,
    pub help: Option<String>,
}

impl StandardError {
    pub fn new(text: String, position: Position, help: Option<String>) -> Self {
        StandardError {
            text: text,
            position: position,
            help: help,
        }
    }

    pub fn formatted_code(text: &str, pos_start: Position, pos_end: Position) {}
}

impl Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(
            format!(
                "error: {}\nwhere: \n    line {},\n    column {},\n    in {}",
                self.text, self.position.line_num, self.position.column_num, self.position.filename
            )
            .as_str(),
        );

        if let Some(msg) = self.help.clone() {
            output.push_str(format!("\nhelp: {}", msg).as_str());
        }

        // this will print the underscores indicating where the issue is
        // output.push_str(format!("\n{}", ).as_str());

        write!(f, "{}", output)
    }
}
