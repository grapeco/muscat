use std::{
    borrow::Cow, collections::HashMap, error::Error, fs::{self}, path::{Path, PathBuf}
};

use mustache;
use serde::{Deserialize};
use serde_json::Value;
use resolve_path::PathResolveExt;

use crate::func::{
    process::{restart, set_wallpaper}, 
    traits::PathBufExt
};

pub const PATH_TO_CONFIG: &str = "~/.config/muscat/config.jsonc";

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub data: PathBuf,
    pub data_dir: Option<PathBuf>,
    pub targets: Vec<PathBuf>,
    wallpapers: Option<Vec<HashMap<String, String>>>,
    pub restarts: Option<Vec<String>>,
}

// FOR FUTURE, maybe

// #[derive(Default, Clone, Serialize, Deserialize, Debug)]
// #[allow(non_snake_case)]
// pub struct Base16 {
//     pub base00: String,
//     pub base01: String,
//     pub base02: String,
//     pub base03: String,
//     pub base04: String,
//     pub base05: String,
//     pub base06: String,
//     pub base07: String,
//     pub base08: String,
//     pub base09: String,
//     pub base0A: String,
//     pub base0B: String,
//     pub base0C: String,
//     pub base0D: String,
//     pub base0F: String,
// }

pub fn list_dir<T: AsRef<Path>>(dir: T) -> Vec<PathBuf> {
    let directory = fs::read_dir(dir).unwrap();
    let mut string_dir: Vec<PathBuf> = Vec::new();
    
    for entry in directory {
        let entry = entry.unwrap();
        string_dir.push(entry.path());
    }
    
    return string_dir;
}

pub fn execute(paths: Vec<PathBuf>, data_path: PathBuf, config: Config) -> Result<(), Box<dyn Error>> {
    let data_content = parse_theme(data_path.resolve().to_path_buf())?;
    
    let paths: Vec<Cow<Path>> = paths
        .iter()
        .map(|target| target.resolve())
        .collect();  

    for file in paths {    
        let name = PathBuf::from(file.clone()).with_extension("");
        
        // This code founds templates by target file name
        // Example: style.css(target) - style-temp.css(template)
        let template_file_content = fs::read_to_string(
            format!(
                "{}-temp{}", 
                name.display(),
                match PathBuf::from(file.clone()).extension() {
                    Some(ext) => format!(".{}", ext.display()),
                    None => format!(""),
                }
            )
        ).expect("Can't read template file");
        let template = mustache::compile_str(&template_file_content).expect("Can't compile str");

        // Writing compiled mustache template into target file
        let target = template.render_to_string(&data_content).expect("Can't render");
        fs::write(file, target).expect("No such file");
    }

    // Check if wallpapers option is set
    if let Some(walls) = &config.wallpapers {
        set_wallpaper(walls.to_owned(), data_path.name_without_extension());
    }

    // Check if restarts option is set
    if let Some(rest) = &config.restarts {
        restart(rest.to_owned());
    }

    return Ok(());
}

pub fn parse_theme(data_file: PathBuf) -> Result<Value, Box<dyn Error>> {
    let content = fs::read_to_string(data_file.resolve())?;
    return Ok(json5::from_str(&content)?);
}

pub fn parse_config() -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(PATH_TO_CONFIG.resolve())?;
    return Ok(json5::from_str(&content)?);
}