#![deny(warnings)]

// Rename crates for accessibility.
extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

mod input;

#[proc_macro]
pub fn mlir_asm(_input: pm::TokenStream) -> pm::TokenStream {
    todo!()
}
