use std::{path::PathBuf, sync::Arc};

use clap::Parser;

mod args;
mod finder;

use args::Args;
use finder::Finder;

fn main() {
    let args = Args::parse();

    let roots = args.roots.clone();
    let search = args.search_file.clone();
    let filter = move |path: &PathBuf| path.join(search.as_str()).exists();

    let finder = Finder::new(roots, Arc::new(filter), args.into());

    for proj in finder {
        println!("{}", proj);
    }
}
