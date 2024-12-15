// types.rs
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Text(String),
    Boolean(bool),
    Null,
}

// Custom PartialEq implementation that handles NaN values
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if a.is_nan() && b.is_nan() {
                    true // Consider all NaN values equal
                } else {
                    a == b
                }
            }
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

// Custom Eq implementation that handles floating point numbers
impl Eq for Value {}

// Custom Hash implementation for Value that handles floating point numbers
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Number(n) => {
                // Handle NaN and infinite values
                if n.is_nan() {
                    state.write_u8(0);
                } else if n.is_infinite() {
                    if n.is_sign_positive() {
                        state.write_u8(1);
                    } else {
                        state.write_u8(2);
                    }
                } else {
                    // Convert to bits for consistent hashing
                    state.write_u64(n.to_bits());
                }
            }
            Value::Text(s) => s.hash(state),
            Value::Boolean(b) => b.hash(state),
            Value::Null => state.write_u8(3),
        }
    }
}
