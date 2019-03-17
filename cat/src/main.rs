use cat::cli::Cli;
use structopt::StructOpt;

fn main() {
    let cli: Cli = Cli::from_args();

    println!("{:#?}", cli);
}
