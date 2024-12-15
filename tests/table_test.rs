// tests/table_test.rs
use dax_macro_impl::DaxToken;
use dax_rust::table::{DaxResult, Table};
use dax_rust::types::Value;

use std::collections::HashSet;

#[test]
fn test_empty_table() {
    let table = Table::new();
    match table.evaluate_dax("SUM([Sales])") {
        DaxResult::Error(e) => assert!(e.contains("Could not calculate")),
        _ => panic!("Expected error for empty table"),
    }
}

#[test]
fn test_mixed_data_types() {
    let mut table = Table::new();
    table.add_column(
        "Mixed".to_string(),
        vec![
            Value::Number(100.0),
            Value::Text("Not a number".to_string()),
            Value::Number(300.0),
        ],
    );

    match table.evaluate_dax("SUM([Mixed])") {
        DaxResult::Number(n) => assert_eq!(n, 400.0), // Should only sum the numbers
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_large_numbers() {
    let mut table = Table::new();
    table.add_column(
        "Large".to_string(),
        vec![
            Value::Number(1_000_000.0),
            Value::Number(2_000_000.0),
            Value::Number(3_000_000.0),
        ],
    );

    match table.evaluate_dax("SUM([Large])") {
        DaxResult::Number(n) => assert_eq!(n, 6_000_000.0),
        _ => panic!("Expected number result for large numbers"),
    }
}

#[test]
fn test_zero_values() {
    let mut table = Table::new();
    table.add_column(
        "Zeros".to_string(),
        vec![Value::Number(0.0), Value::Number(0.0), Value::Number(0.0)],
    );

    match table.evaluate_dax("AVERAGE([Zeros])") {
        DaxResult::Number(n) => assert_eq!(n, 0.0),
        _ => panic!("Expected zero average"),
    }
}

#[test]
fn test_single_value() {
    let mut table = Table::new();
    table.add_column("Single".to_string(), vec![Value::Number(42.0)]);

    match table.evaluate_dax("AVERAGE([Single])") {
        DaxResult::Number(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected single value average"),
    }
}

#[test]
fn test_distinct_count_with_special_floats() {
    let mut table = Table::new();
    let values = vec![
        Value::Number(f64::NAN),
        Value::Number(f64::NAN), // NANs should be considered equal
        Value::Number(f64::INFINITY),
        Value::Number(f64::NEG_INFINITY),
    ];
    table.add_column("special".to_string(), values);

    assert_eq!(table.distinctcount("special"), Some(3));
}

#[test]
fn test_nan_equality() {
    let nan1 = Value::Number(f64::NAN);
    let nan2 = Value::Number(f64::NAN);
    assert_eq!(nan1, nan2); // Should be equal now

    let mut set = HashSet::new();
    set.insert(&nan1);
    set.insert(&nan2);
    assert_eq!(set.len(), 1); // Should only count as one distinct value
}

#[test]
fn test_distinct_count_with_floats() {
    let mut table = Table::new();
    let values = vec![
        Value::Number(1.0),
        Value::Number(1.0), // Duplicate
        Value::Number(2.5),
        Value::Number(3.7),
        Value::Number(3.7), // Duplicate
    ];
    table.add_column("test".to_string(), values);

    assert_eq!(table.distinctcount("test"), Some(3));
}

#[test]
fn test_invalid_function() {
    let mut table = Table::new();
    table.add_column(
        "Sales".to_string(),
        vec![Value::Number(100.0), Value::Number(200.0)],
    );

    match table.evaluate_dax("INVALID([Sales])") {
        DaxResult::Error(e) => assert!(e.contains("Unsupported")),
        _ => panic!("Expected error for invalid function"),
    }
}

#[test]
fn test_tokenization() {
    let tokens = dax_macro::parse_dax!("SUM([Sales]) + AVERAGE([Quantity])");

    let mut iter = tokens.iter();
    assert!(iter.any(|t| matches!(t, DaxToken::Function(name) if name == "SUM")));
    assert!(iter.any(|t| matches!(t, DaxToken::Column(name) if name == "Sales")));
    assert!(iter.any(|t| matches!(t, DaxToken::Operator(op) if *op == '+')));
    assert!(iter.any(|t| matches!(t, DaxToken::Function(name) if name == "AVERAGE")));
    assert!(iter.any(|t| matches!(t, DaxToken::Column(name) if name == "Quantity")));
}

#[test]
fn test_basic_table_operations() {
    let mut table = Table::new();

    // Add columns
    table.add_column(
        "Sales".to_string(),
        vec![
            Value::Number(100.0),
            Value::Number(200.0),
            Value::Number(300.0),
        ],
    );
    table.add_column(
        "Quantity".to_string(),
        vec![
            Value::Number(10.0),
            Value::Number(20.0),
            Value::Number(30.0),
        ],
    );

    // Test SUM
    let tokens = dax_macro::parse_dax!("SUM([Sales])");
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Function(name) if name == "SUM")));
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Column(name) if name == "Sales")));

    // Test AVERAGE
    let tokens = dax_macro::parse_dax!("AVERAGE([Quantity])");
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Function(name) if name == "AVERAGE")));
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Column(name) if name == "Quantity")));

    // Test complex expression
    let tokens = dax_macro::parse_dax!("SUM([Sales]) / SUM([Quantity])");
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Column(name) if name == "Sales")));
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Column(name) if name == "Quantity")));
    assert!(tokens
        .iter()
        .any(|t| matches!(t, DaxToken::Operator(op) if *op == '/')));
}

#[test]
fn test_dax_evaluation() {
    let mut table = Table::new();
    table.add_column(
        "Sales".to_string(),
        vec![
            Value::Number(100.0),
            Value::Number(200.0),
            Value::Number(300.0),
        ],
    );

    match table.evaluate_dax("SUM([Sales])") {
        DaxResult::Number(n) => assert_eq!(n, 600.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_dax_average() {
    let mut table = Table::new();
    table.add_column(
        "Sales".to_string(),
        vec![
            Value::Number(100.0),
            Value::Number(200.0),
            Value::Number(300.0),
        ],
    );

    match table.evaluate_dax("AVERAGE([Sales])") {
        DaxResult::Number(n) => assert_eq!(n, 200.0),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn test_invalid_column() {
    let mut table = Table::new();
    table.add_column(
        "Sales".to_string(),
        vec![
            Value::Number(100.0),
            Value::Number(200.0),
            Value::Number(300.0),
        ],
    );

    match table.evaluate_dax("SUM([NonExistent])") {
        DaxResult::Error(_) => (),
        _ => panic!("Expected error for non-existent column"),
    }
}
