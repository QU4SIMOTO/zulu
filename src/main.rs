use clap::Parser;
use zulu::{
    cli::{Cli, Command},
    printer::Printer,
    upload::UploadCommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut printer = Printer::new(cli.addr, cli.timeout);
    match cli.command {
        Command::Sdg(c) => printer.run_sdg(c),
        Command::Upload(c) => match c {
            UploadCommand::File { loc, file, dest } => printer.upload_file(loc, file, dest),
            UploadCommand::Ssl { .. } => {
                // upload HTTPS_CA.NRD
                // upload HTTPS_CERT.NRD
                // upload HTTPS_KEY.NRD
                todo!()
            }
        },
    }
}
