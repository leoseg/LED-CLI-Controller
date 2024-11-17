mod cli;
mod mqttclient;
mod command;

use clap::Parser;
use crate::cli::CliArgs;



fn main() {
    let args = CliArgs::parse();
    let command = command::get_command(args.command_name.as_str());
    command.execute(args.command_args).unwrap_or_else(|err| {
        println!("Error executing command: {}", err);
    });
}
