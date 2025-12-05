use std::sync::Arc;

#[derive(Debug)]
pub struct FuncDef{
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block
}

#[derive(Debug)]
pub enum FuncType{
    Int,
    Void,
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
pub struct Stmt{
    // pub num: i32,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum PrimaryExp{
    Number(i32),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    UnaryOp(UnaryOp, Arc<UnaryExp>),
}

#[derive(Debug)]
pub struct  Expr{
    pub unary_exp: UnaryExp,
}

#[derive(Debug)]
pub enum UnaryOp{
    ADD,
    SUB,
    MULT,
    DIV,
    NEQ,
}