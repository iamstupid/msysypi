use crate::sysY::util;
enum CompUnit {
    Decl(Decl),
    Func(Func)
}

#[derive(Copy,Clone)]
enum DeclType{ Const, Var }
#[derive(Copy,Clone)]
enum FunctType{ Void, Int }
#[derive(Copy,Clone)]
enum UOper{ Neg, Pos }
#[derive(Copy,Clone)]
enum Oper{ Add, Sub, Mul, Div, Mod }
#[derive(Copy,Clone)]
enum COper{ lt, le, eq, ne, ge, gt }


type VarDecls = Vec<Box<VarDecl>>;
struct Decl(DeclType, VarDecls);
struct VarDecl{
    name: String,
    dims: Vec<i32>,
    init: Option<VarInit>
}
type Exp = Box<Expr>;
type Con = Box<CondExpr>;
enum VarInit{
    Nil,
    A(Exp, Box<VarInit>),
    B(Box<VarInit>, Box<VarInit>)
}
struct Funct{
    name: String,
    ret: FunctType,
    param: VarDecls,
    body: Block
}
enum BlockItem{
    Decl(Decl),
    Stmt(Stmt)
}
type Block=Vec<Box<BlockItem>>;
enum Stmt{
    Assign(LVal,Exp),
    Expr(Exp),
    Block(Block),
    If(Con,Box<Stmt>,Option<Box<Stmt>>),
    While(Con,Box<Stmt>),
    Break,
    Continue,
    Ret(Option<Exp>)
}
struct LVal{
    name: String,
    ind: Vec<i32>
}
enum Expr{
    FnCall(String, Vec<Exp>), // function name, parameter
    Num(i32),
    LVal(LVal),
    Op(),
    UOp(UOper,Exp)
}