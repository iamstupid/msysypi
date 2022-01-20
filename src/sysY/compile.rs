use std::{collections::{HashMap, VecDeque}};

use crate::eeyore::inst::*;
use crate::sysY::eval::{dicast,Prep};
use crate::sysY::eval;

use super::{ast::{CompUnit, Decl, VarDecl, InitCont, VarInit, Expr, Exp, Func, BlockItem, Stmt, CondExpr}, eval::bmult};
use super::ast;
#[derive(Clone)]
struct Vmeta{
    dim:Vec<i32>,
    vdef:Var
}
impl Vmeta{
    fn dump(&self) -> Option<Inst>{
        if let Var(VarUsage::Param,t) = self.vdef{ return None; }
        Some(
            if self.dim.len() == 1 {
                Inst::DI(self.vdef)
            }else{
                Inst::DArr(self.dim[0]*4,self.vdef)
            }
        )
    }
}
struct VScope{
    map:HashMap<String,Vmeta>,
    local:i32,
    temp:i32,
    param:i32
}
struct VStack{
    st:Vec<VScope>,
    lab:i32
}
impl VScope{
    fn new(fcnt:i32)->VScope{
        VScope{map:HashMap::new(),local:fcnt,temp:0,param:0}
    }
    fn reg_loc(&mut self,n:&str,d:&Vec<i32>)->Var{
        let id = self.local;
        self.local += 1;
        let ret = Var(VarUsage::Var, id);
        let meta = Vmeta{dim:d.clone(),vdef:ret};
        self.map.insert(n.to_string(),meta);
        ret
    }
    fn reg_par(&mut self,n:&str,d:&Vec<i32>)->Var{
        let id = self.param;
        self.param += 1;
        let ret = Var(VarUsage::Param, id);
        let meta = Vmeta{dim:d.clone(),vdef:ret};
        self.map.insert(n.to_string(),meta);
        ret
    }
    fn reg_tmp(&mut self)->Var{
        let id = self.temp;
        self.temp += 1;
        let ret = Var(VarUsage::Temp, id);
        ret
    }
    fn dump(&self) -> VI{
        let loc = VecDeque::from_iter(self.map.iter().filter_map(|(k,v)| v.dump()));
        let tmp = VecDeque::from_iter((0..self.temp).map(|x| Inst::DI(Var(VarUsage::Temp,x))));
        mdq(loc,tmp)
    }
}
impl VStack{
    fn new()->VStack{
        VStack{st:vec![VScope::new(0)],lab:0}
    }
    fn get_lab(&mut self)->i32{
        let r=self.lab;
        self.lab+=1;
        r
    }
    fn enter(&mut self){
        let r = self.st.iter().rev().next().unwrap().local;
        self.st.push(VScope::new(r));
    }
    fn exit(&mut self)->VScope{
        self.st.pop().unwrap()
    }
    fn find(&self, name:&String)->Option<Vmeta>{
        self.st.iter().rev().find_map(|t| t.map.get(name)).map(|f| f.clone())
    }
    fn top(&mut self)->&mut VScope{ self.st.iter_mut().rev().next().unwrap() }
}
pub struct Segment{
    ins: VecDeque<Inst>,
    list: Vec<Vec<i32>>,
    ret: Option<RVal>
}
fn mls(mut a:Vec<i32>,b:Vec<i32>,c:i32)->Vec<i32>{
    b.iter().for_each(|t| a.push(t+c));
    a
}
fn mdq(mut a:VecDeque<Inst>, mut b:VecDeque<Inst>)->VecDeque<Inst>{
    if a.len() < b.len(){
        a.iter().rev().for_each(|f| b.push_front(f.clone()));
        b
    }else{
        b.iter().for_each(|f| a.push_back(f.clone()));
        a
    }
}
impl Segment{
    fn new() -> Segment{
        Segment{ins:VecDeque::new(),list:vec![vec![],vec![],vec![],vec![]],ret:None}
    }
    fn from(a:VI) -> Segment{
        Segment{ins:a,list:vec![vec![],vec![],vec![],vec![]],ret:None}
    }
    fn push(&mut self,a:Inst){ self.ins.push_back(a); }
    fn cat(a:Segment, b:Segment) -> Segment{
        let c = a.ins.len() as i32;
        Segment {
            ins: mdq(a.ins,b.ins),
            list: a.list.into_iter().zip(b.list.into_iter()).map(|(a,b)| mls(a,b,c)).collect(),
            ret:b.ret
        }
    }
    fn pre(self) -> Segment{
        let list = self.list.into_iter().map(|x| x.iter().map(|z| z+1).collect()).collect();
        Segment{list,..self}
    }
    fn fill(mut self,i:usize,lab:i32) -> Segment{
        for i in &self.list[i]{
            let i = *i as usize;
            self.ins[i] = match &self.ins[i]{
                //          还好底下这几个都是Copy
                //          所以rust真的只能和Copy玩是吧(😓)
                &Inst::Brch(a,b,c,_) => Inst::Brch(a,b,c,lab),
                &Inst::Jump(_) => Inst::Jump(lab),
                t => t.clone()
            }
        }
        self.list[i] = vec![];
        self
    }
    fn lab(mut self,b:&mut VStack) -> (Segment,i32){
        let label = b.get_lab();
        let lab = Segment::from(VI::from([Inst::Label(label)]));
        (Segment::cat(lab,self),label)
    }
    fn plab(mut self,a:i32) -> Segment{
        self.ins.push_back(Inst::Label(a));
        self
    }
    fn jump(mut self,lab:i32)->Segment{
        self.ins.push_back(Inst::Jump(lab));
        self
    }
    fn jumpx(mut self,c:Inst,x:i32)->Segment{
        self.list[x as usize].push(self.ins.len() as i32);
        self.ins.push_back(c);
        self
    }
}
trait Comp{
    fn cc(&self,b:&mut VStack) -> Segment;
}
impl Comp for CompUnit{
    fn cc(&self,b:&mut VStack) -> Segment{
        match self{
            &CompUnit::Decl(ref a) => a.cc(b),
            &CompUnit::Func(ref a) => a.cc(b)
        }
    }
}
fn vcc<T:Comp>(a:&Vec<Box<T>>,b:&mut VStack) -> Segment{
    a.iter().fold(Segment::new(),|a,x| Segment::cat(a,x.cc(b)))
}
impl Comp for Decl{
    fn cc(&self,b:&mut VStack) -> Segment{ vcc(&self.1,b) }
}
impl Comp for VarDecl{
    fn cc(&self,b:&mut VStack) -> Segment{
        let dims = dicast(&self.dims);
        let dims = bmult(&dims);
        let var = b.top().reg_loc(self.name.as_str(), &dims);
        rci(var,dims,&self.init, b)
    }
}
impl VarDecl{
    fn rg(&self,b:&mut VStack){
        let dims = dicast(&self.dims);
        let dims = bmult(&dims);
        b.top().reg_par(self.name.as_str(), &dims);
    }
}
struct AR<T>(T);
impl VCpl for AR<i32>{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI{
        VI::from([Inst::Ass(a,RVal::Int(self.0))])
    }
}
impl VCpl for AR<ast::Exp>{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI{
        self.0.cz(a,b)
    }
}
impl VCpl for AR<&ast::Exp>{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI{
        self.0.cz(a,b)
    }
}
fn rci(a:Var,dim:Vec<i32>,init:&VarInit,b:&mut VStack)->Segment{
    match init{
        &VarInit::Nil => Segment::new(),
        &VarInit::E(ref t) => Segment::from(t.rcz(a,0,dim.as_slice(),b)),
        &VarInit::I(ref t) => Segment::from(t.rcz(a,0,dim.as_slice(),b))
    }
}
impl InitCont<i32>{
    fn rcz(&self, a: Var, off: i32, dim:&[i32], b:&mut VStack)->VI{
        if dim.len() == 1{
            match self{
                InitCont::Val(t) => AR(*t).cz(LVal::Sym(a), b),
                InitCont::Vax(t) => panic!("dimension mismatch")
            }
        }else{self.cz(a,off,dim,b)}
    }
    fn cz(&self, a: Var, off: i32, dim:&[i32], b:&mut VStack)->VI{
        if dim.len() == 1{
            // one-dimension left that's a single value
            match self{
                InitCont::Val(t) => AR(*t).cz(LVal::SymA(a,RVal::Int(off)), b),
                InitCont::Vax(t) => panic!("dimension mismatch")
            }
        }else{
            match self{
                InitCont::Val(t) => AR(*t).cz(LVal::SymA(a,RVal::Int(off)), b),
                InitCont::Vax(t) => {
                    let da = dim[0];
                    let db = dim[1];
                    let mut ptr = 0;
                    let mut vx=VI::new();
                    let ps = |t:i32| if t%db==0 {t} else {(t+db)%db};
                    for i in t.iter(){
                        match i.as_ref(){
                            &InitCont::Val(_) => {
                                vx=mdq(vx,i.cz(a,off+ptr*4,&dim[1..],b));
                                ptr+=1;
                            },
                            &InitCont::Vax(_) => {
                                ptr = ps(ptr);
                                vx=mdq(vx,i.cz(a,off+ptr*4,&dim[1..],b));
                                ptr += db;
                            }
                        }
                    }
                    vx
                }
            }
        }
    }
}
impl InitCont<Exp>{
    fn rcz(&self, a: Var, off: i32, dim:&[i32], b:&mut VStack)->VI{
        if dim.len() == 1{
            match self{
                InitCont::Val(t) => AR(t).cz(LVal::Sym(a), b),
                InitCont::Vax(t) => panic!("dimension mismatch")
            }
        }else{self.cz(a,off,dim,b)}
    }
    fn cz(&self, a: Var, off: i32, dim:&[i32], b:&mut VStack)->VI{
        if dim.len() == 1{
            // one-dimension left that's a single value
            match self{
                InitCont::Val(t) => AR(t).cz(LVal::SymA(a,RVal::Int(off)), b),
                InitCont::Vax(t) => panic!("dimension mismatch")
            }
        }else{
            match self{
                InitCont::Val(t) => AR(t).cz(LVal::SymA(a,RVal::Int(off)), b),
                InitCont::Vax(t) => {
                    let da = dim[0];
                    let db = dim[1];
                    let mut ptr = 0;
                    let mut vx=VI::new();
                    let ps = |t:i32| if t%db==0 {t} else {(t+db)%db};
                    for i in t.iter(){
                        match i.as_ref(){
                            &InitCont::Val(_) => {
                                vx=mdq(vx,i.cz(a,off+ptr*4,&dim[1..],b));
                                ptr+=1;
                            },
                            &InitCont::Vax(_) => {
                                ptr = ps(ptr);
                                vx=mdq(vx,i.cz(a,off+ptr*4,&dim[1..],b));
                                ptr += db;
                            }
                        }
                    }
                    vx
                }
            }
        }
    }
}


