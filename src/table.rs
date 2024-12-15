// table.rs

/// A table structure that supports DAX (Data Analysis Expressions) operations.
///
/// # Examples
///
/// ```
/// use dax_rust::table::{Table, DaxResult};
/// use dax_rust::types::Value;
///
/// // Create a new table
/// let mut table = Table::new();
///
/// // Add some sales data
/// table.add_column(
///     "Sales".to_string(),
///     vec![
///         Value::Number(100.0),
///         Value::Number(200.0),
///         Value::Number(300.0)
///     ]
/// );
///
/// // Calculate the sum using DAX
/// match table.evaluate_dax("SUM([Sales])") {
///     DaxResult::Number(n) => println!("Total sales: {}", n),
///     DaxResult::Error(e) => println!("Error: {}", e),
///     _ => println!("Unexpected result type"),
/// }
/// ```
///
/// # Supported DAX Functions
///
/// Currently supports the following DAX functions:
/// - `SUM([column])`: Calculates the sum of numeric values in a column
/// - `AVERAGE([column])`: Calculates the average of numeric values in a column
///
/// # Error Handling
///
/// Returns `DaxResult::Error` in the following cases:
/// - Invalid column name
/// - Unsupported function
/// - Invalid DAX expression syntax
use crate::types::Value;
use dax_macro_impl::tokenize;
use dax_macro_impl::DaxToken;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Table {
    columns: HashMap<String, Vec<Value>>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            columns: HashMap::new(),
        }
    }

    pub fn get_column(&self, name: &str) -> Option<&Vec<Value>> {
        self.columns.get(name)
    }

    pub fn add_column(&mut self, name: String, values: Vec<Value>) {
        self.columns.insert(name, values);
    }

    /// Calculate sum of numeric values in a column, ignoring non-numeric values
    pub fn sum(&self, column_name: &str) -> Option<f64> {
        let column = self.get_column(column_name)?;

        let sum = column.iter().fold(0.0, |acc, value| {
            if let Value::Number(n) = value {
                acc + n
            } else {
                acc // Skip non-numeric values
            }
        });

        Some(sum) // Return Some even if sum is 0.0
    }

    /// Calculate average of numeric values in a column, ignoring non-numeric values
    pub fn average(&self, column_name: &str) -> Option<f64> {
        let column = self.get_column(column_name)?;
        let mut sum = 0.0;
        let mut count = 0;

        for value in column {
            if let Value::Number(n) = value {
                sum += n;
                count += 1;
            }
        }

        if count > 0 {
            Some(sum / count as f64)
        } else {
            Some(0.0) // Return Some(0.0) for empty or non-numeric columns
        }
    }

    pub fn count(&self, column_name: &str) -> Option<usize> {
        self.get_column(column_name).map(|column| column.len())
    }

    pub fn distinctcount(&self, column_name: &str) -> Option<usize> {
        self.get_column(column_name).map(|column| {
            let unique_values: std::collections::HashSet<&Value> = column.iter().collect();
            unique_values.len()
        })
    }

    // MIN function
    pub fn min(&self, column_name: &str) -> Option<f64> {
        self.get_column(column_name).and_then(|column| {
            column
                .iter()
                .filter_map(|value| {
                    if let Value::Number(n) = value {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        })
    }

    // MAX function
    pub fn max(&self, column_name: &str) -> Option<f64> {
        self.get_column(column_name).and_then(|column| {
            column
                .iter()
                .filter_map(|value| {
                    if let Value::Number(n) = value {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        })
    }

    fn evaluate_divide(&self, args: &[DaxToken]) -> Result<DaxResult, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err("DIVIDE requires 2 or 3 arguments".to_string());
        }

        // Evaluate numerator
        let numerator = match self.evaluate_dax(&args[0].to_string()) {
            DaxResult::Number(n) => n,
            _ => return Err("Numerator must be a number".to_string()),
        };

        // Evaluate denominator
        let denominator = match self.evaluate_dax(&args[1].to_string()) {
            DaxResult::Number(n) => n,
            _ => return Err("Denominator must be a number".to_string()),
        };

        // Handle division
        if denominator == 0.0 {
            // If there's an alternate result specified
            if args.len() == 3 {
                match self.evaluate_dax(&args[2].to_string()) {
                    DaxResult::Number(n) => Ok(DaxResult::Number(n)),
                    _ => Err("Alternate result must be a number".to_string()),
                }
            } else {
                // Return BLANK (represented as Error in this case)
                Err("Division by zero".to_string())
            }
        } else {
            Ok(DaxResult::Number(numerator / denominator))
        }
    }

    // DIVIDE function with optional alternate result
    pub fn divide(
        &self,
        numerator: f64,
        denominator: f64,
        alternate_result: Option<f64>,
    ) -> Option<f64> {
        if denominator == 0.0 {
            alternate_result
        } else {
            Some(numerator / denominator)
        }
    }

    // Updated evaluate_dax to handle the string literal requirement
    pub fn evaluate_dax(&self, expression: &str) -> DaxResult {
        // Use runtime tokenizer instead of proc macro
        let tokens = tokenize(expression);

        let mut iter = tokens.iter();
        while let Some(token) = iter.next() {
            match token {
                DaxToken::Function(name) => match name.as_str() {
                    "SUM" => {
                        while let Some(token) = iter.next() {
                            if let DaxToken::Column(col_name) = token {
                                return match self.sum(&col_name) {
                                    Some(sum) => DaxResult::Number(sum),
                                    None => DaxResult::Error(format!(
                                        "Could not calculate SUM for column {}",
                                        col_name
                                    )),
                                };
                            }
                        }
                    }
                    "AVERAGE" => {
                        while let Some(token) = iter.next() {
                            if let DaxToken::Column(col_name) = token {
                                return match self.average(&col_name) {
                                    Some(avg) => DaxResult::Number(avg),
                                    None => DaxResult::Error(format!(
                                        "Could not calculate AVERAGE for column {}",
                                        col_name
                                    )),
                                };
                            }
                        }
                    }
                    "MIN" => {
                        while let Some(token) = iter.next() {
                            if let DaxToken::Column(col_name) = token {
                                return match self.min(&col_name) {
                                    Some(min) => DaxResult::Number(min),
                                    None => DaxResult::Error(format!(
                                        "Could not calculate MIN for column {}",
                                        col_name
                                    )),
                                };
                            }
                        }
                    }
                    "MAX" => {
                        while let Some(token) = iter.next() {
                            if let DaxToken::Column(col_name) = token {
                                return match self.max(&col_name) {
                                    Some(max) => DaxResult::Number(max),
                                    None => DaxResult::Error(format!(
                                        "Could not calculate MAX for column {}",
                                        col_name
                                    )),
                                };
                            }
                        }
                    }
                    // "DIVIDE" => {
                    //     while let Some(token) = iter.next() {
                    //         if let DaxToken::Number(numerator) = token {
                    //             while let Some(token) = iter.next() {
                    //                 if let DaxToken::Number(denominator) = token {
                    //                     return match self.evaluate_divide(*numerator, *denominator, None) {
                    //                         Some(result) => DaxResult::Number(result),
                    //                         None => DaxResult::Error(format!(
                    //                             "Could not calculate DIVIDE for numerator {} and denominator {}",
                    //                             numerator, denominator
                    //                         )),
                    //                     };
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    "DISTINCTCOUNT" => {
                        while let Some(token) = iter.next() {
                            if let DaxToken::Column(col_name) = token {
                                return match self.distinctcount(&col_name) {
                                    Some(dc) => DaxResult::Number(dc as f64),
                                    None => DaxResult::Error(format!(
                                        "Could not calculate DISTINCTCOUNT for column {}",
                                        col_name
                                    )),
                                };
                            }
                        }
                    }
                    _ => return DaxResult::Error(format!("Unsupported function: {}", name)),
                },
                _ => continue,
            }
        }

        DaxResult::Error("Invalid or unsupported DAX expression".to_string())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

/// Display

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // First, calculate the maximum width for each column
        let mut column_widths: std::collections::HashMap<&String, usize> =
            std::collections::HashMap::new();

        // Initialize with column name lengths
        for column_name in self.columns.keys() {
            column_widths.insert(column_name, column_name.len());
        }

        // Update with maximum value lengths in each column
        for (column_name, values) in &self.columns {
            for value in values {
                let value_width = match value {
                    Value::Text(s) => s.len(),
                    Value::Number(n) => format!("{:.2}", n).len(),
                    Value::Boolean(b) => {
                        if b.to_owned() {
                            4
                        } else {
                            5
                        }
                    }
                    Value::Null => 0,
                    // Add other value types as needed
                };
                let current_max = column_widths.get(column_name).copied().unwrap_or(0);
                column_widths.insert(column_name, current_max.max(value_width));
            }
        }

        // Get sorted column names for consistent ordering
        let mut column_names: Vec<&String> = self.columns.keys().collect();
        column_names.sort();

        // Write header
        writeln!(
            f,
            "┌{}┐",
            "─".repeat(
                column_names
                    .iter()
                    .map(|name| column_widths[name] + 2)
                    .sum::<usize>()
                    + column_names.len()
                    - 1
            )
        )?;

        // Write column names
        for (i, column_name) in column_names.iter().enumerate() {
            if i > 0 {
                write!(f, "│")?;
            }
            write!(
                f,
                " {:<width$} ",
                column_name,
                width = column_widths[column_name]
            )?;
        }
        writeln!(f)?;

        // Write separator
        writeln!(
            f,
            "├{}┤",
            "─".repeat(
                column_names
                    .iter()
                    .map(|name| column_widths[name] + 2)
                    .sum::<usize>()
                    + column_names.len()
                    - 1
            )
        )?;

        // Write data rows
        let row_count = self.columns.values().next().map(|v| v.len()).unwrap_or(0);

        for row in 0..row_count {
            for (i, column_name) in column_names.iter().enumerate() {
                if i > 0 {
                    write!(f, "│")?;
                }
                if let Some(values) = self.columns.get(*column_name) {
                    if let Some(value) = values.get(row) {
                        match value {
                            Value::Text(s) => {
                                write!(f, " {:<width$} ", s, width = column_widths[column_name])?
                            }
                            Value::Number(n) => {
                                write!(f, " {:>width$.2} ", n, width = column_widths[column_name])?
                            }
                            Value::Boolean(b) => {
                                write!(f, " {:>width$} ", b, width = column_widths[column_name])?
                            }
                            Value::Null => {
                                write!(f, " {:>width$} ", "", width = column_widths[column_name])?
                            }
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        // Write bottom border
        writeln!(
            f,
            "└{}┘",
            "─".repeat(
                column_names
                    .iter()
                    .map(|name| column_widths[name] + 2)
                    .sum::<usize>()
                    + column_names.len()
                    - 1
            )
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum DaxResult {
    Number(f64),
    Text(String),
    Boolean(bool),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dax_sum() {
        let mut table = Table::new();
        table.add_column(
            "Sales".to_string(),
            vec![10.0.into(), 20.0.into(), 30.0.into()],
        );

        match table.evaluate_dax("SUM([Sales])") {
            DaxResult::Number(n) => assert_eq!(n, 60.0),
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_dax_average() {
        let mut table = Table::new();
        table.add_column(
            "Sales".to_string(),
            vec![10.0.into(), 20.0.into(), 30.0.into()],
        );

        match table.evaluate_dax("AVERAGE([Sales])") {
            DaxResult::Number(n) => assert_eq!(n, 20.0),
            _ => panic!("Expected number result"),
        }
    }
}
