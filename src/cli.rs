use crate::{
    printer::Printer,
    sdg::{SdgDo, SdgGet, SdgSet},
    upload::{UploadCommand, UploadFileCommand, UploadLocation, UploadSslCommand},
};
use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, ArgGroup, Parser, Subcommand};
use std::net::SocketAddr;
use tracing::{info, warn};

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
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
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

impl Cli {
    pub fn run(&self, mut printer: Printer) -> Result<()> {
        if self.usb {
            return Err(anyhow!("Usb connection type is not supported yet"));
        }
        match &self.command {
            Command::Get(c) => self.run_sdg_get(&c, &mut printer),
            Command::Set(c) => self.run_sdg_set(&c, &mut printer),
            Command::Do(c) => self.run_sdg_do(&c, &mut printer),
            Command::Upload(c) => match c {
                UploadCommand::File(c) => self.run_upload_file(&c, &mut printer),
                UploadCommand::Ssl(c) => self.run_upload_ssl(&c, &mut printer),
            },
        }
    }

    fn run_sdg_get(&self, c: &SdgGet, printer: &mut Printer) -> Result<()> {
        printer.write_bytes(c).context("writing get command")?;
        let response = printer.read_bytes().context("reading command response")?;
        if response.len() > 0 {
            println!("{}", String::from_utf8_lossy(&response).replace('"', ""));
        }
        Ok(())
    }

    fn run_sdg_set(&self, c: &SdgSet, printer: &mut Printer) -> Result<()> {
        printer
            .write_bytes(Into::<Vec<u8>>::into(c))
            .context("writing set command")
    }

    fn run_sdg_do(&self, c: &SdgDo, printer: &mut Printer) -> Result<()> {
        printer.write_bytes(c).context("writiing do command")
    }

    fn run_upload_file(&self, c: &UploadFileCommand, printer: &mut Printer) -> Result<()> {
        let buff: Vec<_> = c.try_into().context("reading file")?;
        printer.write_bytes(buff).context("writing file to printer")
    }

    fn run_upload_ssl(&self, c: &UploadSslCommand, printer: &mut Printer) -> Result<()> {
        let https_port = c.port;
        let should_reset = c.reset;

        info!("uploading ssl certs");

        info!("uploading ca file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.ca.clone(), "HTTPS_CA.NRD".into()),
            printer,
        )
        .context("ca file")?;

        info!("uploading cert file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.cert.clone(), "HTTPS_CERT.NRD".into()),
            printer,
        )
        .context("cert file")?;

        info!("uploading key file");
        self.run_upload_file(
            &UploadFileCommand::new(UploadLocation::E, c.key.clone(), "HTTPS_KEY.NRD".into()),
            printer,
        )
        .context("key file")?;

        info!("Enabling https setting");
        self.run_sdg_set(&SdgSet::new("ip.https.enable", "on"), printer)
            .context("enabling https setting")?;

        let https_port = https_port.to_string();
        info!("Setting https port to {https_port}");
        self.run_sdg_set(&SdgSet::new("ip.https.port", https_port), printer)
            .context("setting https port")?;

        if should_reset {
            info!("resetting printer");
            self.run_sdg_do(&SdgDo::new("device.reset", None), printer)
                .context("restarting printer")
        } else {
            warn!("skipping reset, updated settings won't apply until the printer is reset.");
            Ok(())
        }
    }
}
