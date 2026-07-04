use garnet_ui::Node;
use garnet_ui::dsl::{button, column, image, page, row, text};

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
}

#[derive(Debug, Default)]
pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, source: &str) -> Result<Node> {
        let mut stack = Vec::new();
        let mut root = None;

        for (index, line) in source.lines().enumerate() {
            let line = line.trim();
            let line_number = index + 1;

            match line {
                "" => {}
                "page do" => {
                    if root.is_some() || !stack.is_empty() {
                        return Err(RuntimeError::UnexpectedPageBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Page, line_number));
                }
                "column do" => {
                    if stack.is_empty() {
                        return Err(RuntimeError::ColumnOutsideBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Column, line_number));
                }
                "row do" => {
                    if stack.is_empty() {
                        return Err(RuntimeError::RowOutsideBlock { line: line_number });
                    }

                    stack.push(Block::new(BlockKind::Row, line_number));
                }
                "end" => {
                    let block = stack
                        .pop()
                        .ok_or(RuntimeError::UnexpectedEnd { line: line_number })?;
                    let node = block.into_node();

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root = Some(node);
                    }
                }
                _ if line.starts_with("text ") => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::TextOutsideBlock { line: line_number });
                    };
                    let value =
                        parse_text_line(line).ok_or_else(|| RuntimeError::InvalidTextLiteral {
                            line: line_number,
                            snippet: line.to_owned(),
                        })?;
                    block.children.push(text(value));
                }
                _ if line.starts_with("button ") => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::ButtonOutsideBlock { line: line_number });
                    };
                    let label = parse_button_line(line).ok_or_else(|| {
                        RuntimeError::InvalidButtonLiteral {
                            line: line_number,
                            snippet: line.to_owned(),
                        }
                    })?;
                    block.children.push(button(label));
                }
                _ if line.starts_with("image ") => {
                    let Some(block) = stack.last_mut() else {
                        return Err(RuntimeError::ImageOutsideBlock { line: line_number });
                    };
                    let path = parse_image_line(line).ok_or_else(|| {
                        RuntimeError::InvalidImageLiteral {
                            line: line_number,
                            snippet: line.to_owned(),
                        }
                    })?;
                    block.children.push(image(path));
                }
                _ => {
                    return Err(RuntimeError::UnsupportedSyntax {
                        line: line_number,
                        snippet: line.to_owned(),
                    });
                }
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
    children: Vec<Node>,
}

impl Block {
    fn new(kind: BlockKind, line: usize) -> Self {
        Self {
            kind,
            line,
            children: Vec::new(),
        }
    }

    fn into_node(self) -> Node {
        match self.kind {
            BlockKind::Page => page(self.children),
            BlockKind::Column => column(self.children),
            BlockKind::Row => row(self.children),
        }
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

fn parse_text_line(line: &str) -> Option<String> {
    let rest = line.strip_prefix("text ")?;
    let rest = rest.trim();

    parse_string_literal(rest)
}

fn parse_button_line(line: &str) -> Option<String> {
    let rest = line.strip_prefix("button ")?;
    let rest = rest.trim();

    parse_string_literal(rest)
}

fn parse_image_line(line: &str) -> Option<String> {
    let rest = line.strip_prefix("image ")?;
    let rest = rest.trim();

    parse_string_literal(rest)
}

fn parse_string_literal(value: &str) -> Option<String> {
    let value = value.strip_prefix('"')?;
    let value = value.strip_suffix('"')?;

    Some(value.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Runtime, RuntimeError};
    use garnet_ui::Node;

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
            Node::Page {
                children: vec![Node::Column {
                    children: vec![
                        Node::Text {
                            value: "Hello Garnet".to_owned(),
                        },
                        Node::Text {
                            value: "Ruby Web Protocol".to_owned(),
                        },
                    ],
                }],
            }
        );
    }

    #[test]
    fn evaluates_row() {
        let source = r#"
            page do
                row do
                    text "A"
                    text "B"
                end
            end
        "#;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Node::Page {
                children: vec![Node::Row {
                    children: vec![
                        Node::Text {
                            value: "A".to_owned(),
                        },
                        Node::Text {
                            value: "B".to_owned(),
                        },
                    ],
                }],
            }
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
            Node::Page {
                children: vec![Node::Column {
                    children: vec![
                        Node::Text {
                            value: "Title".to_owned(),
                        },
                        Node::Row {
                            children: vec![
                                Node::Text {
                                    value: "A".to_owned(),
                                },
                                Node::Text {
                                    value: "B".to_owned(),
                                },
                                Node::Text {
                                    value: "C".to_owned(),
                                },
                            ],
                        },
                    ],
                }],
            }
        );
    }

    #[test]
    fn evaluates_buttons_in_row() {
        let source = r#"
            page do
                column do
                    text "Hello Garnet"

                    row do
                        button "Open"
                        button "Save"
                        button "Exit"
                    end

                    text "Version 0.1"
                end
            end
        "#;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Node::Page {
                children: vec![Node::Column {
                    children: vec![
                        Node::Text {
                            value: "Hello Garnet".to_owned(),
                        },
                        Node::Row {
                            children: vec![
                                Node::Button {
                                    label: "Open".to_owned(),
                                },
                                Node::Button {
                                    label: "Save".to_owned(),
                                },
                                Node::Button {
                                    label: "Exit".to_owned(),
                                },
                            ],
                        },
                        Node::Text {
                            value: "Version 0.1".to_owned(),
                        },
                    ],
                }],
            }
        );
    }

    #[test]
    fn evaluates_image_in_column() {
        let source = r#"
            page do
                column do
                    image "examples/assets/logo.png"

                    text "Garnet"

                    row do
                        button "Open"
                        button "Exit"
                    end
                end
            end
        "#;

        let root = Runtime::new().evaluate(source).unwrap();

        assert_eq!(
            root,
            Node::Page {
                children: vec![Node::Column {
                    children: vec![
                        Node::Image {
                            path: PathBuf::from("examples/assets/logo.png"),
                        },
                        Node::Text {
                            value: "Garnet".to_owned(),
                        },
                        Node::Row {
                            children: vec![
                                Node::Button {
                                    label: "Open".to_owned(),
                                },
                                Node::Button {
                                    label: "Exit".to_owned(),
                                },
                            ],
                        },
                    ],
                }],
            }
        );
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

    #[test]
    fn rejects_unsupported_syntax() {
        let error = Runtime::new()
            .evaluate(
                r#"
                    page do
                        tetx "typo"
                    end
                "#,
            )
            .unwrap_err();

        assert!(matches!(
            error,
            RuntimeError::UnsupportedSyntax { line: 3, .. }
        ));
    }
}
