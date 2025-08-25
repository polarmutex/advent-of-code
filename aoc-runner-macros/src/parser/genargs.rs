use proc_macro2::Ident;
use syn::parse::Parse;

pub struct AocGeneratorArgs {
    pub display_slug: Ident,
}

impl Parse for AocGeneratorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let slug = input.parse::<Ident>()?;
        Ok(AocGeneratorArgs { display_slug: slug })
    }
}
