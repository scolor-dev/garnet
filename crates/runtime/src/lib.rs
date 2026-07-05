use garnet_ui::dsl::{button, column, image, input, page, row, text};
use garnet_ui::{Color, Element, Style};

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("unsupported syntax on line {line}: {snippet}")]
    UnsupportedSyntax { line: usize, snippet: String },

    #[error("invalid text literal on line {line}: {snippet}")]
    InvalidTextLiteral { line: usize, snippet: String },

    #[error("unexpected page block on line {line}")]
    UnexpectedPageBlock { line: usize },

    #[error("column block must be inside another block on line {line}")]
    ColumnOutsideBlock { line: usize },

    #[error("row block must be inside another block on line {line}")]
    RowOutsideBlock { line: usize },

    #[error("text must be inside a block on line {line}")]
    TextOutsideBlock { line: usize },

    #[error("button must be inside a block on line {line}")]
    ButtonOutsideBlock { line: usize },

    #[error("invalid button literal on line {line}: {snippet}")]
    InvalidButtonLiteral { line: usize, snippet: String },

    #[error("input must be inside a block on line {line}")]
    InputOutsideBlock { line: usize },

    #[error("invalid input literal on line {line}: {snippet}")]
    InvalidInputLiteral { line: usize, snippet: String },

    #[error("image must be inside a block on line {line}")]
    ImageOutsideBlock { line: usize },

    #[error("invalid image literal on line {line}: {snippet}")]
    InvalidImageLiteral { line: usize, snippet: String },

    #[error("unexpected end on line {line}")]
    UnexpectedEnd { line: usize },

    #[error("unclosed {kind} block opened on line {line}")]
    UnclosedBlock { kind: &'static str, line: usize },

    #[error("missing page block")]
    MissingPageBlock,

    #[error("unknown style `{name}` on line {line}")]
    UnknownStyle { line: usize, name: String },

    #[error("invalid style value for `{name}` on line {line}: {value}")]
    InvalidStyleValue {
        line: usize,
        name: String,
        value: String,
    },
}

#[derive(Debug, Default)]
pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, source: &str) -> Result<Element> {
        let lines: Vec<_> = source.lines().collect();
        let mut index = 0;
        let mut stack = Vec::new();
        let mut root = None;

        while index < lines.len() {
            let line = lines[index].trim();
            let line_number = index + 1;

            if line.is_empty() {
                index += 1;
                continue;
            }

            let statement = if starts_styled_statement(line) {
                collect_styled_statement(&lines, &mut index)
            } else {
                index += 1;
                line.to_owned()
            };

            match classify_statement(&statement) {
                Statement::Page(style) => {
                    if root.is_some() || !stack.is_empty() {
                        return Err(RuntimeError::UnexpectedPageBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Page, line_number, style));
                }
                Statement::Column(style) => {
                    if stack.is_empty() {
                        return Err(RuntimeError::ColumnOutsideBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Column, line_number, style));
                }
                Statement::Row(style) => {
                    if stack.is_empty() {
                        return Err(RuntimeError::RowOutsideBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Row, line_number, style));
                }
                Statement::End => {
                    let block = stack
                        .pop()
                        .ok_or(RuntimeError::UnexpectedEnd { line: line_number })?;
                    let element = block.into_element();

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(element);
                    } else {
                        root = Some(element);
                    }
                }
                Statement::Text { value, style } => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::TextOutsideBlock { line: line_number });
                    };
                    block.children.push(text(value).with_style(style));
                }
                Statement::Button { label, style } => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::ButtonOutsideBlock { line: line_number });
                    };
                    block.children.push(button(label).with_style(style));
                }
                Statement::Input { value, style } => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::InputOutsideBlock { line: line_number });
                    };
                    block.children.push(input(value).with_style(style));
                }
                Statement::Image { path, style } => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::ImageOutsideBlock { line: line_number });
                    };
                    block.children.push(image(path).with_style(style));
                }
                Statement::Unsupported => {
                    return Err(RuntimeError::UnsupportedSyntax {
                        line: line_number,
                        snippet: statement,
                    });
                }
                Statement::InvalidTextLiteral => {
                    return Err(RuntimeError::InvalidTextLiteral {
                        line: line_number,
                        snippet: statement,
                    });
                }
                Statement::InvalidButtonLiteral => {
                    return Err(RuntimeError::InvalidButtonLiteral {
                        line: line_number,
                        snippet: statement,
                    });
                }
                Statement::InvalidInputLiteral => {
                    return Err(RuntimeError::InvalidInputLiteral {
                        line: line_number,
                        snippet: statement,
                    });
                }
                Statement::InvalidImageLiteral => {
                    return Err(RuntimeError::InvalidImageLiteral {
                        line: line_number,
                        snippet: statement,
                    });
                }
                Statement::StyleError(error) => return Err(error.with_line(line_number)),
            }
        }

        if let Some(block) = stack.pop() {
            return Err(RuntimeError::UnclosedBlock {
                kind: block.kind.as_str(),
                line: block.line,
            });
        }

        root.ok_or(RuntimeError::MissingPageBlock)
    }
}

