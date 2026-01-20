use std::{
    env, fs::{self, File}, io::{BufRead, BufReader}, path::{PathBuf}, thread::sleep, time::Duration,
};

use mustache;
use serde::Deserialize;
use serde_json::Value;
use resolve_path::PathResolveExt;

#[derive(Debug, Deserialize)]
struct Config {
    data: String,
    targets: Vec<String>
}

fn collect_from_file(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
}

fn execute(paths: Vec<PathBuf>, data: PathBuf) {
    let data_content: Value = serde_json::from_str(
        fs::read_to_string(data).unwrap().as_str()
    ).expect("Can't parse data file");

    for file in paths {    
        let mut name: PathBuf = file.clone();
        name.set_extension("");

        let temp_file = format!(
            "{}-temp{}", 
            name.display(),
            match file.extension() {
                Some(ext) => format!(".{}", ext.display()),
                None => format!(""),
            }
        );
        println!("{}", temp_file);
        let temp_content = fs::read_to_string(temp_file).expect("Can't read content");
        
        let template = mustache::compile_str(&temp_content).expect("Can't compile str");

        let target = template.render_to_string(&data_content).expect("Can't render");
        fs::write(file, &target).expect("No such file");
    }
}

fn parse_config(config_file: PathBuf) -> Config {
    let config_content = fs::read_to_string(config_file).expect("No such file");
    let config: Config = serde_json::from_str(config_content.as_str()).unwrap();
    
    return config;
}

fn from_args() {
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

fn from_config() {
    let config = parse_config(PathBuf::from("/home/user/.config/muscat/config.json"));
    let targets = config.targets
        .iter()
        .map(|target| PathBuf::from(target.resolve()))
        .collect();
    
    execute(targets, PathBuf::from(config.data.resolve()));
}

fn main() {
    match File::open("~/.config/muscat/config.json".resolve()) {
        Ok(file) if file.metadata().unwrap().len() == 0 => {
            println!("Config file is empty, processing with args...");
            sleep(Duration::from_millis(500));
            from_args();
        }
        Ok(_) => {
            println!("Processing with config file...");
            sleep(Duration::from_millis(500));
            from_config();
        }
        Err(_) => {
            println!("Can't find config file, processing with args...");
            sleep(Duration::from_millis(500));
            from_args();
        }
    }
}
