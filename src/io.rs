use crate::{DaxError, Table, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_csv(path: &Path) -> Result<Table, DaxError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut table = Table::new();

    let mut headers: Vec<String> = Vec::new();
    let mut columns: Vec<Vec<Value>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let values: Vec<&str> = line.split(',').collect();

        if i == 0 {
            headers = values.into_iter().map(String::from).collect();
            columns = vec![Vec::new(); headers.len()];
        } else {
            for (j, value) in values.iter().enumerate() {
                if j < columns.len() {
                    columns[j].push(parse_value(value));
                }
            }
        }
    }

    for (header, column) in headers.into_iter().zip(columns.into_iter()) {
        table.add_column(header, column);
    }

    Ok(table)
}

fn parse_value(value: &str) -> Value {
    if let Ok(num) = value.parse::<f64>() {
        Value::Number(num)
    } else if value.eq_ignore_ascii_case("true") {
        Value::Boolean(true)
    } else if value.eq_ignore_ascii_case("false") {
        Value::Boolean(false)
    } else if value.is_empty() {
        Value::Null
    } else {
        Value::Text(value.to_string())
    }
}
