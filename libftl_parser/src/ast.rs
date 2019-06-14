use ftl_source::{
    Span,
    Source, 
    Pointer,
};

pub struct AST<S: Source> {
    root: Module<S::Pointer>,
}

impl<S: Source> AST<S> {
    pub fn new(root: Module<S::Pointer>) -> Self {
        Self {
            root,
        }
    } 
}

pub type NodeId = usize;

pub struct Module<T: Pointer> {
    pub id: NodeId,
    pub decl: Vec<TopLevelDecl<T>>,
}

pub struct TopLevelDecl<T: Pointer> {
    pub id: NodeId,
    pub kind: TopLevelDeclKind<T>,
    pub span: Span<T>,
}

pub enum TopLevelDeclKind<T: Pointer> {
    FunctionDef(FuncDef<T>)
}

pub struct FuncDef<T: Pointer> {
    pub ty: Type, 
    pub ident: Ident<T>,
    pub args: Vec<FuncArg<T>>,
    pub body: Expr<T>,
}

pub struct FuncArg<T: Pointer> {
    pub ty: Option<Type>,
    pub ident: Ident<T>,
    pub span: Span<T>
}


pub struct Expr<T: Pointer> {
    pub id: NodeId,
    pub kind: ExprKind<T>,
    pub span: Span<T>,
}

pub enum ExprKind<T: Pointer> {
    FunctionCall(FuncCall<T>),
    Literal(Lit),
    Identifier(Ident<T>),
    Binary(BinOp, Box<Expr<T>>, Box<Expr<T>>),
}

pub struct FuncCall<T: Pointer> {
    pub ident: Ident<T>,
    pub args: Vec<Box<Expr<T>>>,
}

pub enum Lit {
    Int(u64),
}

pub enum BinOp {
    Addition, 
    Substraction,
}

pub struct Ident<T: Pointer> { 
    pub symbol: String,
    pub span: Span<T>,
}



/// Types


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



