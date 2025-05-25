use clap::{Subcommand, ValueEnum};

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

#[derive(Subcommand, Debug)]
pub enum UploadCommand {
    File {
        /// Device location on the device to write the file to: r ram, e flash, b PCMCIA
        #[arg(short, long, value_enum, default_value_t = UploadLocation::B)]
        loc: UploadLocation,
        /// The path of the the file to upload.
        #[arg(short, long, value_name = "FILE")]
        file: std::path::PathBuf,
        /// The destinication filename to write to on the device eg. HTTPC_CA.NRD.
        dest: String,
    },
    Ssl {
        ca: String,
        cert: String,
        key: String,
    },
}
