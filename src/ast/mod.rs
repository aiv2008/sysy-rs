#[derive(Debug)]
pub struct FuncDef{
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block
}

impl FuncDef {
    pub fn defNoParam(func_type: FuncType, ident: String, block: Block) -> Self {
        FuncDef {
            func_type,
            ident,
            block,
        }
    }

    pub fn defWithParam(func_type: FuncType, ident: String, func_f_params: FuncFParams, block: Block) -> Self {
        FuncDef {
            func_type,
            ident,
            block,
        }
    }
}

pub struct FuncFParams{
    pub func_f_param: FuncFParam,
}



impl FuncFParams {
        
}

struct FuncFParam;

#[derive(Debug)]
pub enum FuncType{
    Int,
}

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
    pub num: i32,
}
