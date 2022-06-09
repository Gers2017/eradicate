use std::{error::Error, env::args, path::PathBuf, io, fs};
use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args();
    let vec_args: Vec<String> = args.collect();
    let pattern = vec_args.get(1).map_or("**/*.rs", |x| x.as_str());
    
    let mut paths: Vec<PathBuf> = vec![];

    println!("Searching: [ {} ]", pattern);
    for entry in glob(&pattern)? {
        let path = entry?.to_owned();
        paths.push(path);
    }
    
    if paths.len() > 0 {
        println!("Files to eradicate:");
        for p in paths.iter() {
            println!("{}", p.to_string_lossy());
        }

        println!("Delete files? [Y/n]");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let answer = buffer.to_lowercase().replace("\n", "");
        match answer.trim() {
            "y" => {
                println!("Files were eradicated");
                delete_files(&paths)?
            },
            _ => {
                println!("No eradication");
            }
        };
    } else {
        println!("No files match the pattern");
    }

    return Ok(());
}

fn delete_files(paths: &Vec<PathBuf>) -> io::Result<()> {
    for p in paths {
        fs::remove_file(p)?;
    }
    Ok(())
}
