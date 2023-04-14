#![deny(warnings)]

use proc_macro::TokenStream;

mod input;

#[proc_macro]
pub fn mlir_asm(_input: TokenStream) -> TokenStream {
    todo!()
}
