use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use crate::eeyore::inst::{LVal, RVal, FnName, VarUsage};
use crate::eeyore::{parser,inst};
use crate::tigger::inst as ti;
use ti::Inst as Ti;
use inst::Inst as Yi;
use inst::Var as Yv;
use inst::Oper;
use ti::{fnm,sfn,Fn};
// Reg names
#[derive(Copy,Clone)]
enum Reg{x0,s0,s1,s2,s3,s4,s5,s6,s7,s8,s9,s10,s11,t0,t1,t2,t3,t4,t5,t6,a0,a1,a2,a3,a4,a5,a6,a7}
static reg:[Reg;28] = [x0,s0,s1,s2,s3,s4,s5,s6,s7,s8,s9,s10,s11,t0,t1,t2,t3,t4,t5,t6,a0,a1,a2,a3,a4,a5,a6,a7];
impl Reg{
    fn tu(&self) -> u8{
        match *self{ x0=>0,s0=>1,s1=>2,s2=>3,s3=>4,s4=>5,s5=>6,s6=>7,s7=>8,s8=>9,s9=>10,s10=>11,s11=>12,t0=>13,t1=>14,t2=>15,t3=>16,t4=>17,t5=>18,t6=>19,a0=>20,a1=>21,a2=>22,a3=>23,a4=>24,a5=>25,a6=>26,a7=>27 }
    }
}
use Reg::*;
// RISC-V initializer
fn comm(n:i32, s:usize) -> String{
    format!("    .comm v{},{},4\n",n,s
    )
}
fn sdata(n:i32, s:&Vec<i32>) -> String{
    format!("    .global v{name}
    .section .sdata
    .align  2
    .type   v{name}, @object
    .size   v{name}, {size}
v{name}:
{init}
",name=n, size=s.len()*4, init=s.iter().map(|x| format!("    .word   {}
",*x)).collect::<Vec<String>>().concat())
}
// global definition handler
struct Vdc{
    name: i32,
    size: usize, // 0 for single var
    cont: Vec<i32>
}
impl Vdc{
    fn new(name:i32,size:i32) -> Self{ Vdc{name,size:(size as usize),cont:vec![]} }
    fn tr(&self) -> String{
        let n = self.name;
        if self.size == 0 {
            // single variable
            if self.cont.len() == 0{
                // uninitialized
                sdata(n,&vec![0])
            }else{
                sdata(n,&self.cont)
            }
        }else{
            // array
            if self.cont.len() == 0{
                comm(n,self.size)
            }else{
                sdata(n,&self.cont)
            }
        }
    }
    fn ass(&mut self, ind: usize, cnt: i32){
        if self.size == 0 {
            self.cont = vec![cnt];
        }else{
            if self.cont.len() == 0 {
                self.cont = vec![0; self.size>>2]
            }
            self.cont[ind]=cnt;
        }
    }
}
struct Gdef{
    hm: HashMap<Yv, RefCell<Vdc>>,
    al: i32
}
impl Gdef{
    fn new() -> Self{ Gdef{hm:HashMap::new(), al:0}}
    fn reg(&mut self,a:Yv,b:usize){
        let id = self.al;
        self.al+=1;
        self.hm.insert(a, RefCell::new(Vdc::new(id,b as i32)));
    }
    fn ass(&self,a:Yv,b:i32,cn:i32){
        let b = (b >> 2) as usize;
        let c = self.hm.get(&a).unwrap();
        let mut c = c.borrow_mut();
        c.ass(b,cn);
    }
    fn tr(&self)->String{
        self.hm.iter().map(|(k,v)| v.borrow().tr()).collect::<Vec<String>>().concat()
    }
    fn fnd(&self,a:Yv)->(i32,i32){
        let t = self.hm.get(&a).unwrap();
        let c =t.borrow();
        (c.name, c.size as i32)
    }
}
struct Ldef{
    hm: HashMap<Yv, (i32,i32)>,
    st: i32
}
impl Ldef{
    fn new() -> Self{ Ldef{hm:HashMap::new(), st:0}}
    fn reg(&mut self,a:Yv,b:i32)->i32{
        let id = self.st;
        self.st += b;
        if b==0 {self.st+=4;}
        self.hm.insert(a, (id,b));
        id
    }
    fn fnd(&self,a:Yv)->Option<(i32,i32)>{
        if let Some((i,j)) = self.hm.get(&a){
            Some((*i,*j))
        }else {None}
    }
}
// Eeyore -> RISC-V assembler
pub fn YAss(a:Vec<Yi>) -> String{
    use Yi::*;
    use Ti::*;
    use Oper::*;
    use LVal::Sym as LS;
    use RVal::Sym as RS;
    let mut fns = vec![];
    let mut gv = Gdef::new();
    let mut nfn = ti::Fn::new();
    let mut ifn = false;
    let mut lv = Ldef::new();
    // helper lambdas
    let load = |lv:&Ldef, gv:&Gdef, nfn:&mut ti::Fn, a:RVal, b:Reg| {
        // load a:RVal into b
        let b = b.tu();
        match a{
            RVal::Int(t) => nfn.ps(Li(b,t)),
            RVal::Sym(v) => {
                let c = lv.fnd(v);
                if let Some((i,j)) = c{
                    if j==0 {
                        // load local int
                        nfn.ps(Sld(i>>2,b))
                    }else{
                        // load local array address
                        nfn.ps(Sla(i>>2,b))
                    }
                }else{
                    let (i,j) = gv.fnd(v);
                    if j==0{
                        // load global int
                        nfn.ps(Vld(i,b))
                    }else{
                        // load global array address
                        nfn.ps(Vla(i,b))
                    }
                }
            }
        }
    };
    let store = |lv:&Ldef, gv:&Gdef, nfn:&mut ti::Fn, a:Reg, b:LVal|{
        // store a:Reg into b:LVal
        // note: extra register needed: s4, s5
        let a = a.tu();
        let S4 = s4.tu();
        let S5 = s5.tu();
        let (bvname,bind,r) = match b { LVal::Sym(t) => (t,RVal::Int(0),false), LVal::SymA(t,a)=> (t,a,true)};
        let c = lv.fnd(bvname);
        if let Some((i,j)) = c{
            if !r{
                // local int
                nfn.ps(Sst(a,i>>2));
            }else{
                // local array
                match bind{
                    RVal::Int(t) =>{
                        if j == 0{
                            nfn.ps(Sld(i>>2,S4));
                            nfn.ps(St(S4,t,a));
                        }else{
                            // addressed with int
                            nfn.ps(Sst(a,(i+t)>>2));
                        }
                    },
                    RVal::Sym(t) =>{
                        if j == 0{
                            load(lv,gv,nfn,RVal::Sym(t),s5);
                            nfn.ps(Sld(i>>2,S4));
                            nfn.ps(Op(S4,S4,Add,S5));
                            nfn.ps(St(S5,0,a));
                        } else {
                            // addressed with sym
                            load(lv,gv,nfn,RVal::Sym(t),s4);
                            nfn.ps(Spa(S4,S4));
                            nfn.ps(St(S4,i,a));
                        }
                    }
                }
            }
        }else{
            let (i,j) = gv.fnd(bvname);
            if !r{
                // global int
                nfn.ps(Vla(i,S4));
                nfn.ps(St(S4,0,a));
            }else{
                // global array
                match bind{
                    RVal::Int(t) =>{
                        // addressed with int
                        if j == 0 {nfn.ps(Vld(i,S4));} else {nfn.ps(Vla(i,S4));}
                        nfn.ps(St(S4,t,a));
                    },
                    RVal::Sym(t) =>{
                        // addressed with sym
                        if j == 0 {nfn.ps(Vld(i,S4));} else {nfn.ps(Vla(i,S4));}
                        load(lv,gv,nfn,RVal::Sym(t),s5);
                        nfn.ps(Op(S4,S4,Add,S5));
                        nfn.ps(St(S4,0,a));
                    }
                }
            }
        }
    };
    macro_rules! load{
        ($a:expr, $b:expr) => {load(&lv,&gv,&mut nfn,$a,$b)}
    }
    macro_rules! store{
        ($a:expr, $b:expr) => {store(&lv,&gv,&mut nfn,$a,$b)}
    }
    let S1= s1.tu();
    let S2= s2.tu();
    let mut fnpar:usize = 0;
    for i in a.into_iter(){
        if ifn {
            match i{
                DArr(u, j) => {lv.reg(j,u);},
                DI(u) => {lv.reg(u,0);},
                Ass(l,r) => { load!(r,s1); store!(s1,l); },
                Take(a,b,r) => {
                    load!(r,s1);
                    load!(RS(b),s2);
                    nfn.ps(Op(S1,S1,Add,S2));
                    nfn.ps(Ld(S1,S1,0));
                    store!(s1,LS(a));
                },
                Cpt(r,a,o,b) => {
                    load!(a,s1);
                    load!(b,s2);
                    nfn.ps(Op(S1,S1,o,S2));
                    store!(s1,LS(r));
                },
                UCpt(r, o, a) => {
                    load!(a,s1);
                    nfn.ps(Ou(S1,o,S1));
                    store!(s1,LS(r));
                },
                Brch(l,o,r,lb) => {
                    load!(l,s1);
                    load!(r,s2);
                    nfn.ps(Cj(S1,o,S2,lb));
                },
                Jump(l) => nfn.ps(Jm(l)),
                Label(l) => nfn.ps(Lb(l)),
                Param(r) => {
                    load!(r,reg[20+fnpar]);
                    fnpar+=1;
                },
                Call(FnName(ref s)) => {
                    fnpar=0;
                    nfn.ps(Cl(fnm(s)));
                },
                CallFn(a,FnName(ref s)) => {
                    fnpar=0;
                    nfn.ps(Cl(fnm(s)));
                    store!(a0,LS(a));
                },
                Return(v) => {
                    load!(v,a0);
                    nfn.ps(Rt);
                },
                Ret => nfn.ps(Rt),
                FnE(_) => {
                    nfn.sta = lv.st;
                    fns.push(nfn);
                    nfn = ti::Fn::new();
                    ifn = false;
                    lv = Ldef::new();
                },
                _ => panic!("Should not appear inside function domain")
            }
        }else{
            // outside fn
            match i{
                DArr(u, j) => gv.reg(j,u as usize),
                DI(u) => gv.reg(u,0),
                Ass(LVal::Sym(v),RVal::Int(t)) => gv.ass(v, 0, t),
                Ass(LVal::SymA(v,RVal::Int(i)),RVal::Int(j)) => gv.ass(v,i,j),
                Fn(FnName(ref s),i) => {
                    ifn = true;
                    nfn.name = fnm(s);
                    nfn.par=i;
                    for i in (0..i){
                        let cpos = lv.reg(Yv(VarUsage::Param,i), 0);
                        nfn.inst.push(Sst((20+i) as u8,cpos>>2));
                    }
                },
                _ => panic!("Those should not appear outside function domain")
            }
        }
    }
    format!("{}\n{}",gv.tr(),fns.iter().map(|x|{
        x.tr()
    }).collect::<Vec<String>>().concat())
}