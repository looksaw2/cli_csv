use ::std::fs;
use clap::Parser;

use ::cli_app::{process_csv, Opts, Player, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?
    }
    Ok(())
}
