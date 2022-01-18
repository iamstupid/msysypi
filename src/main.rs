#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate clap;
use std::{io, fs};
mod sysY;
mod eeyore;

use crate::clap::Parser;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    sysY,
    eeyore
}

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
pub struct Arg{
    #[clap(short,long)]
    /// filename to process
    filename : String,
    #[clap(short,long,arg_enum,default_value_t = Mode::sysY)]
    mode : Mode
}

fn main() {
    let arg = Arg::parse();
    let inFile = fs::read_to_string(arg.filename).expect("Oh shit sherlock!");
    // println!("{}",inFile.as_str());
    match arg.mode{
        Mode::sysY =>{
            let z = sysY::sysY::ProgramParser::new().parse(inFile.as_str());
            match z{
                Err(t) => println!("{}",t),
                Ok(p) => {
                    let mut c = sysY::eval::VScope::new();
                    let t = sysY::eval::vprep(p,&mut c);
                    println!("{}",sysY::ast::pvec(&t,"\n","",""))
                }
            }
        },
        Mode::eeyore => {
            let z = eeyore::parser::Parse(&inFile);
            eeyore::parser::Print(&z);
        }
    }
}
