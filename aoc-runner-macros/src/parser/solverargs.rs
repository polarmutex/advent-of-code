use proc_macro2::Ident;
use syn::{parse::Parse, token::Comma};

use crate::partflag::AocPart;

#[derive(Debug, PartialEq, Eq)]
pub struct AocSolverArgs {
    pub problem_part: AocPart,
    pub display_slug: Ident,
}

impl Parse for AocSolverArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let part: AocPart = input.parse()?;
        input.parse::<Comma>()?;
        let slug: Ident = input.parse()?;
        Ok(AocSolverArgs {
            problem_part: part,
            display_slug: slug,
        })
    }
}
