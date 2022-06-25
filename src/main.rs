use std::{error::Error, path::{PathBuf}, io, fs};
use glob::glob;
use argh;
use argh::FromArgs;
use ansi_term::Colour::{Cyan, Purple, Blue};

#[derive(FromArgs)]
/// Eradicate files with patterns and no mercy
pub struct Eradicate {
    /// the pattern to use for the eradication
    #[argh(positional)]
    pub pattern: String,

    /// interactive mode. Slow and Steady
    #[argh(switch, short = 'i')]
    pub interactive: bool,

    /// forcefully delete files. Don't ask, just do it
    #[argh(switch, short = 'f')]
    pub force: bool,

    /// verbose output
    #[argh(switch, short = 'v')]
    pub verbose: bool,
}

type ResultBox<T> = Result<T, Box<dyn Error>>;

fn main() -> ResultBox<()> {
    let erad: Eradicate = argh::from_env();
    let pattern = erad.pattern;
    let verbose = erad.verbose;
    let interactive = erad.interactive;
    let force = erad.force;

    println!("Searching files with pattern {}", paint_purple(pattern.to_string()));
   
    let paths :Vec<PathBuf> = glob(&pattern)?.filter_map(Result::ok).collect();
      
    if paths.len() > 0 {
        eradicate(&paths, verbose, interactive, force)?
    } else {
        println!("No files match the pattern {}", paint_purple(pattern.to_string()));
    }

    return Ok(());
}

fn eradicate(paths: &Vec<PathBuf>, verbose: bool, interactive: bool, force: bool) -> ResultBox<()> {
    let length = paths.len();
    println!("Files to eradicate: {}", paint_purple(length.to_string()));

    if verbose {
        let paths_str:Vec<_> = paths.iter().map(|p| p.to_string_lossy()).collect();
        println!("{}", paint_blue(paths_str.join("  \n")));
    }

    if force {
        delete_files(&paths)?;
        return Ok(());
    }
    
    if interactive {
        for path in paths.iter() {
            let cyan_filepath = paint_cyan(path.display().to_string());
            let is_delete = prompt(format!("Eradicate {}?", cyan_filepath))?;
            if is_delete {
                fs::remove_file(path)?;
                if verbose { println!("{} was eradicated!", cyan_filepath); }
            }
        }
        return Ok(());
    }

    let is_delete = prompt("Eradicate files?".into())?;
    if is_delete {
        delete_files(&paths)?;
        println!("Eradication completed!");
    } else {
        println!("No files were eradicated");
    }
    
    Ok(())
}

fn delete_files(paths: &Vec<PathBuf>) -> io::Result<()> {
    for p in paths {
        fs::remove_file(p)?;
    }
    Ok(())
}

fn prompt(message: String) -> io::Result<bool> {
    println!("{} [Y/n]", message);
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

fn paint_purple(s: String) -> String {
    Purple.bold().paint(s).to_string()
}

fn paint_blue(s: String) -> String {
    Blue.paint(s).to_string()
}

fn paint_cyan(s: String) -> String {
    Cyan.bold().paint(s).to_string()
}
