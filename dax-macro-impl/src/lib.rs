// dax-macro-impl/src/lib.rs
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::fmt;
use std::str::FromStr;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Result, Token,
};

#[derive(Debug)]
pub enum DaxToken {
    Function(String),
    Number(f64),
    Operator(char),
    Column(String),
    Comma,
    ParenOpen,
    ParenClose,
    Whitespace,
}

impl fmt::Display for DaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaxToken::Function(name) => write!(f, "{}", name),
            DaxToken::Number(n) => write!(f, "{}", n),
            DaxToken::Operator(op) => write!(f, "{}", op),
            DaxToken::Column(name) => write!(f, "{}", name),
            DaxToken::Comma => write!(f, ","),
            DaxToken::ParenOpen => write!(f, "("),
            DaxToken::ParenClose => write!(f, ")"),
            DaxToken::Whitespace => write!(f, " "),
        }
    }
}

impl ToTokens for DaxToken {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let token_str = match self {
            DaxToken::Function(name) => format!("DaxToken::Function(\"{}\".to_string())", name),
            DaxToken::Number(n) => format!("DaxToken::Number({:?})", n),
            DaxToken::Operator(op) => format!("DaxToken::Operator('{}')", op),
            DaxToken::Column(name) => format!("DaxToken::Column(\"{}\".to_string())", name),
            DaxToken::Comma => "DaxToken::Comma".to_string(),
            DaxToken::ParenOpen => "DaxToken::ParenOpen".to_string(),
            DaxToken::ParenClose => "DaxToken::ParenClose".to_string(),
            DaxToken::Whitespace => "DaxToken::Whitespace".to_string(),
        };
        tokens.extend(TokenStream2::from_str(&token_str).unwrap());
    }
}

pub fn tokenize(input: &str) -> Vec<DaxToken> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = num.parse() {
                    tokens.push(DaxToken::Number(n));
                }
            }
            '[' => {
                chars.next();
                let mut column = String::new();
                while let Some(&c) = chars.peek() {
                    if c != ']' {
                        column.push(c);
                        chars.next();
                    } else {
                        chars.next();
                        break;
                    }
                }
                tokens.push(DaxToken::Column(column));
            }
            '(' => {
                chars.next();
                tokens.push(DaxToken::ParenOpen);
            }
            ')' => {
                chars.next();
                tokens.push(DaxToken::ParenClose);
            }
            ',' => {
                chars.next();
                tokens.push(DaxToken::Comma);
            }
            '+' | '-' | '*' | '/' => {
                chars.next();
                tokens.push(DaxToken::Operator(c));
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
                tokens.push(DaxToken::Whitespace);
            }
            'A'..='Z' | 'a'..='z' => {
                let mut function = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        function.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(DaxToken::Function(function));
            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}

// Table-related structures
pub struct TableData {
    pub columns: Vec<ColumnDef>,
}

pub struct ColumnDef {
    pub name: syn::LitStr,
    pub values: Vec<syn::Expr>,
}

impl Parse for TableData {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut columns = Vec::new();

        while !input.is_empty() {
            let name = input.parse::<syn::LitStr>()?;
            input.parse::<Token![=>]>()?;

            let content;
            bracketed!(content in input);
            let values = content.parse_terminated(syn::Expr::parse, Token![,])?;

            columns.push(ColumnDef {
                name,
                values: values.into_iter().collect(),
            });

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(TableData { columns })
    }
}

pub fn generate_table_tokens(table_data: &TableData) -> TokenStream2 {
    let column_names = table_data.columns.iter().map(|col| &col.name);
    let column_values = table_data.columns.iter().map(|col| &col.values);

    quote! {
        {
            let mut table = Table::new();
            #(
                table.add_column(
                    #column_names.to_string(),
                    vec![#(Value::from(#column_values)),*]
                );
            )*
            table
        }
    }
}
