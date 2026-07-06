use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub style: Style,
    pub node: Node,
}

impl Element {
    pub fn new(node: Node) -> Self {
        Self {
            style: Style::default(),
            node,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Option<f32>,
    pub margin: Option<f32>,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub background: Option<Color>,
    pub visible: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            padding: None,
            margin: None,
            color: None,
            size: None,
            background: None,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Page {
        children: Vec<Element>,
    },
    Column {
        children: Vec<Element>,
    },
    Row {
        children: Vec<Element>,
    },
    Text {
        value: String,
    },
    Button {
        label: String,
        on_click: Option<Action>,
    },
    Input {
        value: String,
        placeholder: Option<String>,
    },
    Image {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Print { message: String },
    Invoke { name: String },
}

impl Node {
    pub fn children(&self) -> &[Element] {
        match self {
            Node::Page { children } => children,
            Node::Column { children } => children,
            Node::Row { children } => children,
            Node::Text { .. } => &[],
            Node::Button { .. } => &[],
            Node::Input { .. } => &[],
            Node::Image { .. } => &[],
        }
    }
}

pub mod dsl {
    use std::path::PathBuf;

    use super::{Action, Element, Node};

    pub fn page(children: Vec<Element>) -> Element {
        Element::new(Node::Page { children })
    }

    pub fn column(children: Vec<Element>) -> Element {
        Element::new(Node::Column { children })
    }

    pub fn row(children: Vec<Element>) -> Element {
        Element::new(Node::Row { children })
    }

    pub fn text(value: impl Into<String>) -> Element {
        Element::new(Node::Text {
            value: value.into(),
        })
    }

    pub fn button(label: impl Into<String>) -> Element {
        Element::new(Node::Button {
            label: label.into(),
            on_click: None,
        })
    }

    pub fn button_with_action(label: impl Into<String>, action: Action) -> Element {
        Element::new(Node::Button {
            label: label.into(),
            on_click: Some(action),
        })
    }

    pub fn input(value: impl Into<String>) -> Element {
        Element::new(Node::Input {
            value: value.into(),
            placeholder: None,
        })
    }

    pub fn input_placeholder(placeholder: impl Into<String>) -> Element {
        Element::new(Node::Input {
            value: String::new(),
            placeholder: Some(placeholder.into()),
        })
    }

    pub fn image(path: impl Into<PathBuf>) -> Element {
        Element::new(Node::Image { path: path.into() })
    }
}
