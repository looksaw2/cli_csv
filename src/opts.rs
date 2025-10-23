use std::{fmt, str::FromStr};

use anyhow::Ok;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
#[derive(Parser, Debug)]
#[command(name = "rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass" , about = "generate a password")]
    GenPass(GenPassOpts),
}

#[derive(Debug,Parser)]
pub struct GenPassOpts {
    #[arg(short,long,default_value_t = 18)]
    pub length: u8,
    #[arg(long,default_value_t = true)]
    pub uppercase: bool,
    #[arg(long,default_value_t = true)]
    pub lowercase: bool,
     #[arg(long,default_value_t = true)]
    pub number: bool,
     #[arg(long,default_value_t = true)]
    pub symbol: bool

}


#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YMAL,
    TOML,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short,long,value_parser=parse_format,default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short = 'H', long, default_value_t = true)]
    pub header: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    name: String,
    age: String,
    address: String,
}
fn verify_input_file(filename: &str) -> Result<String, anyhow::Error> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::anyhow!("File doesn't exist..........."))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::JSON => "json",
            OutputFormat::TOML => "toml",
            OutputFormat::YMAL => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::JSON),
            "toml" => Ok(OutputFormat::TOML),
            "yaml" => Ok(OutputFormat::YMAL),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }
}