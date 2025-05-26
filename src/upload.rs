use clap::{Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::{Error, IntoZpl};

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
    #[arg(short, long, value_enum, default_value_t = UploadLocation::B)]
    pub loc: UploadLocation,

    /// The path of the file to upload.
    pub file: std::path::PathBuf,

    /// The destination filename to write to on the device, e.g., HTTPC_CA.NRD.
    pub dest: String,
}

impl IntoZpl for UploadFileCommand {
    fn into_zpl(self: Self) -> Result<Vec<u8>, Error> {
        let data = fs::read(&self.file)?;
        let size = data.len();
        let data_format = "B"; // todo support other formats:  B (raw binary format), C (AR compressed), and P (hexadecimal format PNG data)
        let dest_ext = self.dest.split(".").last().unwrap_or("");
        let row_bytes = ""; // only applies to images
        let header = format!(
            "~DY{}:{},{data_format},{dest_ext},{size},{row_bytes},",
            self.loc, self.dest
        );
        let mut buffer = header.into_bytes();
        buffer.extend_from_slice(&data);
        buffer.extend_from_slice(b"\r\n");
        Ok(buffer)
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
}

impl IntoZpl for UploadSslCommand {
    fn into_zpl(self: Self) -> Result<Vec<u8>, Error> {
        Ok([
            UploadFileCommand {
                loc: UploadLocation::B,
                file: self.ca,
                dest: "HTTPS_CA.NRD".into(),
            }
            .into_zpl()?,
            UploadFileCommand {
                loc: UploadLocation::B,
                file: self.cert,
                dest: "HTTPS_CERT.NRD".into(),
            }
            .into_zpl()?,
            UploadFileCommand {
                loc: UploadLocation::B,
                file: self.key,
                dest: "HTTPS_KEY.NRD".into(),
            }
            .into_zpl()?,
        ]
        .concat())
    }
}

#[derive(Subcommand, Debug)]
pub enum UploadCommand {
    File(UploadFileCommand),
    Ssl(UploadSslCommand),
}

impl IntoZpl for UploadCommand {
    fn into_zpl(self: Self) -> Result<Vec<u8>, Error> {
        match self {
            Self::File(c) => c.into_zpl(),
            Self::Ssl(c) => c.into_zpl(),
        }
    }
}
