use clap::Parser;
use zulu::{
    cli::{Cli, Command},
    printer::Printer,
    upload::UploadCommand,
    Error, IntoSgd, IntoZpl,
};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut printer = Printer::new(cli.addr, cli.timeout);
    match cli.command {
        Command::Get(c) => {
            printer.write_bytes(&c.into_sgd())?;
            let response = printer.read_bytes()?;
            if response.len() > 0 {
                println!("{}", String::from_utf8_lossy(&response).replace('"', ""));
            }
            Ok(())
        }
        Command::Set(c) => printer.write_bytes(&c.into_sgd()),
        Command::Do(c) => printer.write_bytes(&c.into_sgd()),
        Command::Upload(c) => match c {
            UploadCommand::File(c) => printer.write_bytes(&c.into_zpl()?),
            UploadCommand::Ssl(c) => printer.write_bytes(&c.into_zpl()?),
        },
    }
}
