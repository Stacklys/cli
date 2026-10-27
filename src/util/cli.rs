use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stacklys")]
#[command(about = "CLI oficial de Stacklys con Houston")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Add {
    #[command(subcommand)]
    entity: AddSubcommand,
  },
}

#[derive(Subcommand)]
pub enum AddSubcommand {
  Editorconfig,
}
