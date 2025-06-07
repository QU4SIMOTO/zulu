use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;
use zulu::{cli::Cli, printer::Printer};

fn init_tracing(log_level: u8) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(match log_level {
            0 => "warn", // default
            1 => "info",
            2 => "debug",
            _ => "trace",
        }))
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);
    cli.run(Printer::new(cli.addr, cli.timeout))
}