#[derive(Debug)]
struct Block {
    kind: BlockKind,
    line: usize,
    style: Style,
    children: Vec<Element>,
}

impl Block {
    fn new(kind: BlockKind, line: usize, style: Style) -> Self {
        Self {
            kind,
            line,
            style,
            children: Vec::new(),
        }
    }

    fn into_element(self) -> Element {
        let element = match self.kind {
            BlockKind::Page => page(self.children),
            BlockKind::Column => column(self.children),
            BlockKind::Row => row(self.children),
        };

        element.with_style(self.style)
    }
}

#[derive(Debug, Clone, Copy)]
enum BlockKind {
    Page,
    Column,
    Row,
}

impl BlockKind {
    fn as_str(self) -> &'static str {
        match self {
            BlockKind::Page => "page",
            BlockKind::Column => "column",
            BlockKind::Row => "row",
        }
    }
}

#[derive(Debug)]
enum Statement {
    Page(Style),
    Column(Style),
    Row(Style),
    End,
    Text { value: String, style: Style },
    Button { label: String, style: Style },
    Input { value: String, style: Style },
    Image { path: String, style: Style },
    Unsupported,
    InvalidTextLiteral,
    InvalidButtonLiteral,
    InvalidInputLiteral,
    InvalidImageLiteral,
    StyleError(StyleParseError),
}

#[derive(Debug)]
enum StyleParseError {
    UnknownStyle { name: String },
    InvalidValue { name: String, value: String },
}

impl StyleParseError {
    fn with_line(self, line: usize) -> RuntimeError {
        match self {
            StyleParseError::UnknownStyle { name } => RuntimeError::UnknownStyle { line, name },
            StyleParseError::InvalidValue { name, value } => {
                RuntimeError::InvalidStyleValue { line, name, value }
            }
        }
    }
}

fn classify_statement(statement: &str) -> Statement {
    if let Some(style) = parse_block_statement(statement, "page do") {
        return style.map_or_else(Statement::StyleError, Statement::Page);
    }

    if let Some(style) = parse_block_statement(statement, "column do") {
        return style.map_or_else(Statement::StyleError, Statement::Column);
    }

    if let Some(style) = parse_block_statement(statement, "row do") {
        return style.map_or_else(Statement::StyleError, Statement::Row);
    }

    if statement == "end" {
        return Statement::End;
    }

    if let Some(rest) = statement.strip_prefix("text ") {
        return parse_leaf_statement(rest).map_or(Statement::InvalidTextLiteral, |parsed| {
            parsed.map_or_else(Statement::StyleError, |(value, style)| Statement::Text {
                value,
                style,
            })
        });
    }

    if let Some(rest) = statement.strip_prefix("button ") {
        return parse_leaf_statement(rest).map_or(Statement::InvalidButtonLiteral, |parsed| {
            parsed.map_or_else(Statement::StyleError, |(label, style)| Statement::Button {
                label,
                style,
            })
        });
    }

    if let Some(rest) = statement.strip_prefix("input ") {
        return parse_leaf_statement(rest).map_or(Statement::InvalidInputLiteral, |parsed| {
            parsed.map_or_else(Statement::StyleError, |(value, style)| Statement::Input {
                value,
                style,
            })
        });
    }

    if let Some(rest) = statement.strip_prefix("image ") {
        return parse_leaf_statement(rest).map_or(Statement::InvalidImageLiteral, |parsed| {
            parsed.map_or_else(Statement::StyleError, |(path, style)| Statement::Image {
                path,
                style,
            })
        });
    }

    Statement::Unsupported
}

