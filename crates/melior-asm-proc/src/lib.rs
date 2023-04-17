#![feature(proc_macro_diagnostic, proc_macro_span_shrink)]
#![deny(warnings)]

use self::{input::MacroInput, verify::verify_mlir};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

// mod codegen;
mod input;
mod verify;

#[proc_macro]
pub fn mlir_asm(input: TokenStream) -> TokenStream {
    // Parse macro input.
    let input = parse_macro_input!(input as MacroInput);

    // Verify MLIR code.
    verify_mlir(input.mlir_code.clone());

    // Generate output.
    let context = input.context;
    let mlir_code = input.mlir_code.span().source_text().unwrap();
    quote! {
        melior_next::ir::Module::parse(#context, #mlir_code)
            .expect("Invalid MLIR code in mlir_asm!() proc macro.")
    }
    .into()
}
