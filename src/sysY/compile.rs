use std::{collections::{HashMap, VecDeque}, cell::RefCell, rc::Rc};

use crate::eeyore::inst::*;
use crate::sysY::eval::{incast,dicast,vicast};

use super::{ast::{CompUnit, Decl, VarDecl, InitCont, VarInit, Expr, Exp, Func, BlockItem}, eval::bmult};
use super::ast;
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
        let r = self.st.iter().rev().next().unwrap().local + 1;
        self.st.push(VScope::new(r));
    }
    fn exit(&mut self)->VScope{
        self.st.pop().unwrap()
    }
    fn find(&self, name:&String)->Option<&Vmeta>{
        self.st.iter().rev().find_map(|t| t.map.get(name))
    }
    fn top(&mut self)->&mut VScope{ self.st.iter_mut().rev().next().unwrap() }
}
struct Segment{
    ins: VecDeque<Inst>,
    list: Vec<Vec<i32>>,
    ret: Option<RVal>
}
fn mls(a:Vec<i32>,b:Vec<i32>,c:i32)->Vec<i32>{
    b.iter().for_each(|t| a.push(t+c));
    a
}
fn mdq(a:VecDeque<Inst>, b:VecDeque<Inst>)->VecDeque<Inst>{
    if a.len() < b.len(){
        a.iter().rev().for_each(|f| b.push_front(*f));
        b
    }else{
        b.iter().for_each(|f| a.push_back(*f));
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
        self.list=self.list.into_iter().map(|x| x.iter().map(|z| z+1).collect()).collect();
        self
    }
    fn fill(self,i:usize,lab:i32) -> Segment{
        for i in self.list[i]{
            let i = i as usize;
            self.ins[i] = match self.ins[i]{
                Inst::Brch(a,b,c,_) => Inst::Brch(a,b,c,lab),
                Inst::Jump(_) => Inst::Jump(lab),
                t => t
            }
        }
        self
    }
}
trait Comp{
    fn cc(&self,b:&mut VStack) -> Segment;
}
impl Comp for CompUnit{
    fn cc(&self,b:&mut VStack) -> Segment{
        match self{
            &CompUnit::Decl(a) => a.cc(b),
            &CompUnit::Func(a) => a.cc(b)
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
fn rci(a:Var,dim:Vec<i32>,init:&VarInit,b:&mut VStack)->Segment{
    match init{
        &VarInit::Nil => Segment::new(),
        &VarInit::E(t) => Segment::from(t.cz(a,0,dim.as_slice(),b)),
        &VarInit::I(t) => Segment::from(t.cz(a,0,dim.as_slice(),b))
    }
}
impl InitCont<i32>{
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
                _ => {}
            }
        }
        let av = a.unwrap_or_else(|| b.top().reg_tmp());
        let rav = RVal::Sym(av);
        match self{
            &Expr::Nil => (VI::new(),RVal::Sym(av)),
            &Expr::Num(t) => (VI::from([Inst::Ass(LVal::Sym(av),RVal::Int(t))]),RVal::Sym(av)),
            t =>{
                match t{
                    &Expr::FnCall(ref fnm,ref para) => {
                        let mut para=para.iter().map(|x|{
                            let par = b.top().reg_tmp();
                            let z=x.cz(LVal::Sym(par),b);
                            z.push_back(Inst::Param(RVal::Sym(par)));
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
            let c = b.top().reg_tmp();
            let d = b.top().reg_tmp();
            v=mdq(v,ov);
            oper(&mut v,c,or,Oper::Mul,RVal::Int(mult));
            oper(&mut v,d,RVal::Sym(c),Oper::Add,rb);
            (v,LVal::SymA(ra,RVal::Sym(d)))
        }else {panic!("Impossiblu");}
    }
}
fn cvi(a:Var, dim: &Vec<i32>, ind: &Vec<ast::Exp>, b: &mut VStack) -> (VI, LVal) {
    // if it is a int type, then return the symbol
    // else, return an array offset
    // for the caller, if |dim| = |ind|+1 then the index should be accessed with a[ind]
    // otherwise a + ind
    if dim.len() == 0 { return (VI::new(),LVal::Sym(a));}
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
        let (v,l,t) = self.cl(a,b);
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
        let body = vcc(&self.body,b);
        let r = b.exit();
        let vdefs = r.dump();
        let fnn = FnName(self.name.to_string());
        vdefs.push_front(Inst::Fn(fnn.clone(),self.param.len() as i32));
        body.ins.push_back(Inst::FnE(fnn));
        Segment::from(mdq(vdefs,body.ins))
    }
}
impl Comp for BlockItem{
    fn cc(&self,b:&mut VStack) -> Segment{
        
    }
}