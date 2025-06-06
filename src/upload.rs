use anyhow::Context;
use clap::{Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, trace};

#[derive(Debug, Clone, ValueEnum)]
pub enum UploadLocation {
    /// RAM
    R,
    /// Flash
    E,
    /// PCMCIA
    B,
}

impl std::fmt::Display for UploadLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R => write!(f, "R"),
            Self::E => write!(f, "E"),
            Self::B => write!(f, "B"),
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct UploadFileCommand {
    /// Device location on the device to write the file to: r ram, e flash, b PCMCIA
    #[arg(short, long, value_enum, default_value_t = UploadLocation::E)]
    pub loc: UploadLocation,

    /// The path of the file to upload.
    pub file: std::path::PathBuf,

    /// The destination filename to write to on the device, e.g., HTTPC_CA.NRD.
    pub dest: String,
}

impl UploadFileCommand {
    pub fn new(loc: UploadLocation, file: std::path::PathBuf, dest: String) -> Self {
        Self { loc, file, dest }
    }
}

impl TryInto<Vec<u8>> for &UploadFileCommand {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<Vec<u8>, Self::Error> {
        let data =
            fs::read(&self.file).with_context(|| format!("reading input file {:?}", self.file))?;
        trace!("Read file {:?}", self.file);

        let size = data.len();
        let data_format = "B"; // todo support other formats:  B (raw binary format), C (AR compressed), and P (hexadecimal format PNG data)
        let dest_ext = self.dest.split(".").last().unwrap_or("");
        let row_bytes = ""; // only applies to images
        let header = format!(
            "~DY{}:{},{data_format},{dest_ext},{size},{row_bytes},",
            self.loc, self.dest
        );
        let mut buf = header.into_bytes();
        buf.extend_from_slice(&data);
        buf.extend_from_slice(b"\r\n");
        debug!("upload file write data: {buf:?}");
        Ok(buf)
    }
}

impl TryInto<Vec<u8>> for UploadFileCommand {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        TryInto::<Vec<u8>>::try_into(&self)
    }
}

#[derive(Debug, clap::Args)]
pub struct UploadSslCommand {
    /// Path to CA file.
    pub ca: PathBuf,
    /// Path to crt file.
    pub cert: PathBuf,
    /// Path to key file.
    pub key: PathBuf,
    /// The port to set https to listen to on the printer.
    #[arg(long, short)]
    #[clap(default_value = "443")]
    pub port: u16,
    /// Do NOT reset the printer after the operation.
    #[arg(long = "no-reset", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub reset: bool,
}

#[derive(Subcommand, Debug)]
pub enum UploadCommand {
    #[command(about = "Upload a file to the printer.")]
    File(UploadFileCommand),
    #[command(about = "Upload ssl certs and configure the printer to enable ssl.")]
    Ssl(UploadSslCommand),
}
