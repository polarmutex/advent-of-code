use proc_macro2::Ident;
use syn::parse::Parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AocPart {
    Part1,
    Part2,
}

impl Parse for AocPart {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "part1" => Ok(AocPart::Part1),
            "Part1" => Ok(AocPart::Part1),
            "p1" => Ok(AocPart::Part1),
            "P1" => Ok(AocPart::Part1),
            "part2" => Ok(AocPart::Part2),
            "Part2" => Ok(AocPart::Part2),
            "p2" => Ok(AocPart::Part2),
            "P2" => Ok(AocPart::Part2),
            _ => Err(input.error("Expected a Part 1 / Part 2 indicator, such as `part1` or `part2`.")),
        }
    }
}
