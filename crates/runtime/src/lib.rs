use garnet_ui::Node;
use garnet_ui::dsl::{page, text};

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("unsupported syntax on line {line}: {snippet}")]
    UnsupportedSyntax { line: usize, snippet: String },

    #[error("invalid text literal on line {line}: {snippet}")]
    InvalidTextLiteral { line: usize, snippet: String },
}

#[derive(Debug, Default)]
pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, source: &str) -> Result<Node> {
        let mut children = Vec::new();

        for (index, line) in source.lines().enumerate() {
            let line = line.trim();
            let line_number = index + 1;

            match line {
                "" | "page do" | "end" => {}
                _ if line.starts_with("text ") => {
                    let value =
                        parse_text_line(line).ok_or_else(|| RuntimeError::InvalidTextLiteral {
                            line: line_number,
                            snippet: line.to_owned(),
                        })?;
                    children.push(text(value));
                }
                _ => {
                    return Err(RuntimeError::UnsupportedSyntax {
                        line: line_number,
                        snippet: line.to_owned(),
                    });
                }
            }
        }

        Ok(page(children))
    }
}

fn parse_text_line(line: &str) -> Option<String> {
    let rest = line.strip_prefix("text ")?;
    let rest = rest.trim();

    parse_string_literal(rest)
}

fn parse_string_literal(value: &str) -> Option<String> {
    let value = value.strip_prefix('"')?;
    let value = value.strip_suffix('"')?;

    Some(value.to_string())
}
