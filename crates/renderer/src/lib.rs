use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui::Ui;
use egui::{ColorImage, TextureHandle, TextureOptions};
use garnet_source::{FileSource, Source};
use garnet_ui::Node;

pub struct Renderer {
    source: FileSource,
    textures: HashMap<PathBuf, TextureHandle>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            source: FileSource,
            textures: HashMap::new(),
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ui: &mut Ui, node: &Node) {
        self.render_node(ui, node);
    }

    fn render_node(&mut self, ui: &mut Ui, node: &Node) {
        match node {
            Node::Page { children } => {
                for child in children {
                    self.render_node(ui, child);
                }
            }

            Node::Column { children } => {
                ui.vertical(|ui| {
                    for child in children {
                        self.render_node(ui, child);
                    }
                });
            }

            Node::Row { children } => {
                ui.horizontal(|ui| {
                    for child in children {
                        self.render_node(ui, child);
                    }
                });
            }

            Node::Text { value } => {
                ui.label(value);
            }

            Node::Button { label } => {
                let _ = ui.button(label);
            }

            Node::Image { path } => {
                self.render_image(ui, path);
            }
        }
    }

    fn render_image(&mut self, ui: &mut Ui, path: &Path) {
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
            ui.image((texture.id(), texture.size_vec2()));
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
