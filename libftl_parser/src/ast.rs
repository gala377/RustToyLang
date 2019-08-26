use ftl_source::{Pointer, Source, Span};

pub struct AST<S: Source> {
    pub root: Module<S::Pointer>,
}

impl<S: Source> AST<S> {
    pub fn new(root: Module<S::Pointer>) -> Self {
        Self { root }
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

#[derive(Clone)]
pub struct FuncDecl<T: Pointer> {
    pub id: NodeId,
    pub ty: Option<Type<T>>, // for now, we dont have infering yet
    pub attrs: Vec<FuncAttr<T>>,
    pub ident: Ident<T>,
}

pub struct FuncDef<T: Pointer> {
    pub id: NodeId,
    pub decl: FuncDecl<T>,
    pub args: Vec<FuncArg<T>>,
    pub body: Expr<T>,
}

pub struct InfixDef<T: Pointer> {
    pub id: NodeId,
    pub ty: Option<Type<T>>,
    pub precedence: usize,
    pub op: Op<T>,
    pub args: (FuncArg<T>, FuncArg<T>),
    pub body: Expr<T>,
}

pub struct FuncArg<T: Pointer> {
    pub id: NodeId,
    pub ty: Option<Type<T>>,
    pub ident: Ident<T>,
    pub span: Span<T>,
}

#[derive(Clone)]
pub struct FuncAttr<T: Pointer> {
    pub id: NodeId,
    pub ident: Ident<T>,
}

#[derive(Clone)]
pub struct Expr<T: Pointer> {
    pub id: NodeId,
    pub kind: ExprKind<T>,
    pub span: Span<T>,
}

#[derive(Clone)]
pub enum ExprKind<T: Pointer> {
    FunctionCall(FuncCall<T>),
    Literal(Lit<T>),
    Identifier(Ident<T>),
    InfixFuncCall(InfixFuncCall<T>),
    InfixOpCall(InfixOpCall<T>),
    // We need it because parser doesn't
    // know about precedence but the
    // later passes need to know about them.
    Parenthesed(Paren<T>),
}

#[derive(Clone)]
pub struct FuncCall<T: Pointer> {
    pub id: NodeId,
    pub lhs: Box<Expr<T>>,
    pub args: Vec<Expr<T>>,
}

#[derive(Clone)]
pub struct Paren<T: Pointer> {
    pub id: NodeId,
    pub expr: Box<Expr<T>>,
}

#[derive(Clone)]
pub struct InfixFuncCall<T: Pointer> {
    pub id: NodeId,
    pub ident: Ident<T>,
    pub lhs: Box<Expr<T>>,
    pub rhs: Box<Expr<T>>,
}

#[derive(Clone)]
pub struct InfixOpCall<T: Pointer> {
    pub id: NodeId,
    pub op: Op<T>,
    pub lhs: Box<Expr<T>>,
    pub rhs: Box<Expr<T>>,
}

#[derive(Clone)]
pub struct Lit<T: Pointer> {
    pub id: NodeId,
    pub kind: LitKind,
    pub span: Span<T>,
}

#[derive(Clone)]
pub enum LitKind {
    Int(u64),
}

#[derive(Clone)]
pub struct Op<T: Pointer> {
    pub id: NodeId,
    pub symbol: String,
    pub span: Span<T>,
}

#[derive(Clone)]
pub struct Ident<T: Pointer> {
    pub id: NodeId,
    pub symbol: String,
    pub span: Span<T>,
}

/// Types

#[derive(Clone)]
pub struct Type<T: Pointer> {
    pub id: NodeId,
    pub kind: TypeKind<T>,
    pub span: Span<T>,
}

#[derive(Clone)]
pub enum TypeKind<T: Pointer> {
    Function(FuncType<T>),
    Literal(LitType),
}

#[derive(Clone)]
pub struct FuncType<T: Pointer> {
    pub id: NodeId,
    pub ret: Box<Type<T>>,
    pub args: Vec<Type<T>>,
}

#[derive(Clone)]
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
