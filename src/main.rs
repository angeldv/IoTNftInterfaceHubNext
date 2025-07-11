// src/main.rs
/*
 * Main executable for IoTNftInterfaceHubNext
 */

use clap::Parser;
use iotnftinterfacehubnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "IoTNftInterfaceHubNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
