mod opts;
mod process;
pub use opts::{Opts, Player, Subcommand};
pub use process::process_csv;
pub use process::process_genpass;