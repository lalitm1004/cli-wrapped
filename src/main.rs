use std::io;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wrapped")]
enum Cli {
    // log a command (interal use)
    #[structopt(name = "log")]
    Log {
        #[structopt(name = "COMMAND", default_value = "")]
        command: String,
    },

    // clear log history
    #[structopt(name = "clear")]
    Clear,

    // default to displaying wrapped
    #[structopt(name = "display")]
    Display,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let opt = if args.len() <= 1 {
        Cli::Display
    } else {
        Cli::from_iter(args)
    };

    match opt {
        Cli::Log { command } => cli_wrapped::log_command(command)?,
        Cli::Clear => cli_wrapped::clear_log()?,
        Cli::Display => cli_wrapped::display_wrapped()?,
    }

    Ok(())
}