use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Token};

type Delimited<T> = Punctuated<T, Token![,]>;
type Vector = Delimited<Expr>;
type Matrix = Punctuated<Vector, Token![;]>;

struct Input {
    matrix: Matrix,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let matrix = Matrix::parse_terminated_with(input, Vector::parse_separated_nonempty)?;
        Ok(Self { matrix })
    }
}

impl Input {
    fn into_rows(self) -> Vec<Vec<Expr>> {
        self.matrix
            .into_iter()
            .map(|vector| vector.into_iter().collect())
            .collect()
    }
}

#[proc_macro]
pub fn matrix(input: TokenStream) -> TokenStream {
    let rows = parse_macro_input!(input as Input).into_rows();

    // Get the length of the first row, i.e. the number of columns
    let n = rows.first().map_or(0, Vec::len);

    // Transpose from row-major order to column-major order
    let columns: Delimited<_> = (0..n)
        .map(|column| {
            let column: Vector = rows
                .iter()
                .filter_map(|row| row.get(column))
                .cloned()
                .collect();
            quote! { [ #column ] }
        })
        .collect();

    TokenStream::from(quote! { [ #columns ] })
}
