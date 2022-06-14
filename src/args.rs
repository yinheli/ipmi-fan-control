use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Subcommands
    #[clap(subcommand)]
    pub command: Command,

    /// Verbose output
    #[clap(long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Auto adjust fan speed by interval checking CPU temperature
    Auto(Auto),

    /// Set fixed RPM percentage for fan
    Fixed {
        /// value range 0-100
        #[clap(value_parser)]
        value: u16,
    },

    /// Print CPU temperature and fan RPM
    Info,
}

#[derive(clap::Args)]
pub struct Auto {
    /// check CPU temperature interval second
    #[clap(short, long, default_value = "5")]
    pub interval: u64,

    /// threshold CPU temperature for full speed Fan, default 75 (degrees), accepted value range [60-100]
    #[clap(short, long, default_value = "80")]
    pub threshold: u16,
}
