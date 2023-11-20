use clap::Parser;

/// Find all git project in given folder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Search project recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Folder will be searched
    pub roots: Vec<String>,
}
