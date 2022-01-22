use crate::eeyore::inst::*;
use crate::eeyore::eeyore;

pub fn Parse(parseString: &String)->Vec<Inst>{
    let r = eeyore::IIParser::new();
    parseString.lines().filter_map(|l| r.parse(l).ok()).collect()
}

pub fn Print(a:&Vec<Inst>){
    a.iter().for_each(|x| println!("{}",x));
}