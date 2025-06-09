use crate::{
    sdg::{SdgDo, SdgGet, SdgSet},
    upload::{UploadCommand, UploadFileCommand, UploadLocation, UploadSslCommand},
    Device,
};
use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, ArgGroup, Parser, Subcommand};
use std::net::SocketAddr;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "Zebra Utilities for Linux Users",
    version = "0.1.0",
    about = "A small cli to make interacting with zebra devices on linux slightly less annoying."
)]
#[command(group(
    ArgGroup::new("connection")
        .args(["addr", "usb"])
))]
pub struct Cli {
    /// The address of the device
    #[arg(long)]
    #[clap(default_value = "192.168.0.40:9100")]
    pub addr: SocketAddr,
    /// Connect to device using usb. NOT SUPPORTED currently
    #[arg(long)]
    pub usb: bool,
    /// Timeout in seconds for network operations
    #[arg(long, short)]
    #[clap(default_value_t = 5)]
    pub timeout: u64,
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Get a configuration value by key")]
    Get(SdgGet),
    #[command(about = "Set a configuration value by key")]
    Set(SdgSet),
    #[command(about = "Perform an action by name")]
    Do(SdgDo),
    #[command(
        subcommand,
        about = "Upload files such as firmware, certificates or keys"
    )]
    Upload(UploadCommand),
}

impl Cli {
    pub fn run(&self, mut device: Device) -> Result<()> {
        if self.usb {
            return Err(anyhow!("Usb connection type is not supported yet"));
        }
        match &self.command {
            Command::Get(c) => self.run_sdg_get(&c, &mut device),
            Command::Set(c) => self.run_sdg_set(&c, &mut device),
            Command::Do(c) => self.run_sdg_do(&c, &mut device),
            Command::Upload(c) => match c {
                UploadCommand::File(c) => self.run_upload_file(&c, &mut device),
                UploadCommand::Ssl(c) => self.run_upload_ssl(&c, &mut device),
            },
        }
    }

    fn run_sdg_get(&self, c: &SdgGet, device: &mut Device) -> Result<()> {
        device.write_bytes(c).context("writing get command")?;
        let response = device.read_bytes().context("reading command response")?;
        if response.len() > 0 {
            println!("{}", String::from_utf8_lossy(&response).replace('"', ""));
        }
        Ok(())
    }

    fn run_sdg_set(&self, c: &SdgSet, device: &mut Device) -> Result<()> {
        device.write_bytes(c).context("writing set command")
    }

    fn run_sdg_do(&self, c: &SdgDo, device: &mut Device) -> Result<()> {
        device.write_bytes(c).context("writiing do command")
    }

    fn run_upload_file(&self, c: &UploadFileCommand, device: &mut Device) -> Result<()> {
        let buf: Vec<_> = c.try_into().context("reading file")?;
        device.write_bytes(buf).context("writing file to device")
    }

    fn run_upload_ssl(&self, c: &UploadSslCommand, device: &mut Device) -> Result<()> {
        let https_port = c.port;
        let should_reset = c.reset;

        info!("uploading ssl certs");

        info!("uploading ca file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.ca.clone(), "HTTPS_CA.NRD".into()),
            device,
        )
        .context("ca file")?;

        info!("uploading cert file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.cert.clone(), "HTTPS_CERT.NRD".into()),
            device,
        )
        .context("cert file")?;

        info!("uploading key file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.key.clone(), "HTTPS_KEY.NRD".into()),
            device,
        )
        .context("key file")?;

        info!("Enabling https setting");
        self.run_sdg_set(&SdgSet::new("ip.https.enable", "on"), device)
            .context("enabling https setting")?;

        let https_port = https_port.to_string();
        info!("Setting https port to {https_port}");
        self.run_sdg_set(&SdgSet::new("ip.https.port", https_port), device)
            .context("setting https port")?;

        if should_reset {
            info!("resetting device");
            self.run_sdg_do(&SdgDo::new("device.reset", None), device)
                .context("restarting device")
        } else {
            warn!("skipping reset, updated settings won't apply until the device is reset.");
            Ok(())
        }
    }
}
