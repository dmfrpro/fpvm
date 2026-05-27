use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Int(i64),
    Real(f64),
    Bool(bool),
    Atom(String),
    List(Vec<Value>),
    Func(String), // reference to a bytecode function (no captures)
    Closure { label: String, captures: Vec<Value> },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(v) => write!(f, "{}", v),
            Value::Real(v) => {
                if v.fract() == 0.0 {
                    write!(f, "{:.1}", v)
                } else {
                    write!(f, "{}", v)
                }
            }
            Value::Bool(v) => write!(f, "{}", v),
            Value::Atom(v) => write!(f, "'{}", v),
            Value::List(v) => {
                write!(f, "'(")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Func(label) => write!(f, "'<func {}>", label),
            Value::Closure { label, .. } => write!(f, "'<closure {}>", label),
        }
    }
}
