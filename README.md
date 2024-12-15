# rust-dax

A Rust library for parsing and evaluating DAX (Data Analysis Expressions) formulas using procedural macros. This library provides compile-time parsing of DAX expressions and basic evaluation capabilities.

## Features

- Compile-time parsing of DAX expressions
- Support for basic DAX functions (SUM, AVERAGE, COUNT)
- Column reference parsing using square bracket notation
- Basic arithmetic operations
- Table creation macro for testing and development

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-dax = "0.1.0"
```

## Usage

### Parsing DAX Expressions

Use the `parse_dax!` macro to parse DAX expressions at compile time:

```rust
use rust_dax::parse_dax;

fn main() {
    let tokens = parse_dax!("SUM([Sales])");
    println!("{:?}", tokens);
}
```

### Creating Test Tables

The library provides a `table!` macro for creating test data tables:

```rust
use rust_dax::table;

let sales_table = table! {
    "Amount" => [100.0, 200.0, 300.0],
    "Quantity" => [1, 2, 3],
};
```

### Supported DAX Functions

Currently supported functions include:
- `SUM`: Calculate the sum of a column
- `AVERAGE`: Calculate the average of a column
- `COUNT`: Count non-empty values in a column

Example:
```rust
let result = parse_dax!("SUM([Amount])");
```

## Implementation Details

The library uses Rust's procedural macro system to parse DAX expressions at compile time. The parsing process includes:

1. Tokenization of DAX expressions into discrete tokens (functions, columns, operators, etc.)
2. Basic syntax validation
3. Generation of corresponding Rust code

## Project Structure

- `src/lib.rs`: Main library interface and proc macro definitions
- `dax-macro-impl/`: Implementation details for the procedural macros
- `dax-macro/`: Public macro interfaces

## Limitations

Current limitations include:
- Limited set of supported DAX functions
- Basic arithmetic operations only
- No support for complex filtering or relationships
- Limited error handling

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

When contributing, please:
1. Add tests for any new features
2. Update documentation
3. Follow the existing code style

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Rust proc-macro2 and syn crates for making this implementation possible
- Microsoft's DAX documentation for reference implementation details
