use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Type {
    Int,
    Bool,
    Func(Vec<Type>, Box<Type>),
    Unit,
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Func(fn(&[Value]) -> Value),
    None(),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::Func(_) => write!(f, "<FunctionCall>"),
            Value::None() => write!(f, "<Unit>"),
        }
    }
}

impl Value {
    pub fn add(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);

        let Value::Int(lhs) = args[0] else {
            panic!("Can't apply + to non-ints!")
        };
        let Value::Int(rhs) = args[1] else {
            panic!("Can't apply + to non-ints!")
        };

        Value::Int(lhs + rhs)
    }

    pub fn mul(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);

        let Value::Int(lhs) = args[0] else {
            panic!("Can't apply * to non-ints!")
        };
        let Value::Int(rhs) = args[1] else {
            panic!("Can't apply * to non-ints!")
        };

        Value::Int(lhs * rhs)
    }

    pub fn sub(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);

        let Value::Int(lhs) = args[0] else {
            panic!("Can't apply - to non-ints!")
        };
        let Value::Int(rhs) = args[1] else {
            panic!("Can't apply - to non-ints!")
        };

        Value::Int(lhs - rhs)
    }

    pub fn div(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);

        let Value::Int(lhs) = args[0] else {
            panic!("Can't apply / to non-ints!")
        };
        let Value::Int(rhs) = args[1] else {
            panic!("Can't apply / to non-ints!")
        };

        Value::Int(lhs / rhs)
    }

    pub fn rem(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);

        let Value::Int(lhs) = args[0] else {
            panic!("Can't apply % to non-ints!")
        };
        let Value::Int(rhs) = args[1] else {
            panic!("Can't apply % to non-ints!")
        };

        Value::Int(lhs / rhs)
    }

    pub fn eq(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] == args[1])
    }

    pub fn neq(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] != args[1])
    }

    pub fn lt(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] < args[1])
    }

    pub fn le(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] <= args[1])
    }

    pub fn gt(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] > args[1])
    }

    pub fn ge(args: &[Self]) -> Self {
        assert_eq!(args.len(), 2);
        Value::Bool(args[0] >= args[1])
    }

    pub fn not(args: &[Self]) -> Self {
        assert_eq!(args.len(), 1);

        let Value::Bool(val) = args[0] else {
            panic!("Can't apply 'not' to non-bools!")
        };

        Value::Bool(!val)
    }

    pub fn neg(args: &[Self]) -> Self {
        assert_eq!(args.len(), 1);

        let Value::Int(val) = args[0] else {
            panic!("Can't apply negation to non-ints!")
        };

        Value::Int(-val)
    }
}
