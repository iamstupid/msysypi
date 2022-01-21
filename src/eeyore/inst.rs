use derive_more::{Display};
use std::fmt;
#[derive(Display,Copy,Clone,PartialEq)]
pub enum UOper{
    #[display(fmt="-")]
    Neg,
    #[display(fmt="!")]
    Not
}
#[derive(Display,Copy,Clone,PartialEq)]
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
    Gt,
    #[display(fmt="&&")]
    And,
    #[display(fmt="||")]
    Or
}
#[derive(Display,Copy,Clone,PartialEq)]
pub enum VarUsage{
    #[display(fmt="T")]
    Var,
    #[display(fmt="t")]
    Temp,
    #[display(fmt="p")]
    Param
}
#[derive(Display,Copy,Clone,PartialEq)]
#[display(fmt="{}{}",_0,_1)]
pub struct Var(pub VarUsage,pub i32);
#[derive(Display,Copy,Clone,PartialEq)]
pub enum LVal{
    #[display(fmt="{}",_0)]
    Sym(Var),
    #[display(fmt="{}[{}]",_0,_1)]
    SymA(Var,RVal),
}
#[derive(Display,Copy,Clone,PartialEq)]
pub enum RVal{
    #[display(fmt="{}",_0)]
    Sym(Var),
    #[display(fmt="{}",_0)]
    Int(i32)
}
#[derive(Display,Clone)]
#[display(fmt="f_{}",_0)]
pub struct FnName(pub String);

#[derive(Display,Clone)]
pub enum Inst{
    #[display(fmt="\tvar {} {}",_0,_1)]
    DArr(i32,Var),
    #[display(fmt="\tvar {}",_0)]
    DI(Var),
    #[display(fmt="\t{} = {}",_0,_1)]
    Ass(LVal,RVal),
    #[display(fmt="\t{} = {}[{}]",_0,_1,_2)]
    Take(Var,Var,RVal),
    #[display(fmt="\t{} = {} {} {}",_0,_1,_2,_3)]
    Cpt(Var,RVal,Oper,RVal),
    #[display(fmt="\t{} = {} {}",_0,_1,_2)]
    UCpt(Var,UOper,RVal),
    #[display(fmt="\tif {} {} {} goto l{}",_0,_1,_2,_3)]
    Brch(RVal,Oper,RVal,i32),
    #[display(fmt="\tgoto l{}",_0)]
    Jump(i32),
    #[display(fmt="l{}:",_0)]
    Label(i32),
    #[display(fmt="\tparam {}",_0)]
    Param(RVal),
    #[display(fmt="\tcall {}",_0)]
    Call(FnName),
    #[display(fmt="\t{} = call {}",_0,_1)]
    CallFn(Var,FnName),
    #[display(fmt="\treturn {}",_0)]
    Return(RVal),
    #[display(fmt="\treturn")]
    Ret,
    #[display(fmt="{} [{}]",_0,_1)]
    Fn(FnName,i32),
    #[display(fmt="end {}",_0)]
    FnE(FnName)
}
/*
    #[derive(Clone)]
    pub struct Fn{
        pub name:FnName,
        pub pcnt:i32,
        pub inst:Vec<Inst>
    }
    impl fmt::Display for Fn{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"{} [{}]\n",self.name,self.pcnt);
            self.inst.iter().for_each(|x| {write!(f,"{}\n",x);});
            write!(f,"end {}\n",self.name)
        }
    }
    #[derive(Display,Clone)]
    pub enum Unit{
        #[display(fmt="{}",_0)]
        I(Inst),
        #[display(fmt="{}",_0)]
        F(Fn)
    }
*/