use std::collections::HashMap;

use syn::{Item, ItemMod, Type};

use crate::{
    domain::{AocGeneratorData, AocSolutionData, AocSolverData},
    parser::{genargs::AocGeneratorArgs, solutionargs::AocSolutionArgs, solverargs::AocSolverArgs},
    partflag::AocPart,
};

const SOLUTION_TYPE_MISMATCH: &str = "All solvers and solutions for a given day and part must return the same type. This solver does not match the first solution type detected.";

pub struct AocSolutionsAggregation<'a> {
    pub solutions_p1: Vec<AocSolutionData<'a>>,
    pub solutions_p2: Vec<AocSolutionData<'a>>,
    pub generators: HashMap<&'a Type, Vec<AocGeneratorData<'a>>>,
    pub solvers_p1: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
    pub solvers_p2: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
    pub p1_result_type: Option<&'a Type>,
    pub p2_result_type: Option<&'a Type>,
}

impl<'a> AocSolutionsAggregation<'a> {
    pub fn new() -> Self {
        AocSolutionsAggregation {
            solutions_p1: Vec::new(),
            solutions_p2: Vec::new(),
            generators: HashMap::new(),
            solvers_p1: HashMap::new(),
            solvers_p2: HashMap::new(),
            p1_result_type: None,
            p2_result_type: None,
        }
    }

    pub fn p1_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p1.iter()
    }

    pub fn p2_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p2.iter()
    }

    pub fn p1_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p1.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter()
                    .flat_map(|g| self.solvers_p1.get(ty).unwrap().iter().map(move |s| (g, s)))
                    .collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>()
                    .into_iter()
            }
        })
    }

    pub fn p2_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p2.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter()
                    .flat_map(|g| self.solvers_p2.get(ty).unwrap().iter().map(move |s| (g, s)))
                    .collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>()
                    .into_iter()
            }
        })
    }
}

pub fn discover_mod_contents(module: &ItemMod) -> syn::Result<AocSolutionsAggregation> {
    let mut errs: Vec<syn::Error> = Vec::new();

    let mut solutions_p1: Vec<AocSolutionData> = Vec::new();
    let mut solutions_p2: Vec<AocSolutionData> = Vec::new();
    let mut generators: HashMap<&Type, Vec<AocGeneratorData>> = HashMap::new();
    let mut solvers_p1: HashMap<&Type, Vec<AocSolverData>> = HashMap::new();
    let mut solvers_p2: HashMap<&Type, Vec<AocSolverData>> = HashMap::new();
    let mut p1_solution_type: Option<&Type> = None;
    let mut p2_solution_type: Option<&Type> = None;

    let Some((_, contents)) = &module.content else {
        return Ok(AocSolutionsAggregation::new());
    };
    for mod_item in contents.iter() {
        match mod_item {
            Item::Fn(fn_data) => {
                for attr in fn_data.attrs.iter() {
                    match attr.path().get_ident().map(|id| id.to_string()).as_deref() {
                        Some("generator") => {
                            let args = attr.parse_args::<AocGeneratorArgs>()?;
                            let data = AocGeneratorData::new(args, fn_data)?;
                            generators.entry(data.gen_type).or_default().push(data);
                        }
                        Some("solver") => {
                            let args = attr.parse_args::<AocSolverArgs>()?;
                            let data = AocSolverData::new(args, fn_data)?;

                            match data.problem_part {
                                AocPart::Part1 => {
                                    if p1_solution_type.is_some_and(|t| t != data.solution_type) {
                                        errs.push(syn::Error::new(data.display_slug.span(), SOLUTION_TYPE_MISMATCH))
                                    }
                                    p1_solution_type = Some(data.solution_type);
                                    solvers_p1.entry(data.input_type).or_default().push(data);
                                }
                                AocPart::Part2 => {
                                    if p2_solution_type.is_some_and(|t| t != data.solution_type) {
                                        errs.push(syn::Error::new(data.display_slug.span(), SOLUTION_TYPE_MISMATCH))
                                    }
                                    p2_solution_type = Some(data.solution_type);
                                    solvers_p2.entry(data.input_type).or_default().push(data);
                                }
                            }
                        }
                        Some("solution") => {
                            let args = attr.parse_args::<AocSolutionArgs>()?;
                            let data = AocSolutionData::new(args, fn_data)?;
                            if data.problem_part == AocPart::Part1 {
                                if p1_solution_type.is_some_and(|t| t != data.solution_type) {
                                    errs.push(syn::Error::new(data.display_slug.span(), SOLUTION_TYPE_MISMATCH))
                                }
                                p1_solution_type = Some(data.solution_type);
                                solutions_p1.push(data);
                            } else {
                                if p2_solution_type.is_some_and(|t| t != data.solution_type) {
                                    errs.push(syn::Error::new(data.display_slug.span(), SOLUTION_TYPE_MISMATCH))
                                }
                                p2_solution_type = Some(data.solution_type);
                                solutions_p2.push(data);
                            }
                        }
                        Some(_) => {
                            continue;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    if let Some(combined) = errs.into_iter().reduce(|mut x, y| {
        x.combine(y);
        x
    }) {
        return Err(combined);
    }

    Ok(AocSolutionsAggregation {
        solutions_p1: solutions_p1,
        solutions_p2: solutions_p2,
        generators: generators,
        solvers_p1: solvers_p1,
        solvers_p2: solvers_p2,
        p1_result_type: p1_solution_type,
        p2_result_type: p2_solution_type,
    })
}
