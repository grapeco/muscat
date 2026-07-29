use std::path::{Path};

pub trait PathExt {
    fn name_without_extension(&self) -> String;
}
impl PathExt for Path {
    fn name_without_extension(&self) -> String {
        return self.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Undefined")
            .to_string();
    }
}