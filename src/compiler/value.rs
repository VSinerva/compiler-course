use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Value {
    Int(i64),
    Bool(bool),
    None(),
}

impl Value {
    pub fn and(&self, other: &Self) -> Self {
        if let Value::Bool(val1) = self {
            if let Value::Bool(val2) = other {
                Value::Bool(*val1 && *val2)
            } else {
                panic!("Can't apply and to non-bools!")
            }
        } else {
            panic!("Can't apply and to non-bools!!")
        }
    }

    pub fn or(&self, other: &Self) -> Self {
        if let Value::Bool(val1) = self {
            if let Value::Bool(val2) = other {
                Value::Bool(*val1 || *val2)
            } else {
                panic!("Can't apply or to non-bools!")
            }
        } else {
            panic!("Can't apply or to non-bools!!")
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::None() => write!(f, "<Unit>"),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if let Value::Int(val1) = self {
            if let Value::Int(val2) = other {
                Value::Int(val1 + val2)
            } else {
                panic!("Can't apply + to non-ints!")
            }
        } else {
            panic!("Can't apply + to non-ints!")
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let Value::Int(val1) = self {
            if let Value::Int(val2) = other {
                Value::Int(val1 * val2)
            } else {
                panic!("Can't apply * to non-ints!")
            }
        } else {
            panic!("Can't apply * to non-ints!")
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let Value::Int(val1) = self {
            if let Value::Int(val2) = other {
                Value::Int(val1 - val2)
            } else {
                panic!("Can't apply - to non-ints!")
            }
        } else {
            panic!("Can't apply - to non-ints!")
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let Value::Int(val1) = self {
            if let Value::Int(val2) = other {
                Value::Int(val1 / val2)
            } else {
                panic!("Can't apply / to non-ints!")
            }
        } else {
            panic!("Can't apply / to non-ints!")
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        if let Value::Int(val1) = self {
            if let Value::Int(val2) = other {
                Value::Int(val1 % val2)
            } else {
                panic!("Can't apply % to non-ints!")
            }
        } else {
            panic!("Can't apply % to non-ints!")
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Value::Int(val) = self {
            Value::Int(-val)
        } else {
            panic!("Can't apply - to non-ints!")
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        if let Value::Bool(val) = self {
            Value::Bool(!val)
        } else {
            panic!("Can't apply ! to non-bools!")
        }
    }
}
