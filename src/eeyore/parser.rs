use crate::eeyore::inst::*;
use crate::eeyore::eeyore;

pub fn Parse(parseString: &String)->Vec<Inst>{
    parseString.lines().filter_map(|l| eeyore::IIParser::new().parse(l).ok()).collect()
}

pub fn Print(a:&Vec<Inst>){
    a.iter().for_each(|x| println!("{}",x));
}