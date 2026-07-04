#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Page { children: Vec<Node> },
    Column { children: Vec<Node> },
    Row { children: Vec<Node> },
    Text { value: String },
}

impl Node {
    pub fn children(&self) -> &[Node] {
        match self {
            Node::Page { children } => children,
            Node::Column { children } => children,
            Node::Row { children } => children,
            Node::Text { .. } => &[],
        }
    }
}

pub mod dsl {
    use super::Node;

    pub fn page(children: Vec<Node>) -> Node {
        Node::Page { children }
    }

    pub fn column(children: Vec<Node>) -> Node {
        Node::Column { children }
    }

    pub fn row(children: Vec<Node>) -> Node {
        Node::Row { children }
    }

    pub fn text(value: impl Into<String>) -> Node {
        Node::Text {
            value: value.into(),
        }
    }
}
