extern crate json5;
extern crate pretty;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate structopt;

mod prettify;

use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

use self::prettify::value;

use json5::Deserializer as Json5Deserializer;
use pretty::Arena;
use serde::Deserialize;
use serde_json::Value;

struct Error;

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
    fn with_input<T, F>(&self, mut f: F) -> Result<T, Error>
    where
        F: FnMut(&mut io::Read) -> Result<T, Error>,
    {
        if self.stdin {
            let stdin = io::stdin();
            let mut handler = stdin.lock();
            f(&mut handler)
        } else if let Some(ref path) = self.input {
            let mut file = File::open(path).map_err(|_| Error)?;
            f(&mut file)
        } else {
            Err(Error)
        }
    }

    fn with_output<T, F>(&self, mut f: F) -> Result<T, Error>
    where
        F: FnMut(&mut io::Write) -> Result<T, Error>,
    {
        if let Some(ref path) = self.output {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .map_err(|e| {
                    eprintln!("{}", e);
                    Error
                })?;
            f(&mut file)
        } else {
            let stdout = io::stdout();
            let mut handler = stdout.lock();
            f(&mut handler)
        }
    }
}

fn run(opt: Opt) -> Result<(), Error> {
    let mut input = String::new();

    opt.with_input(|r| r.read_to_string(&mut input).map_err(|_| Error))?;

    let mut de = Json5Deserializer::from_str(&input).map_err(|_| Error)?;
    let js_val = Value::deserialize(&mut de).map_err(|_| Error)?;
    let max_line_width = opt.line_width;

    opt.with_output(|out| {
        value::<_, ()>(&js_val, &Arena::new())
            .1
            .render(max_line_width, out)
            .map_err(|e| Error)
    })?;

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    match run(opt) {
        Ok(_) => {}
        Err(_) => eprintln!("Failed"),
    }
}
