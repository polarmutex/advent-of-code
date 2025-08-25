use proc_macro2::Ident;
use syn::{spanned::Spanned, Error, ItemFn, ReturnType, Type};

use crate::{
    parser::{genargs::AocGeneratorArgs, solutionargs::AocSolutionArgs, solverargs::AocSolverArgs},
    partflag::AocPart,
};

#[derive(Debug, PartialEq, Eq)]
pub struct AocGeneratorData<'a> {
    pub display_slug: Ident,
    pub gen_type: &'a Type,
    pub source: &'a ItemFn,
}

impl<'a> AocGeneratorData<'a> {
    pub fn new(args: AocGeneratorArgs, source_fn: &'a ItemFn) -> syn::Result<AocGeneratorData<'a>> {
        let ReturnType::Type(_, ty_data) = &source_fn.sig.output else {
            let e = Error::new(
                source_fn.sig.output.span(),
                "Generators must have a return type that can be passed to a solver function.",
            );
            return Err(e);
        };
        Ok(AocGeneratorData {
            display_slug: args.display_slug,
            gen_type: ty_data.as_ref(),
            source: &source_fn,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AocSolverData<'a> {
    pub problem_part: AocPart,
    pub display_slug: Ident,
    pub input_type: &'a Type,
    pub source: &'a ItemFn,
    pub solution_type: &'a Type,
}

impl<'a> AocSolverData<'a> {
    pub fn new(args: AocSolverArgs, source_fn: &'a ItemFn) -> syn::Result<AocSolverData<'a>> {
        if source_fn.sig.inputs.len() != 1 {
            let e = Error::new(source_fn.sig.inputs.span(), "Solvers must accept exactly one argument, the data from the generator. This argument may be a tuple, struct, or other type.");
            return Err(e);
        } else {
            let Some(solve_type) = source_fn.sig.inputs.first() else {
                unreachable!("This should have been handled by previous error handling code.")
            };
            let syn::FnArg::Typed(solve_type) = solve_type else {
                let e = Error::new(solve_type.span(), "Solvers cannot be methods which take a self param");
                return Err(e);
            };
            let ReturnType::Type(_, solution_type_box) = &source_fn.sig.output else {
                let e = Error::new(source_fn.sig.output.span(), "Solvers must have a return type specified. Further, all solvers and solutions for a given part on a given day must return the same type.");
                return Err(e);
            };
            let solve_type = &solve_type.ty;
            return Ok(AocSolverData {
                problem_part: args.problem_part,
                display_slug: args.display_slug,
                input_type: solve_type.as_ref(),
                source: source_fn,
                solution_type: &solution_type_box,
            });
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AocSolutionData<'a> {
    pub problem_part: AocPart,
    pub display_slug: Ident,
    pub source: &'a ItemFn,
    pub solution_type: &'a Type,
}

impl<'a> AocSolutionData<'a> {
    pub fn new(args: AocSolutionArgs, source_fn: &'a ItemFn) -> syn::Result<AocSolutionData<'a>> {
        let ReturnType::Type(_, sol_type) = &source_fn.sig.output else {
            let e = Error::new(source_fn.sig.span(), "Solutions must specify a return type explicitly. Further, all solvers and solutions for a given part on a given day must return the same type.");
            return Err(e);
        };

        Ok(AocSolutionData {
            problem_part: args.problem_part,
            display_slug: args.display_slug,
            source: source_fn,
            solution_type: &sol_type,
        })
    }
}