fn parse_block_statement(
    statement: &str,
    keyword: &'static str,
) -> Option<std::result::Result<Style, StyleParseError>> {
    if statement == keyword {
        return Some(Ok(Style::default()));
    }

    let rest = statement.strip_prefix(keyword)?;
    let rest = rest.trim();
    let style_source = rest.strip_prefix(',')?.trim();

    Some(parse_style_args(style_source))
}

fn parse_leaf_statement(
    rest: &str,
) -> Option<std::result::Result<(String, Style), StyleParseError>> {
    let (value, rest) = parse_string_literal_with_rest(rest.trim())?;
    let rest = rest.trim();

    if rest.is_empty() {
        return Some(Ok((value, Style::default())));
    }

    let style_source = rest.strip_prefix(',')?.trim();
    Some(parse_style_args(style_source).map(|style| (value, style)))
}

fn parse_string_literal_with_rest(value: &str) -> Option<(String, &str)> {
    let value = value.strip_prefix('"')?;
    let end = value.find('"')?;
    let literal = value[..end].to_owned();
    let rest = &value[end + 1..];

    Some((literal, rest))
}

fn parse_style_args(source: &str) -> std::result::Result<Style, StyleParseError> {
    let mut style = Style::default();

    for part in source.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let Some((name, value)) = part.split_once(':') else {
            return Err(StyleParseError::InvalidValue {
                name: part.to_owned(),
                value: String::new(),
            });
        };

        let name = name.trim();
        let value = value.trim();

        match name {
            "width" => style.width = Some(parse_f32(name, value)?),
            "height" => style.height = Some(parse_f32(name, value)?),
            "padding" => style.padding = Some(parse_f32(name, value)?),
            "margin" => style.margin = Some(parse_f32(name, value)?),
            "size" => style.size = Some(parse_f32(name, value)?),
            "color" => style.color = Some(parse_color(name, value)?),
            "background" => style.background = Some(parse_color(name, value)?),
            "visible" => style.visible = parse_bool(name, value)?,
            _ => {
                return Err(StyleParseError::UnknownStyle {
                    name: name.to_owned(),
                });
            }
        }
    }

    Ok(style)
}

fn parse_f32(name: &str, value: &str) -> std::result::Result<f32, StyleParseError> {
    value.parse().map_err(|_| StyleParseError::InvalidValue {
        name: name.to_owned(),
        value: value.to_owned(),
    })
}

fn parse_bool(name: &str, value: &str) -> std::result::Result<bool, StyleParseError> {
    value.parse().map_err(|_| StyleParseError::InvalidValue {
        name: name.to_owned(),
        value: value.to_owned(),
    })
}

fn parse_color(name: &str, value: &str) -> std::result::Result<Color, StyleParseError> {
    let Some(value) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    else {
        return Err(StyleParseError::InvalidValue {
            name: name.to_owned(),
            value: value.to_owned(),
        });
    };
    let Some(hex) = value.strip_prefix('#') else {
        return Err(StyleParseError::InvalidValue {
            name: name.to_owned(),
            value: value.to_owned(),
        });
    };
    if hex.len() != 6 {
        return Err(StyleParseError::InvalidValue {
            name: name.to_owned(),
            value: value.to_owned(),
        });
    }

    let red = parse_hex_byte(name, value, &hex[0..2])?;
    let green = parse_hex_byte(name, value, &hex[2..4])?;
    let blue = parse_hex_byte(name, value, &hex[4..6])?;

    Ok(Color::new(red, green, blue))
}

fn parse_hex_byte(
    name: &str,
    original: &str,
    value: &str,
) -> std::result::Result<u8, StyleParseError> {
    u8::from_str_radix(value, 16).map_err(|_| StyleParseError::InvalidValue {
        name: name.to_owned(),
        value: original.to_owned(),
    })
}

fn starts_styled_statement(line: &str) -> bool {
    line.starts_with("page do")
        || line.starts_with("column do")
        || line.starts_with("row do")
        || line.starts_with("text ")
        || line.starts_with("button ")
        || line.starts_with("input ")
        || line.starts_with("image ")
}

