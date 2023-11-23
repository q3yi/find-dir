use std::path::PathBuf;

use clap::Parser;

mod args;
mod finder;

use args::Args;
use finder::Finder;

fn main() {
    let args = Args::parse();

    let f = |path: &PathBuf| path.join(".git").is_dir();
    let entries = args.roots.clone();

    let finder = Finder::new(entries, f, args.into());

    for proj in finder {
        println!("{}", proj);
    }
}
