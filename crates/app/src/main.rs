use std::path::Path;

use garnet_renderer::Renderer;
use garnet_runtime::Runtime;
use garnet_source::{FileSource, Source};
use garnet_ui::{Color, Element};

struct GarnetApp {
    root: Element,
    renderer: Renderer,
}

impl GarnetApp {
    fn new() -> anyhow::Result<Self> {
        let file_source = FileSource::default();
        let source = file_source.load(Path::new("examples/sample.rb"))?;

        let runtime = Runtime::new();
        let root = runtime.evaluate(&source)?;

        Ok(Self {
            root,
            renderer: Renderer::new(),
        })
    }
}

impl eframe::App for GarnetApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.renderer.render(ui, &self.root);
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        self.root
            .style
            .background
            .map(color_to_clear_color)
            .unwrap_or([1.0, 1.0, 1.0, 1.0])
    }
}

fn color_to_clear_color(color: Color) -> [f32; 4] {
    [
        f32::from(color.red) / 255.0,
        f32::from(color.green) / 255.0,
        f32::from(color.blue) / 255.0,
        1.0,
    ]
}

fn main() -> eframe::Result<()> {
    let app = GarnetApp::new().expect("failed to start Garnet");

    let options = eframe::NativeOptions::default();

    eframe::run_native("Garnet", options, Box::new(|_cc| Ok(Box::new(app))))
}
