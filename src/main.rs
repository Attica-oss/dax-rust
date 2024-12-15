use dax_macro::table;
use dax_rust::table::DaxResult;
use dax_rust::Table;
use dax_rust::Value;

#[derive(Debug)]
pub enum DaxValue {
    Number(f64),
    Text(String),
    Boolean(bool),
}

pub fn eval_dax(table: &Table, dax_expr: &str) -> Result<DaxValue, String> {
    match table.evaluate_dax(dax_expr) {
        DaxResult::Number(n) => Ok(DaxValue::Number(n)),
        DaxResult::Text(s) => Ok(DaxValue::Text(s)),
        DaxResult::Boolean(b) => Ok(DaxValue::Boolean(b)),
        DaxResult::Error(e) => Err(e),
    }
}

fn main() {
    let table = table! {
        
        "Sales" => [100.0, 150.0, 200.0],
        "Discount" => [0.0, 0.0, 0.0],
        "Quantity" => [10.0, 15.0, 15.0],
        "Product" => ["Apple", "Banana", "Orange"]
    };

    println!("{:?}", eval_dax(&table, "SUM([Quantity])"));
    println!("{:?}", eval_dax(&table, "AVERAGE([Quantity]"));
    println!("{}", &table)
}
