use proc_macro::TokenStream;

#[proc_macro_derive(AtomDisplay, attributes(print_atom))]
pub fn atom_display(_input: TokenStream) -> TokenStream {
    // let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
