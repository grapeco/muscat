use std::{
    borrow::Cow, collections::HashMap, error::Error, fs::{self, File}, io::Write, path::{Path, PathBuf}
};

use mustache;
use resolve_path::PathResolveExt;
use serde::{Deserialize};
use serde_json::{Value};

pub const PATH_TO_CONFIG: &str = "~/.config/muscat/config.jsonc";
pub const THEME_DIR: &str = "~/.config/muscat/themes";

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub theme: PathBuf,
    pub targets: Vec<PathBuf>,
    pub wallpapers: Option<Vec<HashMap<String, String>>>,
    pub restarts: Option<Vec<String>>,
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let resolved_config = PathBuf::from(PATH_TO_CONFIG.resolve());
    let resolved_theme_dir = PathBuf::from(THEME_DIR.resolve());

    // Create config file
    if !resolved_config.exists() {
        fs::create_dir_all(&resolved_config.parent().unwrap())?;
        
        let text = fs::read_to_string("./init_files/test.jsonc")?;

        let mut file = File::create(&resolved_config)?;

        file.write_all(text.as_bytes())?;
    } 

    // Create themes directory
    if !resolved_theme_dir.exists() {
        fs::create_dir_all(&resolved_theme_dir)?;

        let text = fs::read_to_string("./init_files/catppuccin.json")?;

        let mut file = File::create(&resolved_theme_dir.join("catppuccin.json"))?;

        println!();

        file.write_all(text.as_bytes())?;
    }

    Ok(())
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

pub fn execute(paths: &[&Path], data_path: &Path) -> Result<(), Box<dyn Error>> {
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