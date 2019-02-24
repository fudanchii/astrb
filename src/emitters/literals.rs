use super::Emitter;
use crate::ast;
use regex::Regex;

pub struct Literals<'l>(pub(crate) &'l ast::ValueVariants);

impl<'l> Emitter for Literals<'l> {
    fn emit(&self) -> String {
        match self.0 {
            ast::ValueVariants::Singleton(var) => Singleton(var).emit(),
            ast::ValueVariants::Integer(i) => i.0.to_string(),
            ast::ValueVariants::Float(f) => f.0.to_string(),
            ast::ValueVariants::Complex(c) => format!("{}i", c.0),
            ast::ValueVariants::Rational(r) => r.0.to_string(),
            ast::ValueVariants::String(s) => format!("\"{}\"", StringVal(s).emit()),
            ast::ValueVariants::Symbol(s) => format!(":{}", SymVal(s).emit()),
            ast::ValueVariants::HereDocument(hd) => HereDoc(hd).emit(),
            ast::ValueVariants::ExecuteString(s) => format!("`{}`", StringVal(s).emit()),
            ast::ValueVariants::RegularExpression(rgx) => RegularExpression(rgx).emit(),
            ast::ValueVariants::Array(arr) => ArrayVal(arr).emit(),
            ast::ValueVariants::Hash(h) => HashVal(h).emit(),
            ast::ValueVariants::Range(r) => RangeVal(r).emit(),
        }
    }
}

pub struct Singleton<'s>(pub(crate) &'s ast::SingletonVariants);

impl<'s> Emitter for Singleton<'s> {
    fn emit(&self) -> String {
        match self.0 {
            ast::SingletonVariants::False => "false".to_string(),
            ast::SingletonVariants::True => "true".to_string(),
            ast::SingletonVariants::Nil => "nil".to_string(),
        }
    }
}

pub struct StringVal<'s>(pub(crate) &'s ast::StringLiteral);

impl<'s> Emitter for StringVal<'s> {
    fn emit(&self) -> String {
        match self.0 {
            ast::StringLiteral::Static(s) => *s,
            ast::StringLiteral::WithInterpolation(v) => {
                v.iter().fold(String::new(), |buff, exp| {
                    format!("{}{}", buff, string_interpolate(exp))
                })
            }
        }
    }
}

use super::expression::Expression;

fn string_interpolate(exp: &ast::Expression) -> String {
    match exp {
        ast::Expression::Literal(l) => match l {
            ast::ValueVariants::String(s) => StringVal(s).emit(),
            _ => format!("#{{{}}}", Literals(l).emit()),
        },
        _ => format!("#{{{}}}", Expression(exp).emit()),
    }
}

pub struct SymVal<'sym>(pub(crate) &'sym ast::StringLiteral);

impl<'sym> Emitter for SymVal<'sym> {
    fn emit(&self) -> String {
        match self.0 {
            ast::StringLiteral::Static(s) => symbol_quote(s),
            ast::StringLiteral::WithInterpolation(v) => format!(
                "\"{}\"",
                v.iter().fold(String::new(), |buff, exp| {
                    format!("{}{}", buff, string_interpolate(exp))
                })
            ),
        }
    }
}

lazy_static! {
    static ref PROP_SYMBOL: Regex = Regex::new(r"[^0-9a-zA-Z]").unwrap();
}

fn symbol_quote(s: &str) -> String {
    if PROP_SYMBOL.is_match(s) {
        return format!("{{{}}}", s);
    }
    s.to_string()
}

pub struct HereDoc<'h>(pub(crate) &'h ast::HereDocumentVariants);

impl<'h> Emitter for HereDoc<'h> {
    fn emit(&self) -> String {
        match self.0 {
            ast::HereDocumentVariants::Plain(hd) => format!(
                "<<{}{}{}",
                hd.enclosure.0,
                StringVal(&hd.document).emit(),
                hd.enclosure.0
            ),
            ast::HereDocumentVariants::Dash(hd) => format!(
                "<<-{}\n{}\n{}",
                hd.enclosure.0,
                StringVal(&hd.document).emit(),
                hd.enclosure.0
            ),
            ast::HereDocumentVariants::Squiggly(hd) => format!(
                "<~{}\n{}\n{}",
                hd.enclosure.0,
                StringVal(&hd.document).emit(),
                hd.enclosure.0
            ),
        }
    }
}

