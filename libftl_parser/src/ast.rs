use ftl_source::{
    Span,
    Source, 
    Pointer,
};

pub struct AST<S: Source> {
    pub root: Module<S::Pointer>,
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
    FunctionDef(FuncDef<T>),
    FunctionDecl(FuncDecl<T>),
    InfixDef(InfixDef<T>),
}

pub struct FuncDecl<T: Pointer> {
    pub ty: Option<Type<T>>, // for now, we dont have infering yet 
    pub attrs: Vec<Ident<T>>,
    pub ident: Ident<T>,
}

pub struct FuncDef<T: Pointer> {
    pub decl: FuncDecl<T>,
    pub args: Vec<FuncArg<T>>,
    pub body: Expr<T>,
}

pub struct InfixDef<T: Pointer> {
    pub ty: Option<Type<T>>,
    pub precedence: usize,
    pub op: Op<T>,
    pub args: (FuncArg<T>, FuncArg<T>),
    pub body: Expr<T>,

}

pub struct FuncArg<T: Pointer> {
    pub ty: Option<Type<T>>,
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
    Literal(Lit<T>),
    Identifier(Ident<T>),
    Binary(BinOp<T>, Box<Expr<T>>, Box<Expr<T>>),
    // We need it because parser doesn't
    // know about precedence but the 
    // later passes need to know about them.
    Parenthesed(Box<Expr<T>>), 
}

pub struct FuncCall<T: Pointer> {
    pub lhs: Box<Expr<T>>,
    pub args: Vec<Box<Expr<T>>>,
}

pub struct Lit<T: Pointer> {
    pub kind: LitKind,
    pub span: Span<T>,
}

pub enum LitKind {
     Int(u64),
}

pub enum BinOp<T: Pointer> {
    Ident(Ident<T>),
    Op(Op<T>),
}

pub struct Op<T: Pointer> {
    pub symbol: String,
    pub span: Span<T>,
}

pub struct Ident<T: Pointer> { 
    pub symbol: String,
    pub span: Span<T>,
}



/// Types


pub struct Type<T: Pointer> {
    pub kind: TypeKind<T>,
    pub span: Span<T>,
}

pub enum TypeKind<T: Pointer> {
    Function(FuncType<T>),
    Literal(LitType)    
}

pub struct FuncType<T: Pointer> {
    pub ret: Box<Type<T>>,
    pub args: Vec<Box<Type<T>>>,
}

pub enum LitType {
    Int, 
    Void,
}

pub fn is_lit_type(symbol: &str) -> Option<LitType> {
    use LitType::*;
    match symbol {
        "int" => Some(Int),
        "void" => Some(Void),
        _ => None,
    }
}



