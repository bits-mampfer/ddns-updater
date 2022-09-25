
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Records {
    pub domain: String,
    pub key: String,
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: ClapCommand,

    /// the interval to update in hours, set to 0 to update only once for use in e.g. crontab
    #[clap(short, long, value_parser, default_value_t = 24)]
    pub interval: u8,
}

#[derive(Subcommand)]
pub enum ClapCommand {
    /// run the updater
    Run,
    /// add a domain and associated key to keep updated
    Add {
        #[clap(value_parser)]
        domain: String,
        #[clap(value_parser)]
        key: String,
    },
    /// remove a doman from being updated
    Remove { domain: String }, // (domain)
}

pub fn clap_initialize() -> Args {
    let args = Args::parse();
    return args;
}
