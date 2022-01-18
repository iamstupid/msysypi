use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::eeyore::inst::*;
struct Vmeta{
    dim:Vec<i32>,
    vdef:Var
}
struct VScope{
    map:HashMap<String,Vmeta>,
    local:i32,
    temp:i32,
    param:i32
}
struct VStack{
    st:Vec<VScope>
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
}
impl VStack{
    fn new()->VStack{
        VStack{st:vec![VScope::new(0)]}
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
}
type SkPtr = Rc<RefCell<Segment>>;
enum SPtr{
    Undef,
    Next(SkPtr),
    Ret(Option<RVal>),
    Brch(Inst,Option<SkPtr>,Option<SkPtr>)
}
struct Segment{
    ins: Vec<Inst>,
    seek: SPtr,
    truelist: Vec<SkPtr>,
    falselist: Vec<SkPtr>,
    accessFlag: bool
}
fn ns()->Segment{
    Segment { ins: vec![], seek: SPtr::Undef, truelist: vec![], falselist: vec![], accessFlag: false }
}
fn 
