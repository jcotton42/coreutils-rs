pub mod cli;

use std::path::PathBuf;

pub enum NumberingMode {
    None,
    NonBlank,
    All,
}

pub struct Options {
    /// Display a ‘$’ after the end of each line
    show_ends: bool,

    /// Suppress repeated adjacent blank lines; output just one empty line instead of several
    squeeze_blank: bool,

    /// Display TAB characters as ‘^I’
    show_tabs: bool,

    /// Display control characters except for LFD and TAB using ‘^’ notation and precede characters that have the high bit set with ‘M-’
    show_nonprinting: bool,

    /// Which lines to number
    numbering_mode: NumberingMode,

    /// Input files
    files: Vec<PathBuf>,
}
