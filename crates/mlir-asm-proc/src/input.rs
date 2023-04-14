use pm2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token,
};

pub struct MacroInput {
    /// The MLIR context to use.
    pub context: Ident,
    /// AST `->` token.
    pub rarrow: Token![->],
    /// The MLIR source code.
    pub mlir_code: TokenStream,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            context: input.parse()?,
            rarrow: input.parse()?,
            mlir_code: {
                let mlir_code = input.cursor().token_stream();

                // Skip everything until EOF.
                input.step(|cursor| {
                    let mut cursor = *cursor;
                    while let Some((_, next_cursor)) = cursor.token_tree() {
                        cursor = next_cursor;
                    }
                    Ok(((), cursor))
                })?;

                mlir_code
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[test]
    fn parse_macro_input() {
        let input: MacroInput = syn::parse2(quote! { ctx ->
            func.func @main() -> i32 {
                %0 = arith.constant 0 : i32
                return %0 : i32
            }
        })
        .unwrap();

        assert_eq!(input.context.to_string(), "ctx");
        assert_eq!(
            input.mlir_code.to_string(),
            quote! {
                func.func @main() -> i32 {
                    %0 = arith.constant 0 : i32
                    return %0 : i32
                }
            }
            .to_string()
        );
    }
}
