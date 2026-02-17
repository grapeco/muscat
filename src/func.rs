use std::{
    env, ffi::OsStr, fmt::Debug, fs::{self, File}, io::{BufRead, BufReader}, path::{Path, PathBuf}, process::Command, thread::sleep, time::Duration
};

use mustache;
use serde::Deserialize;
use serde_json::Value;
use resolve_path::PathResolveExt;

pub const PATH_TO_CONFIG: &str = "config.jsonc";

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    data: String,
    pub targets: Vec<String>,
    restarts: Vec<String>,
}

pub fn list_dir<T: AsRef<Path>>(dir: T) -> Vec<String> {
    let directory = fs::read_dir(dir).unwrap();
    let mut string_dir: Vec<String> = Vec::new();
    
    for entry in directory {
        let entry = entry.unwrap().file_name();
        string_dir.push(entry.into_string().unwrap());
    }
    
    return string_dir;
}

pub fn collect_from_file(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
}

pub fn execute<T>(paths: Vec<T>, data: T)
where 
    T: AsRef<Path> + Clone,
    PathBuf: From<T>
{
    let data_content: Value = serde_json::from_str(
        fs::read_to_string(data).unwrap().as_str()
    ).expect("Can't parse data file");

    for file in paths {    
        let mut name = PathBuf::from(file.clone());
        name.set_extension("");
        
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
}

pub fn parse_config() -> Config {
    let config_content = fs::read_to_string(PATH_TO_CONFIG.resolve()).expect("No such file");
    let config: Config = serde_json::from_str(config_content.as_str()).unwrap();
    
    return config;
}

pub fn from_args() {
    let args: Vec<String> = env::args().collect();
    let mut data: PathBuf = PathBuf::from("");
    let mut targets: Vec<PathBuf> = Vec::new();
    
    let mut i = 1;
    while i < args.len() {
        match (args[i].trim(), args[i+1].trim()) {
            ("--from", data_file) => {
                println!("{} {}", args[i], args[i+1]);
                data = PathBuf::from(data_file.resolve());
                i += 2;
            }
            ("--targets", paths) => {
                println!("{} {}", args[i], args[i+1]);
                let file = File::open(paths).unwrap();
                targets = collect_from_file(&file)
                    .iter()
                    .map(|path| PathBuf::from(path.resolve()))
                    .collect();
                i += 2;
            }
            _ => {}
        }
    }
    
    execute(targets, data);
}

pub fn from_config() {
    let config = parse_config();
    let targets = config.targets
        .iter()
        .map(|target| target.resolve())
        .collect();
    
    execute(targets, config.data.resolve());
}

fn kill_process<T: AsRef<OsStr> + Debug>(process: T) {
    println!("🔪 Завершаем процесс: {:?}", process);
    
    Command::new("pkill")
        .arg(process)
        .status()
        .ok();
}

fn start_process<T: AsRef<OsStr> + Debug>(process: T) {
    println!("Запускаем процесс: {:?}", process);
        
    Command::new(process)
        .spawn()
        .ok();
}

fn check_valid<T: AsRef<OsStr> + Debug>(process: T) -> bool {
    let output = Command::new("pgrep")
        .arg(process)
        .output();
    
    match output {
        Ok(val) if !val.stdout.is_empty() => return true,
        Ok(_) => return false,
        Err(_) => return false
    }
}

pub fn restart() {
    let restarts = parse_config().restarts;
    
    // Iterating in list of restarts
    for i in restarts {
        println!("{}", &i);
        
        if check_valid(&i) == true {
            kill_process(&i);
            
            sleep(Duration::from_millis(100));
            
            let start_name = match i.trim() {
                "zed" => "zeditor",
                other => other
            };
            start_process(start_name);
        }
    }
}