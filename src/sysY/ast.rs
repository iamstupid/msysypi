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
enum UOper{ Neg, Pos, Not }
#[derive(Copy,Clone)]
enum Oper{ Add, Sub, Mul, Div, Mod, lt, le, eq, ne, ge, gt }


type VarDecls = Vec<Box<VarDecl>>;
struct Decl(DeclType, VarDecls);
struct VarDecl{
    dtype: DeclType,
    name: String,
    dims: Vec<i32>,
    init: VarInit
}
type Exp = Box<Expr>;
type Con = Box<CondExpr>;
enum VarInit{
    Nil,
    I(ExprInit),
    E(ValueType)
}
// 我们需要在VarInit中表达是否完成了求值的变量
// VarInit本质上装的应该是一个高维列表，如果求完了值就是整数(i32)，因此我们构造一种通用数据结构
enum InitCont<T>{
    Nil,
    Val(T,Box<InitCont<T>>), // 该值是一个单值
    Lis(Box<InitCont<T>>, Box<InitCont<T>>) // 该值是一个列表
}
type ValueType = InitCont<i32>;
type ExprInit = InitCont<Exp>;

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
    Op(Oper,Exp,Exp),
    UOp(UOper,Exp)
}
enum CondExpr{
    Neg(Con),
    And(Con,Con),
    Or(Con,Con),
    Comp(Expr)
}