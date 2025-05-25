use clap::Parser;
use zulu::{
    cli::{Cli, Command},
    printer::Printer,
    sdg::SdgCommand,
    upload::{UploadCommand, UploadLocation},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut printer = Printer::new(cli.addr, cli.timeout);
    match cli.command {
        Command::Sdg(c) => printer.run_sdg(c),
        Command::Upload(c) => match c {
            UploadCommand::File { loc, file, dest } => {
                printer.upload_file(loc, file, dest.as_str())
            }
            UploadCommand::Ssl { ca, cert, key } => {
                printer.upload_file(UploadLocation::B, ca, "HTTPS_CA.NRD")?;
                printer.upload_file(UploadLocation::B, cert, "HTTPS_CERT.NRD")?;
                printer.upload_file(UploadLocation::B, key, "HTTPS_KEY.NRD")?;
                printer.run_sdg(SdgCommand::Set {
                    key: "ip.http.enable".into(),
                    value: "on".into(),
                })?;
                printer.run_sdg(SdgCommand::Set {
                    key: "ip.http.port".into(),
                    value: "443".into(),
                })?;
                printer.run_sdg(SdgCommand::Do {
                    key: "device.reset".into(),
                    value: None,
                })
            }
        },
    }
}
