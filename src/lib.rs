use clap::Parser;
mod sub_commands;

#[derive(Parser)] // requires `derive` feature
#[command(name = "onece-exec")]
#[command(bin_name = "onece-exec")]
#[command(about = "*************************************\n********** onece-exec **********\n*************************************")]
enum Cli {
  // Subcommands...
  Run(sub_commands::c_run::CliArgs), 
}

pub fn run() {
  let parse_cli = Cli::parse();
  match parse_cli {
    Cli::Run(args) => sub_commands::c_run::run(args),
  }
}