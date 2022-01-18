use crate::sysY::util;
use std::fmt;
use derive_more::{Display};

#[derive(Display)]
pub enum CompUnit {
    #[display(fmt="{} {};\n","_0.0","pvec(&_0.1,\",\",\"\",\"\")")]
    Decl(Box<Decl>),
    #[display(fmt="{}",_0)]
    Func(Box<Func>)
}

#[derive(Copy,Clone,Display,PartialEq)]
pub enum DeclType{ 
    #[display(fmt="const int")]
    Const, 
    #[display(fmt="int")]
    Var 
}
#[derive(Copy,Clone,Display,PartialEq)]
pub enum FunctType{ 
    #[display(fmt="void")]
    Void, 
    #[display(fmt="int")]
    Int 
}
#[derive(Copy,Clone,Display,PartialEq)]
pub enum UOper{
    #[display(fmt="-")]
    Neg,
    #[display(fmt="+")]
    Pos,
    #[display(fmt="!")]
    Not
}
#[derive(Copy,Clone,Display,PartialEq)]
pub enum Oper{
    #[display(fmt="+")]
    Add,
    #[display(fmt="-")]
    Sub,
    #[display(fmt="*")]
    Mul,
    #[display(fmt="/")]
    Div,
    #[display(fmt="%")]
    Mod,
    #[display(fmt="<")]
    Lt,
    #[display(fmt="<=")]
    Le,
    #[display(fmt="==")]
    Eq,
    #[display(fmt="!=")]
    Ne,
    #[display(fmt=">=")]
    Ge,
    #[display(fmt=">")]
    Gt
}

/// Vector printer
pub struct VecPrinter<'a, T:fmt::Display>{
    obj: &'a Vec<T>,
    start: String,
    sep: String,
    end: String
}
impl<T:fmt::Display> fmt::Display for VecPrinter<'_,T>{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
        let mut co = 0;
        if self.obj.len()==0 {return write!(f,"");}
        for i in self.obj.iter() {
            match co{
                0 => write!(f,"{} {}",self.start,i),
                _ => write!(f,"{} {}",self.sep,i)
            };
            co+=1;
        }
        write!(f,"{}",self.end)
    }
}
pub fn pvec<'a, T:fmt::Display>(a:&'a Vec<T>,sep:&str,start:&str,end:&str)->VecPrinter<'a,T>{
    VecPrinter{obj:a,sep:sep.to_string(),start:start.to_string(),end:end.to_string()}
}
fn dvec<'a, T:fmt::Display>(a:&'a Vec<T>)->VecPrinter<'a,T>{
    pvec(a,"][","[","]")
}
fn prvec<'a, T:fmt::Display>(a:&'a Vec<T>)->VecPrinter<'a,T>{
    pvec(a,",","","")
}

pub type VarDecls = Vec<Box<VarDecl>>;
pub struct Decl(pub DeclType, pub VarDecls);
#[derive(Display)]
#[display(fmt="{} {} {}",name,"dvec(&dims)",init)]
pub struct VarDecl{
    pub dtype: DeclType,
    pub name: String,
    pub dims: Vec<Exp>,
    pub init: VarInit
}
pub type Exp = Box<Expr>;
pub type Con = Box<CondExpr>;
#[derive(Display)]
pub enum VarInit{
    #[display(fmt="")]
    Nil,
    #[display(fmt="= {}",_0)]
    I(ExprInit),
    #[display(fmt="= {}",_0)]
    E(ValueType)
}
// 我们需要在VarInit中表达是否完成了求值的变量
// VarInit本质上装的应该是一个高维列表，如果求完了值就是整数(i32)，因此我们构造一种通用数据结构
#[derive(Display)]
#[display(bound = "T:fmt::Display")]
pub enum InitCont<T>{
    #[display(fmt="{}",_0)]
    Val(T),
    #[display(fmt="{{ {} }}","prvec(&_0)")]
    Vax(Vec<Box<InitCont<T>>>)
}
pub type ValueType = Box<InitCont<i32>>;
pub type ExprInit = Box<InitCont<Exp>>;

#[derive(Display)]
#[display(fmt="{} {}({}){{\n{}\n}}\n",ret,name,"prvec(&param)","pvec(&body,\"\n\",\"\",\"\")")]
pub struct Func{
    pub name: String,
    pub ret: FunctType,
    pub param: VarDecls,
    pub body: Block
}
#[derive(Display)]
pub enum BlockItem{
    #[display(fmt="{} {};\n","_0.0","prvec(&_0.1)")]
    Decl(Box<Decl>),
    #[display(fmt="{}",_0)]
    Stmt(Box<Stmt>)
}
pub type Block=Vec<Box<BlockItem>>;
pub enum Stmt{
    Assign(LVal,Exp),
    Expr(Exp),
    Block(Block),
    If(Con,Box<Stmt>,Option<Box<Stmt>>),
    While(Con,Box<Stmt>),
    Break,
    Continue,
    Ret(Exp)
}
impl fmt::Display for Stmt{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            &Stmt::Assign(ref l,ref e) => write!(f,"{} = {};\n",l,e),
            &Stmt::Expr(ref e) => write!(f,"{};\n",e),
            &Stmt::Block(ref e) => write!(f,"{{\n{}\n}}\n",pvec(e,"\n","","")),
            &Stmt::If(ref c,ref t,ref x) => match x{
                &None => write!(f,"if( {} ) {}",c,t),
                &Some(ref x) => write!(f,"if( {} ) {} else {}",c,t,x)
            },
            &Stmt::While(ref c,ref s) => write!(f,"while( {} ) {}",c,s),
            &Stmt::Break => write!(f,"break;\n"),
            &Stmt::Continue => write!(f,"continue;\n"),
            &Stmt::Ret(ref e) => write!(f,"return {};\n",e)
        }
    }
}

#[derive(Display)]
#[display(fmt="{}{}",name,"dvec(&ind)")]
pub struct LVal{
    pub name: String,
    pub ind: Vec<Exp>
}
#[derive(Display)]
pub enum Expr{
    #[display(fmt="")]
    Nil,
    #[display(fmt="{}({})",_0,"prvec(&_1)")]
    FnCall(String, Vec<Exp>), // function name, parameter
    #[display(fmt="{}",_0)]
    Num(i32),
    #[display(fmt="{}",_0)]
    LVal(LVal),
    #[display(fmt="({}) {} ({})",_0,_1,_2)]
    Op(Exp,Oper,Exp),
    #[display(fmt="{} ({})",_0,_1)]
    UOp(UOper,Exp)
}
#[derive(Display)]
pub enum CondExpr{
    #[display(fmt="({}) && ({})",_0,_1)]
    And(Con,Con),
    #[display(fmt="({}) || ({})",_0,_1)]
    Or(Con,Con),
    #[display(fmt="{}",_0)]
    Comp(Exp)
}
impl Stmt{
    pub fn If1(a:Con, b:Box<Stmt>) -> Box<Stmt>{
        Box::new(Stmt::If(a,b,Option::None))
    }
    pub fn If2(a:Con, b:Box<Stmt>, c:Box<Stmt>) -> Box<Stmt> {
        Box::new(Stmt::If(a,b,Option::Some(c)))
    }
    pub fn Whi(a:Con, b:Box<Stmt>) -> Box<Stmt> {
        Box::new(Stmt::While(a,b))
    }
}