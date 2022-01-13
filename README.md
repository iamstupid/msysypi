# msysypi
Micro SYSY comPIler

# 计划
## SysY -> Eeyore
先解析成 AST, 然后用类似求值的做法生成代码。

算法: 对于每个函数进行编译，处理语句的时候旁边开一个“变量作用域”栈，栈中的值是HashMap<String,VarDecl>，记录着这个作用域下声明的变量，而查找就往回找直到找到。
### 常量初始化语句
```rust
type VarScope = HashMap<String, VarDecl>;
```

在生成AST的时候可能只知道初始化表达式而不是实际的值，因此初始化的VarInit如下定义，可以包括未计算的值
```rust
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
```

那么表达式求值的模块就应该是
```rust
trait Evaluatable{
    fn parseEval(&self, &mut Vec<HashMap<String,VarDecl>>) -> ValueType;
}
```

最后，高维数组的初始化列表需要把值填到位置上，这需要一个函数
```rust
struct posVal{
    pos: i32,
    val: i32
}
fn toPosVec(Vec<i32> dims, ValueType) -> Vec<posVal>
```
### 其它语句
变量记录如同初始化时候的`VarScope`,需要加一个符号表,Eeyore有三种变量
```rust
enum VarUsage{
    Prim, Temp, Param
};
struct VarSel{
    usage: VarUsage,
    index: i32, // 分配一个编号
    dims: Vec<i32>
}
struct EVarScope{
    tempUsed: i32,
    varMap: HashMap<String, VarSel>
}
```
Eeyore 是指令式的IR, 因此有个`Inst`类型。
```rust
struct Segment{
    label:i32,
    insts: Vec<Inst>,
    conn: Seek,
    backpatch: Backpatcher
}
enum Backpatcher{
    Next(Vec<SkPtr>),
    Cond(Vec<SkPtr>, Vec<SkPtr>)
}
type SkPtr = Rc<RefCell<Segment>>;
enum Seek{
    Nil,
    Return(VarSel),
    Next(SkPtr),
    Cond(Cond,Option<SkPtr>,Option<SkPtr>) // if true, if false
}
enum RVal{
    Val(i32),
    Var(VarSel)
}
type Cond=(LogicOp, RVal, RVal);
```
将指令填到一段段由Seek连接起来的Segment中后，Segment之间的连接情况形成一个图，DFS一下分配编号，按照编号输出成程序即可。

对于短路求值：
```rust
match a:CondExpr{
    Neg(c) => {
        let r = c.compile(); // r: SkPtr
        let mut p = *r.borrow_mut();
        p.backpatch = match p.backpatch{
            Backpatcher::Next(t) => Backpatcher::Next(t),
            Backpatcher::Cond(l,r) => Backpatcher::Cond(r,l)
        };
        r
    },
    And(l,r) => ...
    ...
}
```