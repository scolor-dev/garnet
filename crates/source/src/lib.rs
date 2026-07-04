use std::fs;
use std::path::Path;

pub trait Source {
    fn load(&self, path: &Path) -> anyhow::Result<String>;
}

#[derive(Debug, Default)]
pub struct FileSource;

impl Source for FileSource {
    fn load(&self, path: &Path) -> anyhow::Result<String> {
        let source = fs::read_to_string(path)?;
        Ok(source)
    }
}
