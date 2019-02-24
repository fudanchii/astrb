use crate::ast;

pub struct Expression<'e>(pub(crate) &'e ast::Expression);

impl<'e> super::Emitter for Expression<'e> {
    fn emit(&self) -> String {
        "".to_string()
    }
}
