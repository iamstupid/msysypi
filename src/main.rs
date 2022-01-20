#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate clap;
use std::{io, fs};
mod sysY;
mod eeyore;

use crate::clap::Parser;


#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Arg{
    #[clap(short = 'S')]
    ass: bool,
    #[clap(short = 'e')]
    eey: bool,
    #[clap(short = 't')]
    tig: bool,
    infile: String,
    #[clap(short = 'o')]
    oufile: String
}

fn main() {
    let c = Arg::parse();
    if c.eey {
        let inFile = fs::read_to_string(c.infile).expect("Fuck.");
        let r = sysY::sysY::ProgramParser::new().parse(&inFile);
        match r {
            Ok(t) => {fs::write(c.oufile,sysY::compile::compile(t).print());},
            Err(v) => println!("{}",v)
        }
    }
}
