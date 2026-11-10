use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Syntax error on line {line}: {message}")]
    InvalidSyntax { line: usize, message: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Delay(u64),
    Move { x: i32, y: i32 },
    Click { x: Option<i32>, y: Option<i32> },
    Type(String),
    Key(String),
    Repeat { count: usize, commands: Vec<Command> },
}

pub fn parse_script(content: &str) -> Result<Vec<Command>, ParseError> {
    let mut commands = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with("DELAY ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(ParseError::InvalidSyntax { line: i + 1, message: "Expected duration after DELAY".into() });
            }
            let ms = parts[1].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid duration number".into() })?;
            commands.push(Command::Delay(ms));
        } else if line.starts_with("MOVE ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return Err(ParseError::InvalidSyntax { line: i + 1, message: "Expected x and y coordinates after MOVE".into() });
            }
            let x = parts[1].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid x coordinate".into() })?;
            let y = parts[2].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid y coordinate".into() })?;
            commands.push(Command::Move { x, y });
        } else if line == "CLICK" {
            commands.push(Command::Click { x: None, y: None });
        } else if line.starts_with("CLICK ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return Err(ParseError::InvalidSyntax { line: i + 1, message: "Expected x and y coordinates after CLICK".into() });
            }
            let x = Some(parts[1].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid x coordinate".into() })?);
            let y = Some(parts[2].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid y coordinate".into() })?);
            commands.push(Command::Click { x, y });
        } else if line.starts_with("TYPE ") {
            let text_part = line[5:].trim();
            if text_part.starts_with('"') && text_part.ends_with('"') && text_part.len() >= 2 {
                let unquoted = &text_part[1..text_part.len()-1];
                commands.push(Command::Type(unquoted.to_string()));
            } else {
                commands.push(Command::Type(text_part.to_string()));
            }
        } else if line.starts_with("KEY ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(ParseError::InvalidSyntax { line: i + 1, message: "Expected key code after KEY".into() });
            }
            commands.push(Command::Key(parts[1].to_string()));
        } else if line.starts_with("REPEAT ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(ParseError::InvalidSyntax { line: i + 1, message: "Expected count after REPEAT".into() });
            }
            let count: usize = parts[1].parse().map_err(|_| ParseError::InvalidSyntax { line: i + 1, message: "Invalid repeat count".into() })?;
            
            i += 1;
            let mut block_lines = Vec::new();
            let mut brace_depth = 1;
            while i < lines.len() {
                let inner = lines[i].trim();
                if inner.contains('{') {
                    brace_depth += 1;
                }
                if inner.contains('}') {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        break;
                    }
                }
                block_lines.push(lines[i]);
                i += 1;
            }
            let block_content = block_lines.join("\n");
            let sub_commands = parse_script(&block_content)?;
            commands.push(Command::Repeat { count, commands: sub_commands });
        } else {
            return Err(ParseError::InvalidSyntax { line: i + 1, message: format!("Unknown command: {}", line) });
        }
        i += 1;
    }

    Ok(commands)
}
