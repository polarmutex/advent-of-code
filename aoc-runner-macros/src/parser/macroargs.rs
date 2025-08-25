use proc_macro2::Ident;
use syn::{parse::Parse, token::Comma, Lit};

#[derive(Debug, PartialEq, Eq)]
pub struct AocMacroArgs {
    pub year_num: u32,
    pub day_num: u32,
}

impl Parse for AocMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year_lit: Lit = input.parse()?;
        input.parse::<Comma>()?;
        let day_ident: Ident = input.parse()?;

        let day_part = day_ident.to_string();
        let day_part = day_part.strip_prefix("day").unwrap_or(&day_part);
        let day_part = day_part.strip_prefix("d").unwrap_or(&day_part);
        let day_num: u32 = day_part.parse().or_else(|a| {
            let msg = format!("Could not parse number from day indicator. Parsing error:\n{}", a);
            let e = syn::Error::new(day_ident.span(), msg);
            return Err(e);
        })?;

        let year_num = if let Lit::Int(li) = year_lit {
            li.base10_parse()?
        } else {
            let e = syn::Error::new(
                year_lit.span(),
                "First argument to `aoc` macro must be the puzzle's year as a number.",
            );
            return Err(e);
        };

        if day_num < 1 || day_num > 25 {
            let e = syn::Error::new(day_ident.span(), "Day number is out of range of 1-25");
            return Err(e);
        }

        Ok(AocMacroArgs { year_num, day_num })
    }
}
