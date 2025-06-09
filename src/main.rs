use anyhow::Result;
use clap::Parser;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::EnvFilter;
use zulu::{Cli, Device};

fn init_tracing(log_level: u8) {
    let writer = BoxMakeWriter::new(std::io::stderr);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(match log_level {
            0 => "warn", // default
            1 => "info",
            2 => "debug",
            _ => "trace",
        }))
        .with_writer(writer)
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);
    cli.run(Device::new(cli.addr, cli.timeout))
}
