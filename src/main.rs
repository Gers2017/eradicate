use std::{error::Error, env::args, path::PathBuf, io, fs};
use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args();
    let vec_args: Vec<String> = args.collect();

    let pattern = vec_args.get(1).map_or("**/*.rs", |x| x.as_str());
    let forcefuly = false;
    let interactive = true;
    let verbose = true;
    
    let mut paths: Vec<PathBuf> = vec![];

    println!("Searching files with pattern {}", pattern);
    for entry in glob(&pattern)? {
        let path = entry?.to_owned();
        paths.push(path);
    }
    
    if paths.len() > 0 {
        if verbose {
            let paths_str:Vec<_> = paths.iter().map(|p| p.to_string_lossy()).collect();
            println!("Files to eradicate:\n{}", paths_str.join("  \n"));
        }

        if forcefuly {
            delete_files(&paths)?;
            return Ok(());
        }

        if interactive {
            for path in paths.iter() {
                println!("Eradicate {}?", path.display());
                let is_delete = prompt()?;
                if is_delete {
                    fs::remove_file(path)?
                }
            }
        } else {
            println!("Eradicate files? [Y/n]");
            let is_delete = prompt()?;
            if is_delete {
                println!("Eradication Completed!");
                delete_files(&paths)?
            } else {
                println!("No files were eradicated");
            }
        }
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

fn prompt() -> io::Result<bool> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let answer = buffer.to_lowercase().replace("\n", "");
    match answer.trim() {
        "y" | "yes" => {
            Ok(true)
        },
        _ => {
            Ok(false)
        }
    }
}