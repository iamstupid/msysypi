use std::collections::HashMap;

use crate::sysY::ast::*;
pub enum VR{
    Int(i32),
    Arr(Vec<VR>)
}
type Value = Option<VR>;
pub fn bmult(t:&Vec<i32>)->Vec<i32>{
    let mut ac = 1;
    let mut a = t.clone();
    for i in a.iter_mut().rev(){
        ac=ac* *i;
        *i=ac;
    }
    a.push(1);
    a
}
impl VR{
    fn at(&self,ind:&[i32])->i32{
        match self{
            &VR::Int(t) => t,
            &VR::Arr(ref a) => match ind.get(0){
                Some(t) => match a.get(*t as usize){
                    Some(ref b)=>b.at(&ind[1..]),
                    None=>0
                },
                None => 0
            }
        }
    }
    fn val(&self)->i32{
        match self{
            &VR::Int(t) => t,
            &VR::Arr(ref a) => match a.get(0){
                Some(p)=>p.val(),
                None=>0
            }
        }
    }
    fn int_reshape(val:VR, ind:&[i32])->VR{
        match ind.len(){
            1 => VR::Int(val.val()),
            l => {
                let mut ptr = 0;
                let di = ind[0];
                let im = ind[1];
                let mut sub = Vec::new();
                let mut thi = Vec::new();
                match val{
                    VR::Arr(ar) => {
                        for i in ar{
                            match i{
                                VR::Int(t) =>{
                                    sub.push(VR::Int(t));
                                    ptr+=1;
                                    if ptr % im == 0{
                                        let p = sub;
                                        sub=Vec::new();
                                        thi.push(VR::int_reshape(VR::Arr(p),&ind[1..]));
                                        if ptr >= di {break;}
                                    }
                                },
                                VR::Arr(v)=>{
                                    if ptr%im != 0 { 
                                        let p = sub;
                                        sub=Vec::new();
                                        thi.push(VR::int_reshape(VR::Arr(p),&ind[1..]));
                                        ptr=ptr-ptr%im; ptr+=im;
                                    }
                                    if ptr>=di {break;}
                                    thi.push(VR::int_reshape(VR::Arr(v),&ind[1..]));
                                    ptr+=im;
                                    if ptr>=di {break;}
                                }
                            }
                        }
                        if sub.len()!=0 {
                            let p = sub;
                            sub=Vec::new();
                            thi.push(VR::int_reshape(VR::Arr(p),&ind[1..]));
                        }
                        VR::Arr(thi)
                    },
                    t => VR::Arr(vec![t])
                }
            }
        }
    }
    fn reshape(val:Value, ind: &Vec<i32>)->VR{
        let ind = bmult(ind);
        match val{
            None => VR::int_reshape(VR::Int(0), ind.as_slice()),
            Some(val) => VR::int_reshape(val,ind.as_slice())
        }
    }
    fn to_cn(&self) -> InitCont<i32>{
        match self{
            &VR::Int(ref t) => InitCont::Val(*t),
            &VR::Arr(ref a) => InitCont::Vax(a.iter().map(|x| bo(x.to_cn())).collect())
        }
    }
    fn to_vi(&self) -> VarInit{
        VarInit::E(bo(self.to_cn()))
    }
}
pub type Scope = HashMap<String, Box<(VR,usize)>>;
pub struct VScope{
    stack : Vec<Scope>
}
impl VScope{
    pub fn new()->VScope{ VScope{stack:vec![Scope::new()]}}
    pub fn find(&self, name:&String) -> Option<&(VR,usize)>{
        for x in self.stack.iter().rev(){
            match x.get(name){
                Some(t) => return Some(&*t),
                _ => continue
            }
        }
        Option::None
    }
    pub fn enter(&mut self){
        self.stack.push(HashMap::new());
    }
    pub fn exit(&mut self){
        self.stack.pop();
    }
    pub fn register(&mut self,name:&String,val:VR,dim:usize){
        let r = self.stack.len() - 1;
        self.stack[r].insert(name.to_string(), bo((val,dim)));
    }
}

pub trait Reduce{
    type Scope;
    fn reduce(&self, _: &Self::Scope)->Self;
}
pub trait Eval{
    type Eval;
    fn eval(&self) -> Option<Self::Eval>; // eval kinda like unwrap actually
}

