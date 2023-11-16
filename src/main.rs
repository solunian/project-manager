use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod handle;

#[derive(Parser)]
#[command(author, version, about, long_about=None, arg_required_else_help=true)]
struct CLI {
  #[command(subcommand)]
  command: Option<Commands>,

  // /// Sets a custom config file
  // #[arg(short, long, value_name = "FILE")]
  // config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
  #[command()]
  /// Initialize tart
  Init {
    /// Path of tart initialization.
    directory: Option<PathBuf>,
  },
  #[command()]
  /// Destroy tart
  Destroy {
    directory: Option<PathBuf>,
  }
}

fn main() {
  let cli = CLI::parse();

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  match &cli.command {
    Some(Commands::Init { directory }) => {
      println!("{}", handle::init(directory.as_ref()));
    },
    Some(Commands::Destroy { directory }) => {
      println!("{}", handle::destroy(directory.as_ref()));
    },
    None => {}
  }
}
