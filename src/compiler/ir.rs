use std::{collections::HashMap, fmt};

use crate::compiler::{token::CodeLocation, variable::Type};

#[derive(PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct IrVar {
    pub name: String,
}

impl fmt::Debug for IrVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for IrVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl IrVar {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    pub fn new_global_types() -> HashMap<IrVar, Type> {
        use Type::*;
        HashMap::from([
            (IrVar::new("print_bool"), Func(vec![Bool], Box::new(Unit))),
            (IrVar::new("print_int"), Func(vec![Int], Box::new(Unit))),
            (IrVar::new("read_int"), Func(vec![], Box::new(Int))),
            (IrVar::new("+"), Func(vec![Int, Int], Box::new(Int))),
            (IrVar::new("*"), Func(vec![Int, Int], Box::new(Int))),
            (IrVar::new("-"), Func(vec![Int, Int], Box::new(Int))),
            (IrVar::new("/"), Func(vec![Int, Int], Box::new(Int))),
            (IrVar::new("%"), Func(vec![Int, Int], Box::new(Int))),
            (IrVar::new("<"), Func(vec![Int, Int], Box::new(Bool))),
            (IrVar::new("<="), Func(vec![Int, Int], Box::new(Bool))),
            (IrVar::new(">"), Func(vec![Int, Int], Box::new(Bool))),
            (IrVar::new(">="), Func(vec![Int, Int], Box::new(Bool))),
            (IrVar::new("=="), Func(vec![Int, Int], Box::new(Bool))), // Arg types faked
            (IrVar::new("!="), Func(vec![Int, Int], Box::new(Bool))), // Arg types faked
            (IrVar::new("unary_not"), Func(vec![Bool], Box::new(Bool))),
            (IrVar::new("unary_-"), Func(vec![Int], Box::new(Int))),
            (IrVar::new("or"), Func(vec![Bool, Bool], Box::new(Bool))),
            (IrVar::new("and"), Func(vec![Bool, Bool], Box::new(Bool))),
        ])
    }
}

impl fmt::Display for IrInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.instruction)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct IrInstruction {
    pub loc: CodeLocation,
    pub instruction: IrInstructionType,
}

impl IrInstruction {
    pub fn new(loc: CodeLocation, instruction: IrInstructionType) -> Self {
        Self { loc, instruction }
    }

    pub fn get_vars(&self) -> Vec<IrVar> {
        use IrInstructionType::*;
        match &self.instruction {
            LoadBoolConst(_, var) | LoadIntConst(_, var) | CondJump(var, _, _) => vec![var.clone()],
            Copy(var1, var2) => vec![var1.clone(), var2.clone()],
            Call(var1, var_vec, var2) => {
                let mut var_vec = var_vec.clone();
                var_vec.push(var1.clone());
                var_vec.push(var2.clone());
                var_vec
            }
            _ => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum IrInstructionType {
    LoadBoolConst(bool, IrVar),
    LoadIntConst(i64, IrVar),
    Copy(IrVar, IrVar),
    Call(IrVar, Vec<IrVar>, IrVar),
    Jump(Box<IrInstruction>),
    CondJump(IrVar, Box<IrInstruction>, Box<IrInstruction>),
    Label(String),
}

impl fmt::Display for IrInstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            IrInstructionType::LoadBoolConst(val, dest) => format!("LoadBoolConst({val}, {dest})"),
            IrInstructionType::LoadIntConst(val, dest) => format!("LoadIntConst({val}, {dest})"),
            IrInstructionType::Copy(src, dest) => format!("Copy({src}, {dest})"),
            IrInstructionType::Call(f, args, res) => format!("Call({f}, {args:?}, {res})"),
            IrInstructionType::Jump(dest) => format!("Jump({})", *dest),
            IrInstructionType::CondJump(cond, then_dest, else_dest) => {
                format!("CondJump({cond}, {then_dest}, {else_dest})")
            }
            IrInstructionType::Label(name) => format!("Label({name})"),
        };

        write!(f, "{}", string)
    }
}
