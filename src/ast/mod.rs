use std::sync::Arc;

#[derive(Debug)]
pub struct FuncDef{
    pub func_type: String,
    pub ident: String,
    pub block: Block
}


// #[derive(Debug)]
// pub enum FuncType{
//     Int,
// }


#[derive(Debug)]
pub struct CompUnit{
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct Block{
    pub stmt: Stmt,
}

#[derive(Debug)]
pub enum Stmt{
    // pub num: i32,
    Expr,
}

#[derive(Debug)]
pub enum PrimaryExp{
    Number(i32),
    Expr,
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp,
    UnaryOpExp(String, Arc<UnaryExp>),
}

#[derive(Debug)]
pub struct  Expr{
    pub unary_exp: UnaryExp,
}

