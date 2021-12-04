use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _st = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
