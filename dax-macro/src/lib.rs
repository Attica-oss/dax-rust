// dax_macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// use syn::parse::Parser;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Result, Token,
};

#[proc_macro]
pub fn parse_dax(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);
    let dax_str = input.value();

    // Use the implementation from dax_macro_impl
    let tokens = dax_macro_impl::tokenize(&dax_str);

    let expanded = quote! {{
        use dax_macro_impl::DaxToken;
        vec![#(#tokens),*]
    }};

    TokenStream::from(expanded)
}

struct TableData {
    columns: Vec<ColumnDef>,
}

struct ColumnDef {
    name: syn::LitStr,
    values: Vec<syn::Expr>,
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

#[proc_macro]
pub fn table(input: TokenStream) -> TokenStream {
    let table_data = parse_macro_input!(input as TableData);

    let column_names = table_data.columns.iter().map(|col| &col.name);
    let column_values = table_data.columns.iter().map(|col| &col.values);

    let expanded = quote! {
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
    };

    expanded.into()
}
