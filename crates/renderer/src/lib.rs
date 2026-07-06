use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui::{
    Color32, ColorImage, Frame, Margin, RichText, TextEdit, TextureHandle, TextureOptions, Ui, Vec2,
};
use garnet_source::{FileSource, Source};
use garnet_ui::{Action, Color, Element, Node, Style};

pub struct Renderer {
    source: FileSource,
    textures: HashMap<PathBuf, TextureHandle>,
    input_values: HashMap<String, String>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            source: FileSource,
            textures: HashMap::new(),
            input_values: HashMap::new(),
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ui: &mut Ui, element: &Element) -> Vec<Action> {
        let mut actions = Vec::new();
        self.render_element(ui, element, "root", &mut actions);
        actions
    }

    fn render_element(
        &mut self,
        ui: &mut Ui,
        element: &Element,
        key: &str,
        actions: &mut Vec<Action>,
    ) {
        if !element.style.visible {
            return;
        }

        if matches!(element.node, Node::Page { .. }) {
            self.render_node(ui, element, key, actions);
            return;
        }

        if has_frame_style(&element.style) {
            let mut frame = Frame::NONE;
            if let Some(background) = element.style.background {
                frame = frame.fill(to_color32(background));
            }
            if let Some(padding) = element.style.padding {
                frame = frame.inner_margin(Margin::same(padding as i8));
            }
            if let Some(margin) = element.style.margin {
                frame = frame.outer_margin(Margin::same(margin as i8));
            }

            frame.show(ui, |ui| {
                self.render_node(ui, element, key, actions);
            });
        } else {
            self.render_node(ui, element, key, actions);
        }
    }

    fn render_node(
        &mut self,
        ui: &mut Ui,
        element: &Element,
        key: &str,
        actions: &mut Vec<Action>,
    ) {
        match &element.node {
            Node::Page { children } => {
                ui.allocate_ui(ui.available_size(), |ui| {
                    if let Some(padding) = element.style.padding {
                        ui.add_space(padding);
                    }
                    for (index, child) in children.iter().enumerate() {
                        let child_key = child_key(key, index);
                        self.render_element(ui, child, &child_key, actions);
                    }
                });
            }

            Node::Column { children } => {
                ui.vertical(|ui| {
                    for (index, child) in children.iter().enumerate() {
                        let child_key = child_key(key, index);
                        self.render_element(ui, child, &child_key, actions);
                    }
                });
            }

            Node::Row { children } => {
                ui.horizontal(|ui| {
                    for (index, child) in children.iter().enumerate() {
                        let child_key = child_key(key, index);
                        self.render_element(ui, child, &child_key, actions);
                    }
                });
            }

            Node::Text { value } => {
                let mut text = RichText::new(value);
                if let Some(color) = element.style.color {
                    text = text.color(to_color32(color));
                }
                if let Some(size) = element.style.size {
                    text = text.size(size);
                }
                ui.label(text);
            }

            Node::Button { label, on_click } => {
                let size = widget_size(&element.style);
                let response = if let Some(size) = size {
                    ui.add_sized(size, egui::Button::new(label))
                } else {
                    ui.button(label)
                };

                if response.clicked() {
                    if let Some(action) = on_click {
                        actions.push(action.clone());
                    }
                }
            }

            Node::Input { value, placeholder } => {
                let input_value = self
                    .input_values
                    .entry(key.to_owned())
                    .or_insert_with(|| value.clone());
                let mut edit = TextEdit::singleline(input_value);
                if let Some(placeholder) = placeholder {
                    edit = edit.hint_text(placeholder);
                }
                if let Some(size) = widget_size(&element.style) {
                    let _ = ui.add_sized(size, edit);
                } else {
                    let _ = ui.add(edit);
                }
            }

            Node::Image { path } => {
                self.render_image(ui, path, &element.style);
            }
        }
    }

    fn render_image(&mut self, ui: &mut Ui, path: &Path, style: &Style) {
        if !self.textures.contains_key(path) {
            match load_texture(ui, &self.source, path) {
                Ok(texture) => {
                    self.textures.insert(path.to_path_buf(), texture);
                }
                Err(error) => {
                    ui.label(format!("failed to load image {}: {error}", path.display()));
                    return;
                }
            }
        }

        if let Some(texture) = self.textures.get(path) {
            let size = image_size(style, texture);
            ui.image((texture.id(), size));
        }
    }
}

fn load_texture(ui: &Ui, source: &impl Source, path: &Path) -> anyhow::Result<TextureHandle> {
    let bytes = source.load_bytes(path)?;
    let image = image::load_from_memory(&bytes)?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

    Ok(ui
        .ctx()
        .load_texture(path.to_string_lossy(), color_image, TextureOptions::LINEAR))
}

fn to_color32(color: Color) -> Color32 {
    Color32::from_rgb(color.red, color.green, color.blue)
}

fn child_key(parent: &str, index: usize) -> String {
    format!("{parent}/{index}")
}

fn has_frame_style(style: &Style) -> bool {
    style.background.is_some() || style.padding.is_some() || style.margin.is_some()
}

fn widget_size(style: &Style) -> Option<Vec2> {
    match (style.width, style.height) {
        (Some(width), Some(height)) => Some(Vec2::new(width, height)),
        (Some(width), None) => Some(Vec2::new(width, 0.0)),
        (None, Some(height)) => Some(Vec2::new(0.0, height)),
        (None, None) => None,
    }
}

fn image_size(style: &Style, texture: &TextureHandle) -> Vec2 {
    let original = texture.size_vec2();

    match (style.width, style.height) {
        (Some(width), Some(height)) => Vec2::new(width, height),
        (Some(width), None) => Vec2::new(width, original.y * (width / original.x)),
        (None, Some(height)) => Vec2::new(original.x * (height / original.y), height),
        (None, None) => original,
    }
}
