use std::{
    env, fs::File, io::{BufRead, BufReader}, path::PathBuf
};

use shutil::pipe;

fn collect_from_file(file: &File) -> Vec<PathBuf> {
    let reader = BufReader::new(file);
    let mut items = Vec::new();

    for line in reader.lines() {
        items.push(PathBuf::from(line.unwrap().as_str()));
    }

    return items;
}

fn execute(paths: Vec<PathBuf>, data: &str) {
    for file in paths {    
        let mut name = file.clone();
        name.set_extension("");

        let parent = file.parent().expect("No such directory");

        // println!("{:?} {:?}", parent, name);
        println!("{}", 
            format!(
                "{}-temp.{}", 
                name.display(),
                file.extension().expect("Don't have extension").display(),
            )
        );

        pipe(vec![
            vec![
                "mustache",
                data,
                parent.join(
                    format!(
                        "{}-temp.{}", 
                        name.display(),
                        file.extension().expect("Don't have extension").display(),
                    )
                ).to_str().expect("No such file"),
            ],

            vec![
                "tee",
                file.to_str().expect("No such file")
            ],
        ]).expect("Can't execute");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut data = "";  
    let mut paths: Vec<PathBuf> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match (args[i].trim(), args[i+1].trim()) {
            ("--targets", targets) => {
                println!("{} {}", args[i], args[i+1]);
                let file = File::open(targets).unwrap();
                paths = collect_from_file(&file);
                i += 2;
            }
            ("--from", data_file) => {
                println!("{} {}", args[i], args[i+1]);
                data = data_file;
                i += 2;
            }
            _ => break
        }
    }

    println!("-------------------------------------------------------");

    execute(paths, data);
}
