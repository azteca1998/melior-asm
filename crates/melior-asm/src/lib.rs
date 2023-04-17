#[doc(hidden)]
pub use melior_asm_proc::mlir_asm as mlir_asm_impl;
#[doc(hidden)]
pub use melior_next;

#[macro_export]
macro_rules! mlir_asm {
    ( $( $inner:tt )* ) => {{
        use $crate::{melior_next as melior_next, mlir_asm_impl};

        // Forward to the procedural macro.
        mlir_asm_impl!( $( $inner )* )
    }};
}
