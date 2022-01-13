enum Ls<T>{
    C(T,Box<Ls<T>>),
    N
}
