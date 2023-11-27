use clap::Parser;

/// Find all parent folder that has given folder or file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Search folder recursively
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// Max threads usage, default set to system logical CPU cores
    #[arg(
        short = 'P',
        long,
        value_parser = clap::builder::RangedU64ValueParser::<usize>::new().range(1..255)
    )]
    pub parallel: Option<usize>,

    /// Subfolder or filename to search, any folder has the given folder or file will return
    #[arg(long = "has", default_value = ".git", value_name = "FOLDER_OR_FILE")]
    pub search_file: String,

    /// Search roots
    pub roots: Vec<String>,
}
