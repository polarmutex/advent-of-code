use syn::{parse::Parse, token::Comma, Expr, Token};

#[derive(Debug, PartialEq, Eq)]
pub struct AocCaseArgs {
    pub expected_p1: Expr,
    pub expected_p2: Option<Expr>,
}

impl Parse for AocCaseArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let p1: Expr = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![,]) {
            let _: Comma = input.parse()?;
            let p2: Expr = input.parse()?;
            Ok(AocCaseArgs {
                expected_p1: p1,
                expected_p2: Some(p2),
            })
        } else {
            if !input.is_empty() {
                Err(input.error("Expected: a single expression for just testing Part 1, or two expressions as two arguments if testing Part 1 and Part 2."))
            } else {
                Ok(AocCaseArgs {
                    expected_p1: p1,
                    expected_p2: None,
                })
            }
        }
    }
}
