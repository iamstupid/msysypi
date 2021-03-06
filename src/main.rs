#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
use std::{io, fs};
mod sysY;
mod eeyore;
mod tigger;

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
    }else if c.tig {
        // not impl
    }else{
        let q = c.infile.clone();
        if q.ends_with(".c"){
            // c => riscv
            let inFile = fs::read_to_string(c.infile).expect("Fuck.");
            let r = sysY::sysY::ProgramParser::new().parse(&inFile);
            match r {
                Ok(t) => {
                    let t = sysY::compile::compile(t);
                    let z = t.ins.into_iter().collect::<Vec<eeyore::inst::Inst>>();
                    fs::write(c.oufile,eeyore::trans::YAss(z));
                },
                Err(v) => println!("{}",v)
            }
        }else {
            if q.ends_with(".eeyore"){
                // eey => riscv
                let inFile = fs::read_to_string(c.infile).expect("Fuck.");
                let z = eeyore::parser::Parse(&inFile);
                fs::write(c.oufile, eeyore::trans::YAss(z));
            }else{
                // tig => riscv
                let inFile = fs::read_to_string(c.infile).expect("Fuck.");
                fs::write(c.oufile, tigger::inst::Prog::ass(inFile));
            }
        }
    }
}
