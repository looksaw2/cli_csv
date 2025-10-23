use ::std::fs;

use crate::{Opts, Player, Subcommand};
use clap::Parser;
use csv::Reader;

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}
