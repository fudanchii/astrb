use super::Emitter;
use crate::ast;

pub struct Access<'a>(pub(crate) &'a ast::AccessVariants);

impl<'a> Emitter for Access<'a> {
    fn emit(&self) -> String {
        match self.0 {
            ast::AccessVariants::ClassVariable(cv) => format!("@@{}", cv.0),
            ast::AccessVariants::Constant(c) => constant_variants(c),
            ast::AccessVariants::GlobalVariable(g) => global_variables(g),
            ast::AccessVariants::InstanceVariable(iv) => format!("@{}", iv.0),
            ast::AccessVariants::LocalVariable(v) => v.0,
            ast::AccessVariants::_Self => "self".to_string(),
        }
    }
}

fn constant_variants(c: &ast::ConstantVariants) -> String {
    match c {
        ast::ConstantVariants::Encoding => "__ENCODING__".to_string(),
        ast::ConstantVariants::File => "__FILE__".to_string(),
        ast::ConstantVariants::Line => "__LINE__".to_string(),
        ast::ConstantVariants::Scoped(vc) => vc
            .iter()
            .map(|cons| cons.0)
            .collect::<Vec<String>>()
            .join("::"),
        ast::ConstantVariants::TopLevel(tlc) => format!("::{}", tlc.0),
        ast::ConstantVariants::Unscoped(uc) => uc.0,
    }
}

fn global_variables(g: &ast::GlobalVariable) -> String {
    match g {
        ast::GlobalVariable::Ampersand => "$&".to_string(),
        ast::GlobalVariable::Aposthrope => "$'".to_string(),
        ast::GlobalVariable::AtSymbol => "$@".to_string(),
        ast::GlobalVariable::Backtick => "$`".to_string(),
        ast::GlobalVariable::Bang => "$!".to_string(),
        ast::GlobalVariable::Colon => "$:".to_string(),
        ast::GlobalVariable::Dollar => "$$".to_string(),
        ast::GlobalVariable::NthReference(i) => format!("${}", i.0),
        ast::GlobalVariable::Plain(pv) => format!("${}", pv.0),
        ast::GlobalVariable::Plus => "$+".to_string(),
        ast::GlobalVariable::QuestionMark => "$?".to_string(),
        ast::GlobalVariable::Splat => "$*".to_string(),
        ast::GlobalVariable::Tilde => "$~".to_string(),
    }
}
