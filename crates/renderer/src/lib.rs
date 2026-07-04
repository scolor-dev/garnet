use egui::Ui;
use garnet_ui::Node;

#[derive(Debug, Default)]
pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut Ui, node: &Node) {
        render_node(ui, node);
    }
}

fn render_node(ui: &mut Ui, node: &Node) {
    match node {
        Node::Page { children } => {
            for child in children {
                render_node(ui, child);
            }
        }

        Node::Column { children } => {
            ui.vertical(|ui| {
                for child in children {
                    render_node(ui, child);
                }
            });
        }

        Node::Row { children } => {
            ui.horizontal(|ui| {
                for child in children {
                    render_node(ui, child);
                }
            });
        }

        Node::Text { value } => {
            ui.label(value);
        }
    }
}
