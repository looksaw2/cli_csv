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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
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
fn verify_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
