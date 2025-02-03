use crate::compiler::value::Value;
use std::collections::HashMap;

#[derive(Default)]
pub struct SymTab<'source> {
    pub locals: HashMap<&'source str, Value>,
    pub parent: Option<Box<SymTab<'source>>>,
}

impl<'source> SymTab<'source> {
    pub fn get(&mut self, symbol: &str) -> &mut Value {
        if let Some(val) = self.locals.get_mut(symbol) {
            val
        } else if let Some(parent) = &mut self.parent {
            parent.get(symbol)
        } else {
            panic!("No symbol {} found!", symbol);
        }
    }

    pub fn new_global() -> SymTab<'source> {
        let locals = HashMap::from([
            ("+", Value::Func(Value::add)),
            ("*", Value::Func(Value::mul)),
            ("-", Value::Func(Value::sub)),
            ("/", Value::Func(Value::div)),
            ("%", Value::Func(Value::rem)),
            ("==", Value::Func(Value::eq)),
            ("!=", Value::Func(Value::neq)),
            ("<", Value::Func(Value::lt)),
            ("<=", Value::Func(Value::le)),
            (">", Value::Func(Value::gt)),
            (">=", Value::Func(Value::ge)),
            ("not", Value::Func(Value::not)),
            ("neg", Value::Func(Value::neg)),
        ]);

        SymTab {
            locals,
            parent: None,
        }
    }
}
