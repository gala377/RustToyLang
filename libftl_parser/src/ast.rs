pub struct AST {
    root: Program,
}

type NodeId = usize;

pub struct Program {
    pub id: NodeId,
    pub decl: Vec<TopLevelDecl>,
}

pub struct TopLevelDecl {
    pub id: NodeId,
    pub kind: TopLevelDeclKind,
}

pub enum TopLevelDeclKind {
    FunctionDef(FuncDef)
}

pub struct FuncDef {
    pub ty: Type, 
    pub ident: Identifier,
    pub args: Vec<FuncArg>,
    pub body: Expr,
}

pub struct FuncArg {
    pub ty: Option<Type>,
    pub ident: Identifier,
}


pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
}

pub enum ExprKind {
    FunctionCall(FuncCall),
    Literal(Lit),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

pub struct FuncCall {
    pub func: NodeId,
    pub args: Vec<Box<Expr>>,
}

pub enum Lit {
    Int(u64),
}

pub enum BinOp {
    Addition, 
    Substraction,
}

pub struct Type {
    pub kind: TypeKind,
}

pub enum TypeKind {
    Function(FuncType),
    Literal(LitType)    
}

pub struct FuncType {
    pub ret: Box<Type>,
    pub args: Vec<Box<Type>>,
}

pub enum LitType {
    Int, 
    Void,
}

pub struct Identifier { 
    pub symbol: String,
}

