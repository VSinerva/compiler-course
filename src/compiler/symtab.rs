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
        SymTab {
            locals: HashMap::new(),
            parent: None,
        }
    }
}
