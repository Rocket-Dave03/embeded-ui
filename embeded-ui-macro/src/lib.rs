use proc_macro::TokenStream;

#[proc_macro]
pub fn test_macro(input: TokenStream) -> TokenStream {
    input
}
