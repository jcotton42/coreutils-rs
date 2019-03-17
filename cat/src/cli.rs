use structopt::StructOpt;
use std::path::PathBuf;
use crate::{Options, NumberingMode};

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Cli {
    /// Equivalent to -vET
    #[structopt(short = "A", long = "show-all")]
    show_all: bool,

    /// Number all nonempty output lines, starting with 1
    #[structopt(short = "b", long = "number-nonblank")]
    number_nonblank: bool,

    /// Equivalent to -vE
    #[structopt(short = "e")]
    show_ends_and_nonprinting: bool,

    /// Display a ‘$’ after the end of each line
    #[structopt(short = "E", long = "show-ends")]
    show_ends: bool,

    /// Number all output lines, starting with 1. This option is ignored if -b is in effect
    #[structopt(short = "n", long = "number")]
    number_all_lines: bool,

    /// Suppress repeated adjacent blank lines; output just one empty line instead of several
    #[structopt(short = "s", long = "squeeze-blank")]
    squeeze_blank: bool,

    /// Equivalent to -vT
    #[structopt(short = "t")]
    show_tabs_and_nonprinting: bool,

    /// Display TAB characters as ‘^I’
    #[structopt(short = "T", long = "show-tabs")]
    show_tabs: bool,

    /// Ignored; for POSIX compatibility
    #[structopt(short = "u")]
    _ignored: bool,

    /// Display control characters except for LFD and TAB using ‘^’ notation and precede characters that have the high bit set with ‘M-’
    #[structopt(short = "v", long = "show-nonprinting")]
    show_nonprinting: bool,

    /// Input files
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

