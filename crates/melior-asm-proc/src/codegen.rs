#![allow(warnings)]

use melior_next::ir::{Location, Module, OperationRef};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub fn codegen_mlir(context: Expr, module: Module) -> TokenStream {
    quote! {{
        use melior_next::ir::{Location, Module};

        let module = Module::new(Location::unknown(#context));
    }}
}

fn codegen_operation(context: &Expr, operation: OperationRef) -> TokenStream {
    todo!()
}

#[cfg(test)]
mod test {
    use melior_next::{dialect::Registry, ir::Module, utility::register_all_dialects, Context};
    use quote::quote;

    #[test]
    fn test_abc() {
        let context = Context::new();

        let registry = Registry::new();
        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);

        let module = Module::parse(
            &context,
            // include_str!("../../../../double-dabble-mlir/print.mlir"),
            // "
            //     func.func @main() -> i32 {
            //         %0 = arith.constant 0 : i32
            //         return %0 : i32
            //     }
            // ",
            "
                arith.constant 0 : i32
            ",
        )
        .unwrap();

        dbg!(module.as_operation());
        todo!()
    }
}
