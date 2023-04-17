use melior_next::{dialect::Registry, ir::Module, utility::register_all_dialects, Context};
use mlir_asm::mlir_asm;

fn main() {
    let context = Context::new();

    let registry = Registry::new();
    register_all_dialects(&registry);
    context.append_dialect_registry(&registry);

    let module: Module = mlir_asm! { &context =>
        func.func @main() -> i32 {
            %0 = arith.constant 0 : i32
            return %0 : i32
        }
    };

    assert!(module.as_operation().verify());
    println!("{}", module.as_operation().debug_print());
}