type VI = VecDeque<Inst>;
trait VCpl{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI;
}
impl VCpl for i32{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI{
        VI::from([Inst::Ass(a,RVal::Int(*self))])
    }
}
impl ast::Oper{
    fn to_ee(&self) -> Oper {
        match self{
            &ast::Oper::Add => Oper::Add,
            &ast::Oper::Sub => Oper::Sub,
            &ast::Oper::Mul => Oper::Mul,
            &ast::Oper::Div => Oper::Div,
            &ast::Oper::Mod => Oper::Mod,
            &ast::Oper::Lt => Oper::Lt,
            &ast::Oper::Le => Oper::Le,
            &ast::Oper::Eq => Oper::Eq,
            &ast::Oper::Ne => Oper::Ne,
            &ast::Oper::Ge => Oper::Ge,
            &ast::Oper::Gt => Oper::Gt
        }
    }
    fn is_co(&self) -> bool {
        match self{
            &ast::Oper::Add => false,
            &ast::Oper::Sub => false,
            &ast::Oper::Mul => false,
            &ast::Oper::Div => false,
            &ast::Oper::Mod => false,
            &ast::Oper::Lt => true,
            &ast::Oper::Le => true,
            &ast::Oper::Eq => true,
            &ast::Oper::Ne => true,
            &ast::Oper::Ge => true,
            &ast::Oper::Gt => true
        }
    }
}
impl LVal{
    fn get_ref(&self, b:&mut VStack) -> Var{
        match self{
            &LVal::Sym(a) => a,
            &LVal::SymA(a,r) => b.top().reg_tmp()
        }
    }
    fn store(&self, a:Var, b:&mut VI){
        match self{
            &LVal::Sym(a) => {},
            t => b.push_back(Inst::Ass(*t,RVal::Sym(a)))
        }
    }
}
impl RVal{
    fn store(&self, a:Var, b:&mut VI){
        match self{
            &RVal::Int(t) => b.push_back(Inst::Ass(LVal::Sym(a),*self)),
            &RVal::Sym(t) => if t!=a {b.push_back(Inst::Ass(LVal::Sym(a),*self));}
        }
    }
}
impl Expr{
    // cv means compile self into VI while the return value is stored in RVal
    // if a storage variable is provided as Some(var) then the return value must be
    // RVal::Sym(var)
    fn cv(&self, a:Option<Var>, b:&mut VStack) -> (VI, RVal){
        if let None = a{
            match self{
                &Expr::Nil => return (VI::new(),RVal::Int(0)),
                &Expr::Num(t) => return (VI::new(),RVal::Int(t)),
                &Expr::LVal(ref a) => {
                    if a.ind.len() == 0 {
                        let me = b.find(&a.name);
                        if let Some(c) = me{
                            return (VI::new(),RVal::Sym(c.vdef))
                        }
                    }
                },
                _ => {}
            }
        }
        let av = a.unwrap_or_else(|| b.top().reg_tmp());
        let rav = RVal::Sym(av);
        match self{
            &Expr::Nil => (VI::new(),RVal::Sym(av)),
            &Expr::Num(t) => (VI::from([Inst::Ass(LVal::Sym(av),RVal::Int(t))]),RVal::Sym(av)),
            &Expr::FnCall(ref fnm,ref para) => {
                let mut para=para.iter().map(|x|{
                    let (mut z,r)=x.cv(None,b);
                    z.push_back(Inst::Param(r));
                    z
                }).reduce(mdq).unwrap_or_else(|| VI::new());
                para.push_back(Inst::CallFn(av,FnName(fnm.to_string())));
                (para,rav)
            },
            &Expr::LVal(ref c) => c.cv(Some(av),b),
            &Expr::Op(ref u,o,ref v) => {
                let (uu,su) = u.cv(None,b);
                let (vv,sv) = v.cv(None,b);
                let mut cs = mdq(uu,vv);
                cs.push_back(Inst::Cpt(av,su,o.to_ee(),sv));
                (cs,rav)
            },
            &Expr::UOp(o,ref v) =>{
                let (mut vv,sv) = v.cv(None,b);
                let z = match o{
                    ast::UOper::Pos => return (vv,sv),
                    ast::UOper::Neg => UOper::Neg,
                    ast::UOper::Not => UOper::Not
                };
                vv.push_back(Inst::UCpt(av,z,sv));
                (vv,rav)
            }
        }
    }
}
impl VCpl for Expr{
    fn cz(&self, a:LVal, b:&mut VStack) -> VI{
        match self{
            &Expr::Nil => VI::new(),
            &Expr::Num(t) => VI::from([Inst::Ass(a,RVal::Int(t))]),
            t => {
                let ax = a.get_ref(b);
                let (mut uu,su) = t.cv(Some(ax),b);
                a.store(ax, &mut uu);
                uu
            }
        }
    }
}
fn oper(q:&mut VI, v:Var, a:RVal, o:Oper, b:RVal){
    q.push_back(Inst::Cpt(v,a,o,b));
}
fn pass(q:&mut VI, a:Var, b:RVal){
    q.push_back(Inst::Ass(LVal::Sym(a),b));
}
fn cvind(a:Var,dim:&[i32],ind:&[ast::Exp],b:&mut VStack) -> (VI, LVal){
    if ind.len() == 0 {
        (VI::new(), LVal::SymA(a,RVal::Int(0)))
    }else{
        let (mut v, r) = cvind(a,&dim[1..], &ind[1..], b);
        let mult = dim[0]*4;
        let (ov,or) = ind[0].cv(None,b);
        if let LVal::SymA(ra,rb) = r{
            if RVal::Int(0) == rb{
                let c = b.top().reg_tmp();
                v=mdq(v,ov);
                oper(&mut v,c,or,Oper::Mul,RVal::Int(mult));
                (v,LVal::SymA(ra,RVal::Sym(c)))
            }else{
                let c = b.top().reg_tmp();
                let d = b.top().reg_tmp();
                v=mdq(v,ov);
                oper(&mut v,c,or,Oper::Mul,RVal::Int(mult));
                oper(&mut v,d,RVal::Sym(c),Oper::Add,rb);
                (v,LVal::SymA(ra,RVal::Sym(d)))
            }
        }else {panic!("Impossiblu");}
    }
}
fn cvi(a:Var, dim: &Vec<i32>, ind: &Vec<ast::Exp>, b: &mut VStack) -> (VI, LVal) {
    // if it is a int type, then return the symbol
    // else, return an array offset
    // for the caller, if |dim| = |ind|+1 then the index should be accessed with a[ind]
    // otherwise a + ind
    if dim.len() == 1 { return (VI::new(),LVal::Sym(a));}
    cvind(a,&dim[1..], ind.as_slice(), b)
}
impl ast::LVal{
    fn cl(&self, a:Option<Var>, b:&mut VStack) -> (VI, LVal, bool){
        // ast::LVal into inst::LVal cast
        // if a = None, if lval is partial evaluated(into pointer) then returns LVal(Sym,Offset),false
        // otherwise return LVal(Sym,Offset),true
        // if a = Some(var), then var = lval and returns (var,false)/(var,true) depend on whether it is
        // completely evaluated
        let meta = b.find(&self.name);
        if let Some(meta) = meta{
            /*
            let av = a.unwrap_or_else(|| b.top().reg_tmp());
            let rav = RVal::Sym(av);
            */
            let (mut v,q) = cvi(meta.vdef,&meta.dim,&self.ind,b);
            match a{
                None => match q{
                    LVal::Sym(t) => { (v,q,true)},
                    LVal::SymA(t,ind) => {
                        if meta.dim.len() == self.ind.len()+1 {
                            (v,q,true)
                        }else{
                            (v,q,false)
                        }
                    }
                },
                Some(a) => match q{
                    LVal::Sym(t) => { pass(&mut v,a,RVal::Sym(t)); (v,LVal::Sym(a),true)},
                    LVal::SymA(t,ind) => {
                        if meta.dim.len() == self.ind.len()+1 {
                            // array access
                            v.push_back(Inst::Take(a,t,ind));
                            (v,LVal::Sym(a),true)
                        }else{
                            // pointer
                            oper(&mut v,a,RVal::Sym(t),Oper::Add,ind);
                            (v,LVal::Sym(a),false)
                        }
                    }
                }
            }
        }else{panic!("Symbol referred not found in scope");}
    }
    fn cv(&self, a:Option<Var>, b:&mut VStack) -> (VI, RVal){
        // ast::LVal into inst::RVal cast
        // if a = None, then returns orig if a is int, otherwise a+ind / a[ind] depend on 
        // if a = Some(var), then var = lval and returns var
        let (mut v,l,t) = self.cl(a,b);
        match l{
            LVal::Sym(t) => (v,RVal::Sym(t)),
            LVal::SymA(vr,off) => {
                let re = b.top().reg_tmp();
                if t{
                    // completely evaluated
                    v.push_back(Inst::Take(re,vr,off));
                    (v,RVal::Sym(re))
                }else{
                    // partially evaluated
                    oper(&mut v,re,RVal::Sym(vr),Oper::Add,off);
                    (v,RVal::Sym(re))
                }
            }
        }
    }
}
// start control flow compilation
impl Comp for Func{
    fn cc(&self,b:&mut VStack) -> Segment{
        b.enter();
        self.param.iter().for_each(|f| f.rg(b));
        let mut body = vcc(&self.body,b);
        let r = b.exit();
        let mut vdefs = r.dump();
        let fnn = FnName(self.name.to_string());
        vdefs.push_front(Inst::Fn(fnn.clone(),self.param.len() as i32));
        body.ins.push_back(Inst::FnE(fnn));
        Segment::from(mdq(vdefs,body.ins))
    }
}
impl Comp for BlockItem{
    fn cc(&self,b:&mut VStack) -> Segment{
        match self{
            &BlockItem::Decl(ref t) => t.cc(b),
            &BlockItem::Stmt(ref t) => t.cc(b)
        }
    }
}
impl Comp for Stmt{
    fn cc(&self,b:&mut VStack) -> Segment{
        match self{
            &Stmt::Assign(ref l,ref e) => {
                let (mut il,lv,cm) = l.cl(None,b);
                if !cm {panic!("Trying to assign value into a partially evaluated type")}
                let qv = lv.get_ref(b);
                let (mut ins,qb) = e.cv(Some(qv),b);
                lv.store(qv, &mut ins);
                Segment::from(mdq(il,ins))
            },
            &Stmt::Expr(ref e) => Segment::from(e.cv(None,b).0),
            &Stmt::Block(ref e) => vcc(e,b),
            &Stmt::If(ref c,ref l,ref r) => {
                let mut cs:Segment = c.cc(b);
                let (mut ls,tlab) = l.cc(b).lab(b);
                let qlab = b.get_lab();
                let (rs,flab) = match r{
                    None => (Segment::new().plab(qlab),qlab),
                    Some(t) => {
                        let (u,v) =t.cc(b).lab(b);
                        (u.plab(qlab),v)
                    }
                };
                cs=cs.fill(0, tlab);
                cs=cs.fill(1,flab);
                if let None = r {} else {ls=ls.jump(qlab);}
                Segment::cat(Segment::cat(cs,ls),rs)
            },
            &Stmt::While(ref c,ref s) => {
                let (mut cs, cl) :(Segment,i32)= c.cc(b).lab(b);
                let (mut ss ,sl)= s.cc(b).lab(b);
                let q=b.get_lab();
                ss=ss.jump(cl);
                ss=ss.plab(q);
                cs=cs.fill(0,sl);
                cs=cs.fill(1,q);
                ss=ss.fill(2,q);
                ss=ss.fill(3,cl);
                Segment::cat(cs,ss)
            },
            &Stmt::Break => Segment::new().jumpx(Inst::Jump(-1), 2),
            &Stmt::Continue => Segment::new().jumpx(Inst::Jump(-1),3),
            &Stmt::Ret(ref r) => {
                if r.is_n() { Segment::from(VI::from([Inst::Ret])) }
                else {
                    let (mut i,c) = r.cv(None, b);
                    i.push_back(Inst::Return(c));
                    Segment::from(i)
                }
            }
        }
    }
}
impl Expr{
    fn cn(&self,b:&mut VStack) -> Segment{
        match self{
            &Expr::Op(ref u,o,ref v) => {
                if o.is_co() {
                    let (ls,l) = u.cv(None,b);
                    let (rs,r) = v.cv(None,b);
                    let mut zs = Segment::from(mdq(ls,rs));
                    zs = zs.jumpx(Inst::Brch(l,o.to_ee(),r,-1),0);
                    return zs.jumpx(Inst::Jump(-1),1);
                }
            }, _ => {}
        }
        let (zs,q) = self.cv(None,b);
        let mut zs = Segment::from(zs);
        zs = zs.jumpx(Inst::Brch(RVal::Int(0),Oper::Ne,q,-1),0);
        zs.jumpx(Inst::Jump(-1),1)
    }
}
impl Comp for CondExpr{
    fn cc(&self,b:&mut VStack) -> Segment{
        match self{
            &CondExpr::Comp(ref t) => t.cn(b),
            &CondExpr::And(ref l, ref r) =>{
                let ls = l.cc(b);
                let (rs,r) = r.cc(b).lab(b);
                Segment::cat(ls.fill(0,r),rs)
            },
            &CondExpr::Or(ref l,ref r) => {
                let ls = l.cc(b);
                let (rs,r) = r.cc(b).lab(b);
                Segment::cat(ls.fill(1,r),rs)
            }
        }
    }
}
pub fn compile(a:Vec<Box<CompUnit>>) -> Segment {
    let mut scope = eval::VScope::new();
    let mut b = VStack::new();
    let a = eval::vprep(a, &mut scope);
    let rseg = a.into_iter().map(|x| x.cc(&mut b)).reduce(Segment::cat).unwrap_or_else(|| Segment::new());
    let cc = b.exit().dump();
    Segment::cat(Segment::from(cc),rseg)
}
impl Segment{
    pub fn print(&self)->String{
        Vec::from_iter(self.ins.iter().map(|f| f.to_string())).join("\n") + "\n"
    }
}