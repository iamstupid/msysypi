use derive_more::{Display};
use core::fmt;
use std::{collections::HashMap};
use std::sync::Mutex;
use crate::eeyore::inst::{Oper,UOper};
use super::tig;

fn is_int12(i:i32) -> bool{
    i>=-2048 && i<2048
}
fn is_int10(i:i32) -> bool{
    i>=-512 && i<512
}

#[derive(Display,Copy,Clone,PartialEq)]
pub enum VarD{
    #[display(fmt="v{} = {}\n",_0,_1)]
    I(i32,i32),
    #[display(fmt="v{} = malloc {}\n",_0,_1)]
    A(i32,i32)
}

pub fn treg(a:&str)->u8{
    let c = u8::from_str_radix(&a[1..],10).unwrap();
    let o = match a.chars().nth(0).unwrap(){
        'x' => 0,
        's' => 1,
        't' => 13,
        'a' => 20,
        _ => panic!("not gonna happen")
    };
    c+o
}
pub fn freg(a:u8)->(char,u8){
    if a < 1 { return ('x',0) }
    if a < 13{ return ('s',a-1) }
    if a < 20{ return ('t',a-13) }
    ('a',a-20)
}
pub fn sreg(a:u8)->String{
    let (a,b) = freg(a);
    format!("{}{}",a,b)
}

#[derive(Clone)]
pub struct Glo{
    fnMap: Vec<String>,
    rnMap: HashMap<String,i32>
}
impl Glo{
    fn new() -> Glo {
        Glo{fnMap:vec![], rnMap: HashMap::new() }
    }
    fn fnd(&mut self, name: &str) -> i32{
        let c = self.rnMap.get(name);
        match c{
            None => {
                let id = self.fnMap.len() as i32;
                self.fnMap.push(name.to_string());
                self.rnMap.insert(name.to_string(),id);
                id
            },
            Some(t) => *t
        }
    }
}

lazy_static!(
    static ref fglo: Mutex<Glo> =  Mutex::new(Glo::new());
);

#[derive(Clone)]
struct Fn{
    name:i32,
    par:i32,
    sta:i32,
    inst:Vec<Inst>
}

#[derive(Clone)]
pub struct Prog{
    vdef: Vec<VarD>,
    fdef: Vec<Fn>
}

pub fn fnm(a:&str) -> i32{
    let mut z = fglo.lock().unwrap();
    z.fnd(a)
}
pub fn sfn(a:i32) -> String{
    let mut z = fglo.lock().unwrap();
    z.fnMap.get(a as usize).unwrap().clone()
}

pub enum FD{
    F(i32,i32,i32),
    D(VarD)
}

impl Prog{
    pub fn parse(a:String) -> Prog{
        let mut vdef = vec![];
        let mut fdef = vec![];
        let mut idef = vec![];
        let mut inf : Option<(i32,i32,i32)> = None;
        let fdp = tig::fdParser::new();
        let inp = tig::insParser::new();
        for i in a.lines(){
            let c = i.split("//").next().unwrap().trim().to_string();
            if c.len() == 0 { continue; }
            match inf{
                None => {
                    match fdp.parse(c.as_str()){
                        Err(t) => println!("{}",t),
                        Ok(FD::F(i,j,k)) => {inf = Some((i,j,k));},
                        Ok(FD::D(d)) => {vdef.push(d);}
                    }
                },
                Some((n,i,j)) => {
                    match inp.parse(c.as_str()){
                        Err(t) => println!("{}",t),
                        Ok(Inst::Edf) => {
                            fdef.push(Fn{name:n,par:i,sta:j,inst:idef});
                            idef = vec![];
                            inf = None;
                        },
                        Ok(i) => idef.push(i)
                    }
                }
            }
        }
        Prog{vdef,fdef}
    }
}

