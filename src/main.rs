// src/main.rs
/*
 * Main executable for MicroPlatform
 */

use clap::Parser;
use microplatform::{Result, run};

#[derive(Parser)]
#[command(version, about = "MicroPlatform - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
