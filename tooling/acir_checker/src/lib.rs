// Function for running the verifier for ACIR

use std::collections::HashMap;
use noirc_driver::CompiledProgram;
use crate::{
    encoder::Translator,
    smt::{
        Bool, 
        Output as SolverOutput,
        Value,
    },
};

pub use crate::smt::Solver;
pub use crate::error::Error;

mod encoder;
mod error;
mod smt;

const _: () = {
    // Ensure that exactly one of the features is selected.
    #[cfg(all(feature = "bn254", not(feature = "bls12_381")))]
    compile_error!("feature \"bn254\" requires feature \"bls12_381\" to be disabled");
};

pub const PRIME: &str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

type Model = HashMap<String, Value>;

pub enum Output {
    Falsified(Model),
    Verified,
    Unknown,
}

impl Output {
    pub fn is_verified(&self) -> bool {
        matches!(self, Output::Verified)
    }
    pub fn is_falsified(&self) -> bool {
        matches!(self, Output::Falsified(_))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BackendType {
    FfGb,
    FfSplit,
    Int,
}

impl Default for BackendType {
    fn default() -> Self {
        BackendType::FfGb
    }
}

pub fn check_program(
    program: &CompiledProgram,
    backend: BackendType,
    strict: bool,
) -> Result<Vec<Output>, Error> {
    let mut solver = match backend {
        BackendType::FfGb => Solver::new_ff_gb(PRIME),
        BackendType::FfSplit => Solver::new_ff_split(PRIME),
        BackendType::Int => Solver::new_int(PRIME),
    };
    let circuit = program.program.functions.first().unwrap();
    let mut brillig_funcs: HashMap<u32, String> = HashMap::new();
    for (fn_index, name) in program.brillig_names.clone().into_iter().enumerate() {
        brillig_funcs.insert(fn_index as u32, name);
    }
    let vec_conds = {
        let mut translator = match backend {
            BackendType::FfGb | BackendType::FfSplit => {
                Translator::new(&mut solver, brillig_funcs, circuit.num_vars(), false, strict)
            }
            BackendType::Int => {
                Translator::new(&mut solver, brillig_funcs, circuit.num_vars(), true, strict)
            }
        };
        translator.translate_to_smt(circuit)?;
        translator.ver_conds()
    };
    let mut outputs = Vec::new();
    for actlit_name in &vec_conds {
        let out_sat = solver.check_sat_assuming(actlit_name).unwrap();
        let output = match out_sat {
            SolverOutput::Sat => Output::Falsified(solver.get_model()),
            SolverOutput::Unsat => Output::Verified,
            SolverOutput::Unknown => Output::Unknown,
        };
        if let Output::Verified = output {
            solver.assert(Bool::new_const(actlit_name).neg());
        }
        outputs.push(output);
    }
    Ok(outputs)
}
