use std::fs;
use std::path::Path;

pub trait Source {
    fn load(&self, path: &Path) -> anyhow::Result<String>;
    fn load_bytes(&self, path: &Path) -> anyhow::Result<Vec<u8>>;
}

#[derive(Debug, Default)]
pub struct FileSource;

impl Source for FileSource {
    fn load(&self, path: &Path) -> anyhow::Result<String> {
        let source = fs::read_to_string(path)?;
        Ok(source)
    }

    fn load_bytes(&self, path: &Path) -> anyhow::Result<Vec<u8>> {
        let bytes = fs::read(path)?;
        Ok(bytes)
    }
}
