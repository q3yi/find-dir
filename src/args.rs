use clap::Parser;

/// Find all git project in given folder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Search project recursively
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// Max threads usage
    #[arg(
        short = 'P',
        long,
        value_parser = clap::builder::RangedU64ValueParser::<usize>::new().range(1..255)
    )]
    pub parallel: Option<usize>,

    /// Folder will be searched
    pub roots: Vec<String>,
}
