mod atom_print;
mod errors;
mod extract;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Printer, attributes(print_comp))]
pub fn create_print(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    atom_print::PrinterStruct::try_from(&ast)
        .map(|st| st.create_print())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ImplMp4AtomPrint, attributes(print_comp))]
pub fn create_mp4_print(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    atom_print::PrinterStruct::try_from(&ast)
        .map(|st| st.create_mp4_print())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
