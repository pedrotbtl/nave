use crate::{encoder::Translator, smt::Value, smt::Output as SolverOutput};
pub use crate::smt::Solver;
use acir::{AcirField, FieldElement};
use noirc_driver::CompiledProgram;
use std::collections::HashMap;

mod encoder;
// mod annotation;
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
    Sat(Model),
    Unsat,
    Unknown,
}

pub fn check_program(program: &CompiledProgram, use_int: bool) -> Output {
    let mut solver = if use_int { Solver::new_int(PRIME) } else { Solver::new_ff(PRIME) };
    let circuit = program.program.functions.first().unwrap();
    // println!("Circuit: \n {:?}", circuit);
    let mut brillig_funcs: HashMap<u32, String> = HashMap::new();
    // println!("Brillig functions ---------");
    for (fn_index, name) in program.brillig_names.clone().into_iter().enumerate() {
        // println!("id: {}, name: {}", fn_index, name);
        brillig_funcs.insert(fn_index as u32, name);
    }
    let mut translator = Translator::new(&mut solver, brillig_funcs, circuit.num_vars(), use_int);
    translator.translate_to_smt(circuit);
    let output = solver.check_sat().unwrap();
    match output {
        SolverOutput::Sat => {
            Output::Sat(solver.get_model())
        },
        SolverOutput::Unsat => Output::Unsat,
        SolverOutput::Unknown => Output::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::smt::{FField, Solver, Type};

    use super::*;

    use noirc_driver::{CompileOptions, CompiledProgram, file_manager_with_stdlib, prepare_crate};
    use noirc_frontend::hir::{Context, def_map::parse_file};

    #[allow(unused)]
    const SOURCE_1: &str = "fn main(mut x : u8, y : pub u8, mut array : [Field; 5]) {
        // assert(y > 1);
        x = 255 + y + 1;
        // assert(x < y);
        // assert(y == x);
        // array[i+1] = y;
        // assert(x == array[i]);
        // assert(array[i+1] != array[i]);
        // assert(array[4] == x);
        // unsafe { verify_assert(x == y);}
        // assert(x == y);
        // unsafe { verify_assert(x != y);}
        
    }
    
    unconstrained fn verify_assert(b : bool) {assert(false);}";

    // const SOURCE_1 : &str = "fn main(x : Field, y : pub Field, mut array : [Field; 5], i : u32) {

    //     // assert(x < y);
    //     // assert(y == x);
    //     array[i+1] = y;
    //     assert(x == array[i]);
    //     assert(array[i+1] != array[i]);
    //     // assert(array[4] == x);
    //     unsafe { verify_assert(x == y);}
    //     // assert(x == y);
    //     // unsafe { verify_assert(x != y);}

    // }

    // unconstrained fn verify_assert(b : bool) {assert(false);}";

    #[allow(unused)]
    const SOURCE_2: &str = "fn main(x : Field, y : pub Field) {
        let x = 25;
        let y = 25;
        assert(x != y);
    }";

    #[allow(unused)]
    const SOURCE_3: &str = "fn main(index1: u32, index2: u32, mut values: [Field; 5]) -> pub u32 {
        // values[index1] = 2;
        // assert(values[index1] == 2);

        let val1 = values[index1] as u32;
        let val2 = values[index2] as u32;

        let mut result = 0;

        if val1 > val2 {
            result = val1 + val2;
        } else {
            result = val1 * val2;
        }

        result
    }";

    const PRIME: &str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";

    // #[test]
    // fn test_solve_add() {

    //     let source = "fn main(x : Field, y : Field) {
    //             let z = x + y;
    //             assert(z/(x + y) == 1);
    //         }

    //         unconstrained fn verify_assert(b : bool) {assert(false);}";

    //     let smt_res = compile_and_solve(source);
    //     assert_eq!(smt_res, ast::CheckSatResponse::Sat);
    // }

    #[test]
    fn test_solver_model() {
        // let st = Storage::new();
        let mut solver = Solver::new_ff(PRIME);
        solver.declare_const("x", Type::FField);
        let x = FField::new_const("x");
        let zero = FField::new_value("0");

        solver.assert(x.eq(zero));

        let res = solver.check_sat().unwrap();
        println!("Sat: {:?}", res);

        let model = solver.get_model();
        println!("Model: {:?}", model);
    }

    #[test]
    fn test_solve_mul() {
        #[allow(unused)]
        let source = "fn main(x : u8) {
                // Safety: for verification purposes
                unsafe { verify_assume(x > 0); }
                let z = x * x;
                // assert (z != 1);
                // Safety: for verification purposes
                unsafe { verify_assert(x == 1); }
                // Safety: for verification purposes
                unsafe { verify_assert(z != 4); }
                // assert(x == 1);
            }
            
            unconstrained fn verify_assert(b : bool) {}
            unconstrained fn verify_assume(b : bool) {}";

        #[allow(unused)]
        let source2 = "fn main(mut x: u8)
        //-> pub u8
        {
            // Safety: for verification purposes
            // unsafe { verify_pre(x < 85); }
            x = 120;
            // Safety: for verification purposes
            unsafe { verify_assert(x + 1 == 121); }
            // x = 100;
            // x
            // assert(x < 85);
            // assert(x == 120);
        }
        unconstrained fn verify_assert(exp: bool) {}
        unconstrained fn verify_pre(exp: bool) {}";

        #[allow(unused)]
        let raw_source1 = "fn main(x : u8) {
                // Safety: for verification purposes 
                let y = x as Field;
                // assert(b);
                unsafe { verify_assert(y != 255); }
            }
            unconstrained fn verify_assert(exp: bool) {}";

        #[allow(unused)]
        let raw_source2 = "// @precondition(x < 85)
        fn main(mut x: u8) -> pub u8
        {
            x = 120;
            // @assert(x + 1 == 121)
            if (x > 34) { x = 34; }
            x
        }
        // @precondition(x < 80)
        fn test(mut x: u8) -> u8 {
            x = 120;
            // @assert(x + 1 == 121)
            if (x > 34) { x = 34; }
            x
        }";

        #[allow(unused)]
        let raw_source3 = "fn main(mut x: i8, y: Field) -> pub Field {
            assert(x > -67);
            // x = x + 20;
            let z = y * 2;
            z
        }";

        #[allow(unused)]
        let raw_source4 = "fn main(y: Field) -> pub Field {
            // y = y + 20;
            // @assert y == 0
            assert(y == 1);
            // @assert y == 1
            // @assert y == 0
            let x = y * 2;
            // @assert x == 0
            // test comment
            x
        }
        unconstrained fn verify_assert(b: bool) {}";

        #[allow(unused)]
        let source_for_attr = "
        #![postcondition x == 20
        fn main(y: Field) -> pub Field {
            // y = y + 20;
            // @assert y == 0
            assert(y == 1);
            // @assert y == 1
            // @assert y == 0
            let x = y * 2;
            // @assert x == 0
            // test comment
            x
        }

        #![postcondition(x > 20)
        fn test(mut x: u8) -> u8 {
            x = 120;
            // @assert x + 1 == 121
            if (x > 34) { x = 34; }
            x
        }
        
        fn postcondition(b: bool) {}
        unconstrained fn verify_assert(b: bool) {}";

        #[allow(unused)]
        let source_for_doc_comments = "
        fn main(y: Field) -> pub Field {
            // y = y + 20;
            // @assert y == 0
            assert(y == 1);
            // @assert y == 1
            // @assert y == 0
            let x = y * 2;
            // @assert x == 0
            // test comment
            x
        }
        
        unconstrained fn verify_postcondition(b: bool) {}
        unconstrained fn verify_assert(b: bool) {}";

        // let program = noirc_frontend::parse_program_with_dummy_file(raw_source4);
        // println!("Parsed module {:?}", program.0.items);

        // let input = parse_and_translate(raw_source4);
        let smt_res = compile_and_solve(&raw_source1, false);
        match smt_res {
            Output::Sat(model) => {
                println!("Sat with Model: {:?}", model);
            },
            Output::Unsat => {
                println!("Unsat");
            },
            Output::Unknown => {
                println!("Unknown");
            }
            
        }

        // assert_eq!(smt_res, ast::CheckSatResponse::Sat);
    }

    fn compile_and_solve(source: &str, use_int: bool) -> Output {
        // let mut solver = if use_int { Solver::new_int(PRIME) } else { Solver::new_ff(PRIME) };
        let comp_program = compile_noir_source_from_string(source);
        println!("Program: \n {:?}", comp_program);

        check_program(&comp_program, use_int)
    }

    fn compile_noir_source_from_string(prog_str: &str) -> CompiledProgram {
        let root = Path::new("");
        let file_name = Path::new("main.nr");
        let mut file_manager = file_manager_with_stdlib(root);
        file_manager.add_file_with_source(file_name, prog_str.to_owned()).expect(
            "Adding source buffer to file manager should never fail when file manager is empty",
        );
        let parsed_files = file_manager
            .as_file_map()
            .all_file_ids()
            .map(|&file_id| (file_id, parse_file(&file_manager, file_id)))
            .collect();

        let mut context = Context::new(file_manager, parsed_files);
        let root_crate_id = prepare_crate(&mut context, file_name);

        let ((), _) =
            noirc_driver::check_crate(&mut context, root_crate_id, &Default::default()).unwrap();

        // let options = CompileOptions::default();
        let skip_passes = vec![
            "Inlining Brillig Calls".to_string(),
            "Remove Unreachable Instructions".to_string(),
            "Dead Instruction Elimination - ACIR".to_string(),
        ];
        let options = CompileOptions {
            // show_brillig: true,
            skip_ssa_pass: skip_passes,
            // show_ssa: true,
            // show_monomorphized: true,
            // minimal_ssa: true,
            // force_brillig: true,
            ..CompileOptions::default()
        };

        // context.def_map(root_crate_id).unwrap();

        let main_id = context.get_main_function(&root_crate_id).unwrap();

        noirc_driver::compile_no_check(&mut context, &options, main_id, None, false).unwrap()
    }
}
