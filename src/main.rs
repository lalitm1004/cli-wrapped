use std::io;

use cli_wrapped::{
    clear_logs,
    display_wrapped,
    log_command,
    cli::args::Cli,
};
use structopt::StructOpt;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let opt = if args.len() <= 1 {
        Cli::Display
    } else {
        Cli::from_iter(args)
    };

    match opt {
        Cli::Log { command } => log_command(command)?,
        Cli::Clear => clear_logs()?,
        Cli::Display => display_wrapped()?,
    }

    Ok(())
}