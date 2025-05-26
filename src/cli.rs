use crate::{
    sdg::{SdgDo, SdgGet, SdgSet},
    upload::UploadCommand,
};
use clap::{Parser, Subcommand};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The address of the printer.
    #[arg(long)]
    #[clap(default_value = "192.168.0.40:9100")]
    pub addr: SocketAddr,
    /// Timeout in seconds.
    #[arg(long, short)]
    #[clap(default_value_t = 5)]
    pub timeout: u64,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get(SdgGet),
    Set(SdgSet),
    Do(SdgDo),
    #[command(subcommand)]
    Upload(UploadCommand),
}
