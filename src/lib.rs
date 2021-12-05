//! Common utilities for the challenges

use include_dir::{Dir, File};

pub trait InputProvider {
    fn get_input(&self, name: &str) -> anyhow::Result<&'static str>;
}

impl InputProvider for Dir<'static> {
    fn get_input(&self, name: &str) -> anyhow::Result<&'static str> {
        self.get_file(name)
            .and_then(File::contents_utf8)
            .ok_or_else(|| anyhow::anyhow!("missing file"))
    }
}
