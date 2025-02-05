use crate::compiler::variable::{Type, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct SymTab<'source, T> {
    tables: Vec<HashMap<&'source str, T>>,
}

impl<'source, T> SymTab<'source, T> {
    pub fn get(&mut self, symbol: &str) -> &mut T {
        for i in (0..self.tables.len()).rev() {
            if self.tables[i].contains_key(symbol) {
                return self.tables[i].get_mut(symbol).unwrap();
            }
        }
        panic!("No symbol {} found!", symbol);
    }

    pub fn push_level(&mut self) {
        self.tables.push(HashMap::new());
    }

    pub fn remove_level(&mut self) {
        self.tables.pop();
    }

    pub fn insert(&mut self, name: &'source str, val: T) {
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

impl<'source, T> SymTab<'source, T> {
    pub fn new() -> SymTab<'source, T> {
        SymTab {
            tables: vec![HashMap::new()],
        }
    }
}

impl<'source> SymTab<'source, Type> {
    pub fn new_type_table() -> SymTab<'source, Type> {
        use Type::*;
        let globals = HashMap::from([
            ("+", Func(vec![Int, Int], Box::new(Int))),
            ("*", Func(vec![Int, Int], Box::new(Int))),
            ("-", Func(vec![Int, Int], Box::new(Int))),
            ("/", Func(vec![Int, Int], Box::new(Int))),
            ("%", Func(vec![Int, Int], Box::new(Int))),
            ("<", Func(vec![Int, Int], Box::new(Bool))),
            ("<=", Func(vec![Int, Int], Box::new(Bool))),
            (">", Func(vec![Int, Int], Box::new(Bool))),
            (">=", Func(vec![Int, Int], Box::new(Bool))),
            ("not", Func(vec![Bool], Box::new(Bool))),
            ("neg", Func(vec![Int], Box::new(Int))),
            ("or", Func(vec![Bool, Bool], Box::new(Bool))),
            ("and", Func(vec![Bool, Bool], Box::new(Bool))),
        ]);

        SymTab {
            tables: vec![globals],
        }
    }
}

impl<'source> SymTab<'source, Value> {
    pub fn new_val_table() -> SymTab<'source, Value> {
        use Value::*;
        let globals = HashMap::from([
            ("+", Func(Value::add)),
            ("*", Func(Value::mul)),
            ("-", Func(Value::sub)),
            ("/", Func(Value::div)),
            ("%", Func(Value::rem)),
            ("==", Func(Value::eq)),
            ("!=", Func(Value::neq)),
            ("<", Func(Value::lt)),
            ("<=", Func(Value::le)),
            (">", Func(Value::gt)),
            (">=", Func(Value::ge)),
            ("not", Func(Value::not)),
            ("neg", Func(Value::neg)),
        ]);

        SymTab {
            tables: vec![globals],
        }
    }
}
