use std::{ffi::OsStr, path::PathBuf};

pub trait PathBufExt {
    fn name_without_extension(&self) -> String;
    fn name(&self) -> String;
}
impl PathBufExt for PathBuf {
    fn name_without_extension(&self) -> String {
        return self.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Undefined")
            .to_string();
    }
    fn name(&self) -> String {
        return self.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Undefined")
            .to_string();
    }
}

pub trait StringExt {
    fn to_path_buf(&self) -> PathBuf;
}
impl StringExt for String {
    fn to_path_buf(&self) -> PathBuf {
        return PathBuf::from(self);
    }
}