fn collect_styled_statement(lines: &[&str], index: &mut usize) -> String {
    let mut statement = lines[*index].trim().to_owned();
    *index += 1;

    while statement.trim_end().ends_with(',') && *index < lines.len() {
        let next = lines[*index].trim();
        if next.is_empty() {
            *index += 1;
            continue;
        }

        statement.push(' ');
        statement.push_str(next);
        *index += 1;
    }

    statement
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Runtime, RuntimeError};
    use garnet_ui::{Color, Element, Node, Style};

    #[test]
    fn evaluates_nested_column() {
        let source = r#"
            page do
                column do
                    text "Hello Garnet"
                    text "Ruby Web Protocol"
                end
            end
        "#;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Element::new(Node::Page {
                children: vec![Element::new(Node::Column {
                    children: vec![
                        Element::new(Node::Text {
                            value: "Hello Garnet".to_owned(),
                        }),
                        Element::new(Node::Text {
                            value: "Ruby Web Protocol".to_owned(),
                        }),
                    ],
                })],
            })
        );
    }

    #[test]
    fn evaluates_styled_text_button_and_image() {
        let source = r##"
            page do
                column do
                    text "Garnet",
                        color: "#3b82f6",
                        size: 28

                    button "Open",
                        width: 160,
                        height: 40

                    input "Search",
                        width: 220

                    image "examples/assets/logo.png",
                        width: 128
                end
            end
        "##;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Element::new(Node::Page {
                children: vec![Element::new(Node::Column {
                    children: vec![
                        Element::new(Node::Text {
                            value: "Garnet".to_owned(),
                        })
                        .with_style(Style {
                            color: Some(Color::new(0x3b, 0x82, 0xf6)),
                            size: Some(28.0),
                            ..Style::default()
                        }),
                        Element::new(Node::Button {
                            label: "Open".to_owned(),
                        })
                        .with_style(Style {
                            width: Some(160.0),
                            height: Some(40.0),
                            ..Style::default()
                        }),
                        Element::new(Node::Input {
                            value: "Search".to_owned(),
                        })
                        .with_style(Style {
                            width: Some(220.0),
                            ..Style::default()
                        }),
                        Element::new(Node::Image {
                            path: PathBuf::from("examples/assets/logo.png"),
                        })
                        .with_style(Style {
                            width: Some(128.0),
                            ..Style::default()
                        }),
                    ],
                })],
            })
        );
    }

    #[test]
    fn evaluates_column_and_row_nesting() {
        let source = r#"
            page do
                column do
                    text "Title"

                    row do
                        text "A"
                        text "B"
                        text "C"
                    end
                end
            end
        "#;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Element::new(Node::Page {
                children: vec![Element::new(Node::Column {
                    children: vec![
                        Element::new(Node::Text {
                            value: "Title".to_owned(),
                        }),
                        Element::new(Node::Row {
                            children: vec![
                                Element::new(Node::Text {
                                    value: "A".to_owned(),
                                }),
                                Element::new(Node::Text {
                                    value: "B".to_owned(),
                                }),
                                Element::new(Node::Text {
                                    value: "C".to_owned(),
                                }),
                            ],
                        }),
                    ],
                })],
            })
        );
    }

    #[test]
    fn rejects_unknown_style() {
        let error = Runtime::new()
            .evaluate(
                r#"
                    page do
                        text "Nope",
                            radius: 8
                    end
                "#,
            )
            .unwrap_err();

        assert!(matches!(error, RuntimeError::UnknownStyle { line: 3, .. }));
    }

    #[test]
    fn evaluates_sample_file() {
        let source = include_str!("../../../examples/sample.rb");

        Runtime::new().evaluate(source).unwrap();
    }

    #[test]
    fn rejects_unexpected_end() {
        let error = Runtime::new().evaluate("end").unwrap_err();

        assert!(matches!(error, RuntimeError::UnexpectedEnd { line: 1 }));
    }

    #[test]
    fn rejects_unclosed_block() {
        let error = Runtime::new()
            .evaluate(
                r#"
                    page do
                        column do
                "#,
            )
            .unwrap_err();

        assert!(matches!(
            error,
            RuntimeError::UnclosedBlock {
                kind: "column",
                line: 3,
            }
        ));
    }
}
