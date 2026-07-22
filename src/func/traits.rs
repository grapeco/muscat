use std::{path::PathBuf};

pub trait PathBufExt {
    fn name_without_extension(&self) -> String;
}
impl PathBufExt for PathBuf {
    fn name_without_extension(&self) -> String {
        return self.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Undefined")
            .to_string();
    }
}