use clap::Parser;
use expanduser::expanduser;
use std::io;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Base directory
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    base_dir: String,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    let base_dir = expanduser(cli.base_dir)?;
    std::env::set_current_dir(base_dir)?;
    let walker = WalkDir::new(".").into_iter().filter_entry(|e| {
        if !e.file_type().is_dir() {
            return false;
        }
        if e.path().join(".git").exists() {
            // print the directory, but do not recurse into it
            println!("{}", e.path().display().to_string());
            return false;
        }
        true
    });
    for _ in walker {}
    Ok(())
}
