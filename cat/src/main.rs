use cat::cli::Cli;
use structopt::StructOpt;

fn main() {
    cat::run(Cli::from_args().normalize());
}
