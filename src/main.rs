mod render;
mod scan;

use std::path::PathBuf;

use clap::Parser;

/// Show which files and folders take up the most disk space.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// How many entries to show
    #[arg(short = 'n', long, default_value_t = 20)]
    top: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let entries = scan::scan(&args.path)?;
    render::print(&args.path, &entries, args.top);
    Ok(())
}