pub struct RegularExpression<'r>(pub(crate) &'r ast::RegularExpression);

impl<'r> Emitter for RegularExpression<'r> {
    fn emit(&self) -> String {
        let flags = self.0.options.iter().fold(String::new(), |opts, fl| {
            format!(
                "{}{}",
                opts,
                match fl {
                    ast::RegularExpressionFlag::E => "e".to_string(),
                    ast::RegularExpressionFlag::I => "i".to_string(),
                    ast::RegularExpressionFlag::M => "m".to_string(),
                    ast::RegularExpressionFlag::N => "n".to_string(),
                    ast::RegularExpressionFlag::U => "u".to_string(),
                    ast::RegularExpressionFlag::X => "x".to_string(),
                }
            )
        });
        format!("/{}/{}", StringVal(&self.0.expression).emit(), flags)
    }
}

pub struct ArrayVal<'a>(pub(crate) &'a ast::ArrayLiteral);

impl<'a> Emitter for ArrayVal<'a> {
    fn emit(&self) -> String {
        match self.0 {
            ast::ArrayLiteral::Plain(vexp) => format!(
                "[{}]",
                vexp.iter()
                    .map(|exp| format!("{}", Expression(exp).emit()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ast::ArrayLiteral::Splat(aexp) => array_expression(aexp),
            ast::ArrayLiteral::WithInterpolation(vaip) => format!(
                "[{}]",
                vaip.iter()
                    .map(|exp| match exp {
                        ast::ArrayInterpolation::Expression(exp) => Expression(exp).emit(),
                        ast::ArrayInterpolation::Splat(aexp) => array_expression(aexp),
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

use super::access::Access;

fn array_expression(aexp: &ast::ArrayExpression) -> String {
    format!(
        "*{}",
        match aexp {
            ast::ArrayExpression::Access(av) => Access(av).emit(),
            ast::ArrayExpression::Literal(al) => ArrayVal(al).emit(),
        }
    )
}

pub struct HashVal<'h>(pub(crate) &'h ast::HashLiteral);

impl<'h> Emitter for HashVal<'h> {
    fn emit(&self) -> String {
        format!(
            "{{\n{}\n}}",
            match self.0 {
                ast::HashLiteral::Plain(vh) => vh
                    .iter()
                    .map(|elt| hash_element(elt))
                    .collect::<Vec<String>>()
                    .join(", "),
                ast::HashLiteral::Splat(sxp) => hash_expression(sxp),
                ast::HashLiteral::WithInterpolation(hwp) => hwp
                    .iter()
                    .map(|hint| {
                        match hint {
                            ast::HashInterpolation::Element(elt) => hash_element(elt),
                            ast::HashInterpolation::Splat(exp) => hash_expression(exp),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            }
        )
    }
}

fn hash_element(elt: &ast::HashElement) -> String {
    match elt {
        ast::HashElement::Pair(pelt) => format!(
            "{} => {}",
            Expression(&pelt.key).emit(),
            Expression(&pelt.value).emit()
        ),
        ast::HashElement::WithLabel(lelt) => format!(
            "{}: {}",
            StringVal(&lelt.key).emit(),
            Expression(&lelt.value).emit()
        ),
    }
}

fn hash_expression(exp: &ast::HashExpression) -> String {
    format!(
        "**{}",
        match exp {
            ast::HashExpression::Access(acc) => Access(acc).emit(),
            ast::HashExpression::Literal(hl) => HashVal(hl).emit(),
        }
    )
}

pub struct RangeVal<'r>(pub(crate) &'r ast::RangeLiteral);

impl<'r> Emitter for RangeVal<'r> {
    fn emit(&self) -> String {
        match self.0 {
            ast::RangeLiteral::Exclusive(i, si) => format!(
                "{}...{}",
                i.0,
                si.as_ref().map_or(String::new(), |int| int.0.to_string())
            ),
            ast::RangeLiteral::Inclusive(i, si) => format!(
                "{}..{}",
                i.0,
                si.as_ref().map_or(String::new(), |int| int.0.to_string())
            ),
        }
    }
}
