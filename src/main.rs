use anyhow::{Context, Result};
use clap::Parser;
use zulu::{
    cli::{Cli, Command},
    printer::Printer,
    sdg::{SdgDo, SdgSet},
    upload::UploadCommand,
    IntoSgd, IntoZpl,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut printer = Printer::new(cli.addr, cli.timeout);
    match cli.command {
        Command::Get(c) => {
            printer
                .write_bytes(&c.into_sgd())
                .context("writing get command")?;
            let response = printer.read_bytes().context("reading command response")?;
            if response.len() > 0 {
                println!("{}", String::from_utf8_lossy(&response).replace('"', ""));
            }
            Ok(())
        }
        Command::Set(c) => printer
            .write_bytes(&c.into_sgd())
            .context("writing set command"),
        Command::Do(c) => printer
            .write_bytes(&c.into_sgd())
            .context("writiing do command"),
        Command::Upload(c) => match c {
            UploadCommand::File(c) => printer
                .write_bytes(&c.into_zpl().context("reading upload file")?)
                .context("writing file to printer"),
            UploadCommand::Ssl(c) => {
                let https_port = c.port;
                let should_reset = c.reset;
                printer
                    .write_bytes(&c.into_zpl().context("reading ssl file")?)
                    .context("writing file to printer")?;
                printer
                    .write_bytes(
                        &SdgSet {
                            key: "ip.https.enable".into(),
                            value: "on".into(),
                        }
                        .into_sgd(),
                    )
                    .context("enabling https setting")?;
                printer
                    .write_bytes(
                        &SdgSet {
                            key: "ip.https.port".into(),
                            value: https_port.to_string(),
                        }
                        .into_sgd(),
                    )
                    .context("setting https port")?;
                if should_reset {
                    printer
                        .write_bytes(
                            &SdgDo {
                                key: "device.reset".into(),
                                value: None,
                            }
                            .into_sgd(),
                        )
                        .context("restarting printer")?;
                }
                Ok(())
            }
        },
    }
}
