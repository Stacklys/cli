mod constant;
mod util;

use crate::util::Logger;
use crate::util::cli::{AddSubcommand, Cli, Commands};
use crate::util::create_template;
use clap::Parser;

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Add { entity } => match entity {
      AddSubcommand::Editorconfig => {
        Logger.info("Creando .editorconfig");
        create_template("editorconfig");
      }
    },
  }
}
