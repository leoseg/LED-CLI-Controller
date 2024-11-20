mod commands;
mod mqttclient;

use clap::Parser;
use crate::commands::CommandCLI;
fn main() {
    let mut cli_args = CommandCLI::parse();
    cli_args.execute();
}
