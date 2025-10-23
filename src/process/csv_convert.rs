use ::std::fs;

use crate::{Opts, Player, Subcommand, opts::OutputFormat};
use clap::Parser;
use csv::Reader;
use serde_json::Value;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::JSON => serde_json::to_string_pretty(&ret)?,
        OutputFormat::YMAL => serde_yaml::to_string(&ret)?,
        OutputFormat::TOML => toml::to_string(&ret)?
    };
    fs::write(output, content)?;
    Ok(())
}
