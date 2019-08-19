// FIXME: WIP
use ftl_source::Pointer;
use ftl_parser::ast::*;
use ftl_parser::visitor_mut::*;

pub struct FuncTypeDeduction;

impl FuncTypeDeduction {
    pub fn new() -> Self {
        Self{}
    }
}

impl<P: Pointer> MutPass<'_, P> for FuncTypeDeduction {

    fn visit_module(&mut self, node: &mut Module<P>) {
        let mut etd = ExprTypeDeduction{};
        etd.visit_module(node);
    }
}


struct ExprTypeDeduction;

impl ExprTypeDeduction {

    fn get_expr_type<P: Pointer>(expr: &Expr<P>) -> Option<Type<P>> {
        use ExprKind::*;        
        match expr.kind {
            Literal(ref lit) => Self::get_lit_type(lit),
            _ => None,
        }
    }

    fn get_lit_type<P: Pointer>(lit: &Lit<P>) -> Option<Type<P>> {
        use LitKind::*;
        Some(
            match lit.kind {
                Int(_) => Type {
                    // dummy id, we don't clone literals one 
                    // as someone might refer to it later.
                    id: 0,
                    kind: TypeKind::Literal(LitType::Int),
                    // cloning the span for error messages
                    span: lit.span.clone(),
                }
        })
    }

}

impl<P: Pointer> MutPass<'_, P> for ExprTypeDeduction {
    
    fn visit_expr(&mut self, node: &mut Expr<P>) {
        noop_expr(self, node);
        let ty = if let Some(ty) = Self::get_expr_type(node) {
            ty
        } else {
            return;
        };
        node.kind = ExprKind::Typed(Typed {
            // FIXME: Not a good idea tbh
            // better would be to create a new id
            // or set a dummy one.
            id: node.id,
            expr: Box::new(node.clone()),  
            ty,
        })
    }
}
