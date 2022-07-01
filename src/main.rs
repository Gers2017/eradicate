use std::{error::Error, path::{PathBuf}, io, fs};
use glob::{glob_with, MatchOptions};
use argh;
use argh::FromArgs;
use ansi_term::Colour::{Cyan, Purple, Blue};

#[derive(FromArgs)]
/// Eradicate files/directories with patterns and no mercy
pub struct Eradicate {
    /// the pattern to use for the eradication
    #[argh(positional)]
    pub pattern: String,

    /// interactive mode. Slow and Steady
    #[argh(switch, short = 'i')]
    pub interactive: bool,

    /// ignore case search
    #[argh(switch, short = 'g')]
    pub ignore_case: bool,

    /// forcefully delete entries. Don't ask, just do it
    #[argh(switch, short = 'f')]
    pub force: bool,

    /// verbose output
    #[argh(switch, short = 'v')]
    pub verbose: bool,
}

type ResultBox<T> = Result<T, Box<dyn Error>>;

fn main() -> ResultBox<()> {
    let erad: Eradicate = argh::from_env();
    let Eradicate { pattern, verbose, interactive, force, ignore_case } = erad;
    println!("Searching with pattern {}", paint_purple(pattern.to_string()));
    
    let options = MatchOptions {
        case_sensitive: !ignore_case,
        ..Default::default()
    };

    let paths: Vec<PathBuf> = glob_with(&pattern, options)?.filter_map(Result::ok).collect();
      
    if paths.len() > 0 {
        eradicate(&paths, verbose, interactive, force)?
    } else {
        println!("No entries match the pattern {}", paint_purple(pattern.to_string()));
    }

    return Ok(());
}

fn eradicate(paths: &Vec<PathBuf>, verbose: bool, interactive: bool, force: bool) -> ResultBox<()> {
    let length = paths.len();
    println!("Entries to eradicate: {}", paint_purple(length.to_string()));

    if verbose {
        let paths_str:Vec<_> = paths.iter().map(|p| p.to_string_lossy()).collect();
        println!("{}", paint_blue(paths_str.join("  \n")));
    }

    if force {
        delete_paths(&paths)?;
        return Ok(());
    }
    
    if interactive {
        for path in paths.iter() {
            let cyan_filepath = paint_cyan(path.display().to_string());
            let is_delete = prompt(format!("Eradicate {}?", cyan_filepath));
            if is_delete {
                delete_path(path)?;
                if verbose { println!("{} was eradicated!", cyan_filepath); }
            }
        }
        return Ok(());
    }

    let is_delete = prompt("Eradicate entries?".into());
    if is_delete {
        delete_paths(&paths)?;
        println!("Eradication completed!");
    } else {
        println!("No entries were eradicated");
    }
    
    Ok(())
}

fn delete_paths(paths: &Vec<PathBuf>) -> io::Result<()> {
    for p in paths {
        delete_path(p)?;
    }
    Ok(())
}

fn delete_path(p: &PathBuf) -> io::Result<()> {
    if p.is_file() {
        fs::remove_file(p)?;
    } else {
        fs::remove_dir_all(p)?;
    }
    Ok(())
}

fn prompt(message: String) -> bool {
    println!("{} [Y/n]", message);
    let mut buffer = String::new();
    let stdin = io::stdin();
    
    if let Err(e) = stdin.read_line(&mut buffer) {
        eprintln!("{}", e);
        return false;
    }

    let answer = buffer.to_lowercase().replace("\n", "");
    match answer.trim() {
        "y" | "yes" => {
            true
        },
        _ => {
            false
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
