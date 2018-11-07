extern crate json5;
extern crate pretty;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate structopt;

mod error;
mod prettify;

use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

use self::error::Error;
use self::prettify::value;

use json5::Deserializer as Json5Deserializer;
use pretty::Arena;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, StructOpt)]
#[structopt(name = "jsonpp5", about = "jsonpp5 usage")]
struct Opt {
    /// Use stdin as input
    #[structopt(long)]
    stdin: bool,

    /// Indentation spaces
    #[structopt(short = "i", long = "indent", default_value = "2")]
    indent: usize,

    /// Max line width
    #[structopt(short = "w", long = "--max-line-width", default_value = "80")]
    line_width: usize,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

impl Opt {
    fn with_input<T, E, F>(&self, mut f: F) -> Result<T, Error>
    where
        F: FnMut(&mut io::Read) -> Result<T, E>,
        E: Into<Error>,
    {
        if self.stdin {
            let stdin = io::stdin();
            let mut handler = stdin.lock();
            f(&mut handler).map_err(|e| e.into())
        } else if let Some(ref path) = self.input {
            let mut file = File::open(path)?;
            f(&mut file).map_err(|e| e.into())
        } else {
            Err(Error::NoInput)
        }
    }

    fn with_output<T, E, F>(&self, mut f: F) -> Result<T, Error>
    where
        F: FnMut(&mut io::Write) -> Result<T, E>,
        E: Into<Error>,
    {
        if let Some(ref path) = self.output {
            let mut file = OpenOptions::new().write(true).create(true).open(path)?;
            f(&mut file).map_err(|e| e.into())
        } else {
            let stdout = io::stdout();
            let mut handler = stdout.lock();
            f(&mut handler).map_err(|e| e.into())
        }
    }
}

fn run(opt: Opt) -> Result<(), Error> {
    let mut input = String::new();

    opt.with_input(|r| r.read_to_string(&mut input))?;

    let mut de = Json5Deserializer::from_str(&input)?;
    let js_val = Value::deserialize(&mut de)?;
    let max_line_width = opt.line_width;

    opt.with_output(|out| {
        value::<_, ()>(&js_val, &Arena::new())
            .1
            .render(max_line_width, out)
    })?;

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    match run(opt) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
