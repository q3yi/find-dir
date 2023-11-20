use clap::Parser;

mod args;
mod finder;

use args::Args;
use finder::GitFinder;


fn main() {
    let args = Args::parse();
    let finder = GitFinder::new(args.roots, args.recursive);
    for proj in finder {
        println!("{}", proj)
    }
}
