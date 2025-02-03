use crate::compiler::value::Value;
use std::collections::HashMap;

#[derive(Default)]
pub struct SymTab<'source> {
    tables: Vec<HashMap<&'source str, Value>>,
}

impl<'source> SymTab<'source> {
    pub fn get(&mut self, symbol: &str) -> &mut Value {
        for i in (0..self.tables.len()).rev() {
            if self.tables[i].contains_key(symbol) {
                return self.tables[i].get_mut(symbol).unwrap();
            }
        }
        panic!("No symbol {} found!", symbol);
    }

    pub fn new() -> SymTab<'source> {
        let globals = HashMap::from([
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
            tables: vec![globals],
        }
    }

    pub fn push_level(&mut self) {
        self.tables.push(HashMap::new());
    }

    pub fn remove_level(&mut self) {
        self.tables.pop();
    }

    pub fn insert(&mut self, name: &'source str, val: Value) {
        if self
            .tables
            .last_mut()
            .expect("Symbols table should never be empty!")
            .insert(name, val)
            .is_some()
        {
            panic!("Variable {} already defined in this scope!", name)
        }
    }
}
