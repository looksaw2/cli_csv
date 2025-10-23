use ::std::fs;
use clap::Parser;

use cli_app::process_genpass;
use ::cli_app::{Opts, Player, Subcommand, process_csv};
use csv::Reader;
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                "output.json".into()
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        Subcommand::GenPass(opts) => {
            process_genpass(opts.length,opts.uppercase,opts.lowercase,opts.number,opts.symbol)?
        }
    }
    Ok(())
}
