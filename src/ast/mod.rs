#[derive(Debug)]
pub struct FuncDef{
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block
}

#[derive(Debug)]
pub struct CompUnit{
    pub fun_def: FuncDef,
}