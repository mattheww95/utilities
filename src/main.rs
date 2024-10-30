//! Some generic utilities to tackle some daily issues I hav
//! Matthew Wells: 2024-10-30

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use std::env::current_dir;
use scandir::Count;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {

    /// Directory to count files in
    #[command(subcommand)]
    command: Option<Commands>,
    
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// Count files in a directory matching a specific pattern
    CountFiles {
        #[arg()]
        directory: Option<PathBuf>,

        #[arg(long, short)]
        pattern: Option<String>,
    },
}


fn count_files_directory(path: &PathBuf, match_pattern: Option<Vec<String>>) -> () {
    let instance = Count::new(path).unwrap().file_include(match_pattern).max_depth(1).collect();

    //let _results = instance.results();
    println!("{:#?}", instance.unwrap());
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::CountFiles { directory, pattern }) => {
            let work_dir: PathBuf;
            if let Some(dir) = directory { 
                if !dir.is_dir() {
                    panic!("Argument passed is not a directory: {:?}", dir)
                }
                work_dir = dir.clone();
            }else{
                work_dir = dbg!(current_dir().ok().unwrap());
            }
            
            let mut include_pat: Option<Vec<String>> = None;
            if let Some(match_pattern) = pattern {
                include_pat = Some(vec![match_pattern.clone()]);
            }
            count_files_directory(&work_dir, include_pat);

        }
        None => {}
    }
}
