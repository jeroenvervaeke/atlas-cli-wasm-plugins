use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Tests the host functionality
    TestHost,
    /// Prints the environment variables
    PrintEnv,
    /// Prints what was passed in stdin
    PrintStdIn,
}
