use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    pub command_name: String,
    pub command_args: Vec<String>,
}
