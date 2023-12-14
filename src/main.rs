use std::sync::Arc;

use clap::Parser;

mod args;
mod finder;

use args::Args;

use finder::{Config, Filter, Finder};

fn main() {
    let args = Args::parse();

    let roots = args.roots.clone();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.parallel.unwrap_or(num_cpus::get()))
        .build()
        .unwrap();
    let pool = Arc::new(pool);

    let finder = Finder::new(
        pool,
        Filter::new().has(args.search_file.clone()),
        Config::new(args.recursive),
    );

    for proj in finder.scan(roots).into_iter() {
        println!("{}", proj);
    }
}
