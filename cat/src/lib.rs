pub mod cli;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub enum NumberingMode {
    None,
    NonBlank,
    All,
}

#[derive(Debug)]
pub struct Options {
    /// Display a ‘$’ after the end of each line
    pub show_ends: bool,

    /// Suppress repeated adjacent blank lines; output just one empty line instead of several
    pub squeeze_blank: bool,

    /// Display TAB characters as ‘^I’
    pub show_tabs: bool,

    /// Display control characters except for LFD and TAB using ‘^’ notation and precede characters that have the high bit set with ‘M-’
    pub show_nonprinting: bool,

    /// Which lines to number
    pub numbering_mode: NumberingMode,

    /// Input files
    pub files: Vec<PathBuf>,
}

pub fn run(options: Options) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = Vec::with_capacity(1024);

    for file in options.files {
        let mut f = BufReader::new(File::open(file).expect("TODO handle open failures"));

        loop {
            match f.read_until(b'\n', &mut buf) {
                Ok(0) => {
                    buf.clear();
                    break;
                }
                Ok(read) => {
                    stdout
                        .write(&buf[..read].as_ref())
                        .expect("TODO handle stdout write failures");
                    buf.clear();
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => todo!("Handle read failures"),
            }
        }
    }
}
