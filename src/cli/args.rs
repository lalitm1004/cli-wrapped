use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wrapped")]
pub enum Cli {
    #[structopt(name = "log")]
    Log {
        #[structopt(name = "COMMAND", default_value = "")]
        command: String,
    },

    #[structopt(name = "clear")]
    Clear,

    #[structopt(name = "display")]
    Display,
}