#[derive(Copy,Clone,PartialEq)]
pub enum Inst{
    Op(u8,u8,Oper,u8),
    Oi(u8,u8,Oper,i32),
    Ou(u8,UOper,u8),
    Ts(u8,u8),
    Li(u8,i32),
    St(u8,i32,u8),
    Ld(u8,u8,i32),
    Cj(u8,Oper,u8,i32),
    Jm(i32),
    Lb(i32),
    Cl(i32),
    Rt,
    Sst(u8,i32),
    Sld(i32,u8),
    Vld(i32,u8),
    Sla(i32,u8),
    Vla(i32,u8),
    Edf
}
fn popr(a:u8, u:u8, o:Oper, v:u8) -> String{
    use Oper::*;
    let (ao,b) = match o{
        Add => ("add",0),
        Sub => ("sub",0),
        Mul => ("mul",0),
        Div => ("div",0),
        Mod => ("rem",0),
        Lt => ("slt",0),
        Gt => ("sgt",0),
        Le => ("sgt",1),
        Ge => ("slt",1),
        And => ("",2),
        Or => ("",2),
        Eq => ("xor",1),
        Ne => ("",2)
    };
    if b == 0 { format!("{} {},{},{}\n",ao,sreg(a),sreg(u),sreg(v)) }
    else if b ==1 {
        format!("{} {},{},{}\nseqz {},{}\n",ao,sreg(a),sreg(u),sreg(v),sreg(a),sreg(a))
    }else {match o{
        And => format!(
"snez {},{}
snez s0,{}
and {},{},s0\n",sreg(a),sreg(u),sreg(v),sreg(a),sreg(a)),
        Or => format!("or {},{},{}\nsnez {},{}\n",sreg(a),sreg(u),sreg(v),sreg(a),sreg(a)),
        Ne => format!("xor {},{},{}\nsnez {},{}\n",sreg(a),sreg(u),sreg(v),sreg(a),sreg(a)),
        _ => panic!("impossible branch")
    }}
}
fn sw<A:fmt::Display, B:fmt::Display>(a:A,c:i32,d:B) -> String{
    if is_int12(c) { format!("sw {},{}({})\n",d,c,a) } else {
        let t = format!("li s0,{}\nadd s0,s0,{}\n",c,a);
        format!("{}sw {},0(s0)\n",t,d)
    }
}
fn lw<A:fmt::Display, B:fmt::Display>(a:A,c:B,d:i32) -> String{
    if is_int12(d) { format!("lw {},{}({})\n",a,d,c) } else {
        let t = format!("li s0,{}\nadd s0,s0,{}\n",d,c);
        format!("{}lw {},0(s0)\n",t,a)
    }
}
impl Inst{
    fn ts(&self) -> String {
        use Inst::*;
        match *self{
            Op(a,b,c,d) => format!("{} = {} {} {}",sreg(a),sreg(b),c,sreg(d)),
            Oi(a,b,c,d) => format!("{} = {} {} {}",sreg(a),sreg(b),c,d),
            Ou(a,c,d) => format!("{} = {} {}",sreg(a),c,sreg(d)),
            Ts(a,b) => format!("{} = {}",sreg(a),sreg(b)),
            Li(a,b) => format!("{} = {}",sreg(a),b),
            St(a,c,d) => format!("{}[{}] =  {}",sreg(a),c,sreg(d)),
            Ld(a,c,d) => format!("{} =  {}[{}]",sreg(a),sreg(c),d),
            Cj(a,b,c,d) => format!("if {} {} {} goto l{}",sreg(a),b,sreg(c),d),
            Jm(a) => format!("goto l{}",a),
            Lb(a) => format!("l{}:",a),
            Cl(a) => format!("call {}",sfn(a)),
            Rt => "return".to_string(),
            Sst(a,b) => format!("store {} {}",sreg(a),b),
            Sld(a,b) => format!("load {} {}",a,sreg(b)),
            Vld(a,b) => format!("load v{} {}",a,sreg(b)),
            Sla(a,b) => format!("loadaddr {} {}",a,sreg(b)),
            Vla(a,b) => format!("loadaddr v{} {}",a,sreg(b)),
            Edf => "".to_string()
        }
    }
    fn tr(&self, stk:i32) -> String {
        use Inst::*;
        use Oper::*;
        match *self{
            Op(a,b,c,d) => popr(a,b,c,d),
            Oi(a,b,c,d) => {
                if is_int12(d) {
                    match c{
                        Add => format!("addi {},{},{}\n",sreg(a),sreg(b),d),
                        Lt => format!("slti {},{},{}\n",sreg(a),sreg(b),d),
                        _ => format!("li s0,{}\n{}",d,popr(a,b,c,1))
                    }
                }else{
                    format!("li s0,{}\n{}",d,popr(a,b,c,1))
                }
            },
            Ou(a,c,d) => match c{
                UOper::Neg => format!("neg {},{}\n",sreg(a),sreg(d)),
                UOper::Not => format!("seqz {},{}\n",sreg(a),sreg(d))
            },
            Ts(a,b) => format!("mv {},{}\n",sreg(a),sreg(b)),
            Li(a,b) => format!("li {},{}\n",sreg(a),b),
            St(a,c,d) => sw(sreg(a), c, sreg(d)),
            Ld(a,c,d) => lw(sreg(a),sreg(c),d),
            Cj(a,b,c,d) => {
                let r = match b{ Lt => "blt", Gt => "bgt", Le => "ble", Ge => "bge", Ne => "bne", Eq => "beq", _ => panic!("NTR")};
                format!("{} {},{},.l{}\n",r,sreg(a),sreg(c),d)
            },
            Jm(a) => format!("j .l{}\n",a),
            Lb(a) => format!(".l{}:\n",a),
            Cl(a) => format!("call {}\n",sfn(a)),
            Rt => if is_int12(stk) {
format!("lw ra, {stk2}(sp)
addi sp, sp, {stk1}
ret\n",stk1=stk,stk2=stk-4)}else{
format!("
li s0,{stk}
add sp,sp,s0
lw ra, -4(sp)
ret
",stk=stk)
},
            Sst(a,b) => sw("sp",b*4,sreg(a)),
            Sld(a,b) => lw(sreg(b),"sp",a*4),
            Vld(a,b) => format!("lui {},%hi(v{})\nlw {},%lo(v{})({})\n",sreg(b),a,sreg(b),a,sreg(b)),
            Sla(a,b) => {
                if is_int10(a){
                    format!("addi {},sp,{}\n",sreg(b),a*4)
                }else{
                    format!(
"li s0,{}
add {},s0,sp\n",a,sreg(b)
                    )
                }
            },
            Vla(a,b) => format!("la {},v{}\n",sreg(b),a),
            Edf => "".to_string()
        }
    }
}
impl Prog{
    pub fn tr(&self) -> String{
        let decs = self.vdef.iter().map(|x| x.tr()).collect::<Vec<String>>();
        let decs = decs.concat();
        let fns = self.fdef.iter().map(|x| x.tr()).collect::<Vec<String>>();
        decs + &fns.concat()
    }
}
impl VarD{
    fn tr(&self) -> String{
        use VarD::*;
        match *self{
            I(a,b) => format!(
"   .global v{}
    .section .sdata
    .align 2
    .type v{},@object
    .size v{},4
v{}:
    .word {}\n",a,a,a,a,b)
        ,
        A(a,b) => format!(
"   .comm v{}, {}, 4\n",a,b)
        }
    }
}
impl Fn{
    fn tr(&self) -> String{
        let func = sfn(self.name);
        let stk = (self.sta/4+1)*16;
        let code = self.inst.iter().map(|x| x.tr(stk)).collect::<Vec<String>>().concat();
        if is_int12(stk){
        format!(
"   .text
    .align 2
    .global {func}
    .type {func}, @function
{func}:
    addi sp, sp, {stk1}
    sw ra,{stk2}(sp)
{code}
    .size {func},.-{func}\n",func=func,code=code,stk1=-stk,stk2=stk-4)
}else{
    format!(
"   .text
.align 2
.global {func}
.type {func}, @function
{func}:
sw ra,-4(sp)
li s0, {stk1}
add sp, sp, s0
{code}
.size {func},.-{func}\n",func=func,code=code,stk1=-stk)
}
}
}