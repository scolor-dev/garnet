use std::path::Path;

use garnet_renderer::Renderer;
use garnet_runtime::Runtime;
use garnet_source::{FileSource, Source};
use garnet_ui::Node;

struct GarnetApp {
    root: Node,
    renderer: Renderer,
}

impl GarnetApp {
    fn new() -> anyhow::Result<Self> {
        let file_source = FileSource::default();
        let source = file_source.load(Path::new("examples/hello.rb"))?;

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
}

fn main() -> eframe::Result<()> {
    let app = GarnetApp::new().expect("failed to start Garnet");

    let options = eframe::NativeOptions::default();

    eframe::run_native("Garnet", options, Box::new(|_cc| Ok(Box::new(app))))
}
