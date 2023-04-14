#![deny(warnings)]

use crate::input::MacroInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

mod input;

#[proc_macro]
pub fn mlir_asm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    let context = input.context;
    let mlir_code = input.mlir_code.span().source_text().unwrap();
    quote! {
        melior_next::ir::Module::parse(#context, #mlir_code)
            .expect("Invalid MLIR code in mlir_asm!() proc macro.")
    }
    .into()
}