impl Eval for Expr{
    type Eval = i32;
    fn eval(&self) -> Option<i32>{
        match self{
            &Expr::Num(t) => Some(t),
            _ => None
        }
    }
}
fn cmp(a:Oper, u:i32, v:i32) -> bool{
    match a{
        Oper::Lt=>u<v, Oper::Le=>u<=v, Oper::Eq=>u==v, Oper::Ne=>u!=v,Oper::Ge=>u>=v,Oper::Gt=>u>v,
        _=>true
    }
}
fn op(a:Oper, u:i32, v:i32) -> i32{
    match a{
        Oper::Add=>u+v, Oper::Sub=>u-v, Oper::Mul=>u*v, Oper::Div=>u/v, Oper::Mod=>u%v,
        t=>cmp(t,u,v) as i32
    }
}
fn uop(a:UOper, u:i32) -> i32{
    match a{
        UOper::Pos => u, UOper::Neg => -u, UOper::Not => !u as i32
    }
}
impl Expr{
    fn is_evaluated(&self)->bool{ match *self{Expr::Num(_)=>true,Expr::Nil=>true,_=>false}}
}
fn ExprOp(l:Expr,o:Oper,r:Expr)->Expr{
    if let Expr::Num(u) = l{
        if let Expr::Num(v) = r{
            return Expr::Num(op(o,u,v));
        }
    }
    return Expr::Op(Box::new(l),o,Box::new(r))
}
fn ExprUop(o:UOper, r:Expr)->Expr{
    if let Expr::Num(v) = r{
        return Expr::Num(uop(o,v));
    }
    Expr::UOp(o,Box::new(r))
}
/*
impl Reduce for Expr{
    type Scope = VScope;
    fn reduce(&self, scope:&VScope) -> Expr{
        match self{
            &Expr::Nil => Expr::Nil,
            &Expr::FnCall(ref st, ref ve) => Expr::FnCall(st.to_string(),ve.iter().map(|x| Box::new(x.reduce(scope))).collect()),
            &Expr::Num(ref t) => Expr::Num(*t),
            &Expr::LVal(ref l) =>{
                match scope.find(&l.name){
                    None => {
                        let c=l.ind.iter().map(|x| Box::new(x.reduce(scope))).collect();
                        Expr::LVal(LVal{
                            name:l.name.to_string(),
                            ind:c
                        })
                    },
                    Some(t) => {
                        let c=l.ind.iter().map(|x| Box::new(x.reduce(scope)));
                        let z:Vec<Exp>=c.collect();
                        let r=z.iter().map(|x| x.eval());
                        let mut ind=vec![];
                        for i in r{
                            match i{
                                Some(t) => ind.push(t),
                                None => return Expr::LVal(LVal{
                                    name:l.name.to_string(),
                                    ind:z
                                })
                            }
                        }
                        Expr::Num(t.at(ind.as_slice()))
                    }
                }
            },
            &Expr::Op(ref l,o,ref r)=>ExprOp(l.reduce(scope),o,r.reduce(scope)),
            &Expr::UOp(o,ref r)=>ExprUop(o,r.reduce(scope))
        }
    }
}
*/
// VarInit cast
impl InitCont<Exp>{
    fn valCast(self) -> ValueType{
        match self{
            InitCont::Val(x) => bo(InitCont::Val(x.eval().unwrap())), // if not completely evaluated will panic
            InitCont::Vax(t) => bo(InitCont::Vax(t.into_iter().map(|x| x.valCast()).collect()))
        }
    }
}
impl InitCont<i32>{
    fn toValue(&self) -> VR{
        match self{
            InitCont::Val(x) => VR::Int(*x),
            InitCont::Vax(x) => VR::Arr(x.iter().map(|x| x.toValue()).collect())
        }
    }
}
impl VarInit{
    fn valCast(self) -> Self{
        match self{
            VarInit::Nil => VarInit::Nil,
            VarInit::I(t) => VarInit::E(t.valCast()),
            VarInit::E(t) => VarInit::E(t)
        }
    }
    fn toValue(&self) -> Value{
        match self{
            VarInit::Nil => Value::None,
            VarInit::I(t) => panic!(),
            VarInit::E(t) => Some(t.toValue())
        }
    }
}
fn ifpush<T>(q:Option<Vec<T>>,p:Option<T>)->Option<Vec<T>>{
    match (q,p){
        (None,_) => None,
        (_,None) => None,
        (Some(mut t),Some(p)) => {t.push(p); Some(t)}
    }
}
pub fn incast(t:&Vec<Exp>)->Option<Vec<i32>>{
    t.iter().fold(Some(Vec::new()),|v,x| ifpush(v,x.eval()))
}
pub fn vicast(t:&Vec<Exp>)->Vec<i32>{
    t.iter().map(|x| x.eval().unwrap()).collect()
}
pub fn dicast(t:&Vec<Exp>)->Vec<i32>{
    // dimension cast where Expr::Nil is evaluated into -1
    t.iter().map(|x| match x.eval(){Some(t)=>t,None=>-1}).collect()
}
// preprocess
fn bo<T>(a:T)->Box<T>{ Box::new(a) }
pub trait Prep{
    fn prep(self, scope: &mut VScope) -> Self;
}
pub fn vprep<T:Prep>(a:Vec<Box<T>>,s:&mut VScope)->Vec<Box<T>>{
    a.into_iter().map(|x| bo(x.prep(s))).collect()
}
fn vpp(a:Block,s:&mut VScope)->Block{
    s.enter();
    let a=vprep(a,s);
    s.exit();
    a
}
impl Prep for CompUnit{
    fn prep(self, scope: &mut VScope) -> CompUnit{
        match self{
            CompUnit::Decl(t) => CompUnit::Decl(bo(t.prep(scope))),
            CompUnit::Func(t) => CompUnit::Func(bo(t.prep(scope)))
        }
    }
}
impl Prep for Decl{
    fn prep(self, scope: &mut VScope) -> Decl{
        let Decl(l,r) = self;
        Decl(l,vprep(r,scope))
    }
}
impl Prep for VarDecl{
    fn prep(self, scope: &mut VScope) -> Self{
        let mut r = VarDecl{
            dims:vprep(self.dims,scope),
            init:self.init.prep(scope),
            ..self
        };
        if r.dtype == DeclType::Const{
            r.init = r.init.valCast();
            let eval = r.init.toValue();
            let dims = vicast(&r.dims);
            let eval = VR::reshape(eval, &dims);
            r.init = eval.to_vi();
            scope.register(&r.name, eval,dims.len());
            r
        }else {r}
    }
}
impl Prep for VarInit{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            VarInit::Nil => VarInit::Nil,
            VarInit::I(e) => VarInit::I(Box::new(e.prep(scope))),
            VarInit::E(e) => VarInit::E(e)
        }
    }
}
impl Prep for InitCont<Exp>{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            InitCont::Val(e) => InitCont::Val(bo(e.prep(scope))),
            InitCont::Vax(e) => InitCont::Vax(e.into_iter().map(|x| bo(x.prep(scope))).collect())
        }
    }
}
impl Prep for Func{
    fn prep(self, scope: &mut VScope) -> Self{
        Func{
            param:vprep(self.param,scope),
            body:vpp(self.body,scope),
            ..self
        }
    }
}
impl Prep for BlockItem{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            BlockItem::Decl(t) => BlockItem::Decl(bo(t.prep(scope))),
            BlockItem::Stmt(t) => BlockItem::Stmt(bo(t.prep(scope)))
        }
    }
}
impl Prep for Stmt{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            Stmt::Assign(l,e) => Stmt::Assign(l.prep(scope), bo(e.prep(scope))),
            Stmt::Expr(e) => Stmt::Expr(bo(e.prep(scope))),
            Stmt::Block(b) => Stmt::Block(vpp(b,scope)),
            Stmt::If(c,t,s)=>Stmt::If(bo(c.prep(scope)),bo(t.prep(scope)),s.map(|x| bo(x.prep(scope)))),
            Stmt::While(c,s) => Stmt::While(bo(c.prep(scope)),bo(s.prep(scope))),
            Stmt::Ret(e) => Stmt::Ret(bo(e.prep(scope))),
            n => n
        }
    }
}
impl Prep for CondExpr{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            CondExpr::And(a,b) => CondExpr::And(bo(a.prep(scope)),bo(b.prep(scope))),
            CondExpr::Or(a,b) => CondExpr::Or(bo(a.prep(scope)),bo(b.prep(scope))),
            CondExpr::Comp(a) => CondExpr::Comp(bo(a.prep(scope)))
        }
    }
}
impl Prep for LVal{
    fn prep(self, scope: &mut VScope) -> Self{
        LVal{
            ind:vprep(self.ind, scope),
            ..self
        }
    }
}
impl Prep for Expr{
    fn prep(self, scope: &mut VScope) -> Self{
        match self{
            Expr::Nil => Expr::Nil,
            Expr::FnCall(a,b) => Expr::FnCall(a,vprep(b,scope)),
            Expr::Num(a) => Expr::Num(a),
            Expr::LVal(a) =>{
                let a = a.prep(scope);
                let val = scope.find(&a.name);
                if let Some((t,d)) = val{
                    let r = incast(&a.ind);
                    if let Some(r) = r{
                        if r.len() == *d{
                            Expr::Num(t.at(r.as_slice()))
                        }else {Expr::LVal(a)}
                    }else {Expr::LVal(a)}
                }else {
                    Expr::LVal(a)
                }
            },
            Expr::Op(l,o,r)=>ExprOp(l.prep(scope),o,r.prep(scope)),
            Expr::UOp(o,r)=>ExprUop(o,r.prep(scope))
        }
    }
}