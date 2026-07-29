use std::{
    borrow::Cow, collections::HashMap, error::Error, fs::{self}, path::{Path, PathBuf}
};

use mustache;
use resolve_path::PathResolveExt;
use serde::{Deserialize};
use serde_json::Value;

use crate::func::{
    process::{restart, set_wallpaper}, 
    traits::PathExt
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

pub fn list_dir<T: AsRef<Path>>(dir: T) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let directory = fs::read_dir(dir)?;
    let mut string_dir: Vec<PathBuf> = Vec::new();
    
    for entry in directory {
        let entry = entry?;
        string_dir.push(entry.path());
    }
    
    return Ok(string_dir);
}

pub fn execute(paths: Vec<PathBuf>, data_path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    let data_content = parse_theme(&data_path.resolve())?;
    
    let paths: Vec<Cow<Path>> = paths
        .iter()
        .map(|target| target.resolve())
        .collect();  

    for file in paths {    
        let name = file.with_extension("");
        
        // This code founds templates by target file name
        // Example: style.css(target) - style-temp.css(template)
        let template_file_content = fs::read_to_string(
            format!(
                "{}-temp{}", 
                name.display(),
                match file.extension() {
                    Some(ext) => format!(".{}", ext.display()),
                    None => format!(""),
                }
            )
        )?;
        let template = mustache::compile_str(&template_file_content)?;

        // Writing compiled mustache template into target file
        let target = template.render_to_string(&data_content)?;
        fs::write(file, target)?;
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

pub fn parse_theme(data_file: &Path) -> Result<Value, Box<dyn Error>> {
    let content = fs::read_to_string(data_file.resolve())?;
    return Ok(json5::from_str(&content)?);
}

pub fn parse_config() -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(PATH_TO_CONFIG.resolve())?;
    return Ok(json5::from_str(&content)?);
}