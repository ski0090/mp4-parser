mod atom_print;
mod errors;
mod extract;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Printer, attributes(print_comp))]
pub fn atom_display(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    atom_print::AtiomPrintSt::try_from(&ast)
        .map(|st| st.emit())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
