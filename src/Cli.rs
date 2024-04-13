use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(
    author,
    version,
    about,
    long_about = None,
    subcommand_required = false
)]
pub struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommands>,

    #[clap(
        short,
        long,
        required = false,
        help = "Ultimate Encode, Ecrypt and Hashing",
        value_name = "plain text"
    )]
    all: Option<String>,
}

impl Args {
    pub fn sub_command(&self) -> &Option<SubCommands> {
        &self.sub_command
    }
    pub fn all(&self) -> &Option<String> {
        &self.all
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {}
