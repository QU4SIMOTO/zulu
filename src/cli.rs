use crate::{
    sdg::{SdgDo, SdgGet, SdgSet},
    upload::UploadCommand,
};
use clap::{ArgGroup, Parser, Subcommand};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(
    name = "Zebra Utilities for Linux Users",
    version = "0.1.0",
    about = "A small cli to make interacting with zebra printers on linux slightly less annoying."
)]
#[command(group(
    ArgGroup::new("connection")
        .args(["addr", "usb"])
))]
pub struct Cli {
    /// The address of the printer.
    #[arg(long)]
    #[clap(default_value = "192.168.0.40:9100")]
    pub addr: SocketAddr,
    /// Connect to printer using usb. NOT SUPPORTED currently
    #[arg(long)]
    pub usb: bool,
    /// Timeout in seconds for network operations.
    #[arg(long, short)]
    #[clap(default_value_t = 5)]
    pub timeout: u64,
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Get a configuration value by key.")]
    Get(SdgGet),
    #[command(about = "Set a configuration value by key.")]
    Set(SdgSet),
    #[command(about = "Perform an action by name.")]
    Do(SdgDo),
    #[command(
        subcommand,
        about = "Upload files such as firmware, certificates or keys."
    )]
    Upload(UploadCommand),
}
