use std::{collections::HashMap, marker::PhantomData};

use acir::{
    AcirField, brillig::Opcode, circuit::{
        Circuit,
        brillig::{BrilligFunctionId, BrilligInputs, BrilligOutputs},
        opcodes::{
            AcirFunctionId, BlackBoxFuncCall, BlockId, ConstantOrWitnessEnum,
            FunctionInput, MemOp,
        },
    }, native_types::{Expression, Witness}
};

use crate::smt::{FField, Int, Solver, Type};

pub(crate) struct Translator<'a, F: AcirField> {
    solver: &'a mut Solver,
    // witness_map: HashMap<Field, Option<Expression<F>>>,
    brillig_funcs: HashMap<u32, String>,
    next_witness_index: u32,
    use_int : bool,
    _f: PhantomData<F>,
}

struct MemTrace<F: AcirField> {
    block_id: BlockId,
    init: Vec<Witness>,
    ops: Vec<MemOp<F>>,
}

impl<'a, F: AcirField> Translator<'a, F> {
    pub(crate) fn new(
        solver: &'a mut Solver,
        brillig_funcs: HashMap<u32, String>, 
        next_witness_index: u32,
        use_int : bool,
    ) -> Translator<'a, F> {
        assert!(solver.prime() == F::modulus().to_string());
        Translator { 
            solver, 
            // witness_map: HashMap::new(),
            brillig_funcs,
            next_witness_index, 
            use_int,
            _f: PhantomData,
        }
    }

    pub(crate) fn translate_to_smt(&mut self, circuit: &Circuit<F>) {
        let num_vars = circuit.num_vars();
        let witnesses = circuit.circuit_arguments();
        let public_inputs = circuit.public_inputs();
        // println!("Witness: {:?} Public inputs: {:?}", witnesses, public_inputs);

        for wi in 0..num_vars {
            if self.use_int {
                self.solver.declare_const(&format!("{}", Witness(wi)), Type::Int);
                // let prime = self.prime_int();
                // let zero = Int::zero();
                // let wit = self.new_const_int(Witness(wi));
                // self.solver.assert(wit.clone().gte(zero));
                // self.solver.assert(wit.lt(prime));
            } else {
                self.solver.declare_const(&format!("{}", Witness(wi)), Type::FField);
            }
        }
        let mut mem_traces: HashMap<BlockId, MemTrace<F>> = HashMap::new();
        for opcode in &circuit.opcodes {
            // println!("Opcode: {:?}", opcode);
            match opcode {
                acir::circuit::Opcode::AssertZero(expression) => {
                    self.translate_assert_zero(expression)
                }
                acir::circuit::Opcode::BlackBoxFuncCall(black_box_func_call) => {
                    self.translate_blackbox_call(black_box_func_call)
                }
                acir::circuit::Opcode::MemoryOp { block_id, op, predicate: _ } => {
                    // self.translate_memory_op(*block_id, op, predicate.as_ref())
                    let mem_trace = mem_traces
                        .get_mut(block_id)
                        .expect("MemInit opcode should have run before");
                    mem_trace.ops.push(op.clone());
                }
                acir::circuit::Opcode::MemoryInit { block_id, init, block_type: _ } => {
                    // self.translate_memory_init(*block_id, init, block_type)
                    let mem_trace = MemTrace {
                        block_id: *block_id,
                        init: init.iter().cloned().collect(),
                        ops: Vec::new(),
                    };
                    mem_traces.insert(*block_id, mem_trace);
                }
                acir::circuit::Opcode::BrilligCall { id, inputs, outputs, predicate } => {
                    self.translate_brilling_call(*id, inputs, outputs, predicate.as_ref())
                }
                acir::circuit::Opcode::Call { id, inputs, outputs, predicate } => {
                    self.translate_call(*id, inputs, outputs, predicate.as_ref())
                }
            }
        }
        for mem_trace in mem_traces.values() {
            // println!("Translating memory block {}", mem_trace.block_id.0);
            self.translate_memory_init(mem_trace.block_id, &mem_trace.init);
            let mem_block_len = mem_trace.init.len();
            let mut f = 0;
            for op in &mem_trace.ops {
                f = if self.use_int {
                    self.translate_memory_op_int(mem_trace.block_id, op, mem_block_len, f)
                } else {
                    self.translate_memory_op(mem_trace.block_id, op, mem_block_len, f)
                };
            }
        }
        // println!("witness map {:?}", self.witness_map);
    }

    fn translate_memory_init(&mut self, block_id: BlockId, init: &[Witness]) {
        //, _block_type: &BlockType) {
        // println!("INIT {} {}",block_id.0, init.len());
        for (i, wit) in init.iter().enumerate() {
            self.solver.declare_const(&format!("_m_{}_{}_0", block_id.0, i), Type::FField);
            let gate = FField::new_const(&format!("_m_{}_{}_0", block_id.0, i));
            let wit = self.new_const(*wit);
            let eq = wit.eq(gate);
            self.solver.assert(eq);
        }
    }

    // Predicate is excluded
    fn translate_memory_op(&mut self, block_id: BlockId, op: &MemOp<F>, len: usize, f: u32) -> u32 {
        // println!("MEMOP {} {} {:?}", block_id.0, f, op);
        let value = &op.value;
        let index = &op.index;
        let op = &op.operation;
        assert!(op.is_const());
        
        let index_wit = self.new_witness();
        let index_exp = self.translate_expression(index);
        // self.witness_map.insert(index_wit.1, Some(index.clone()));
        self.solver.assert(index_wit.clone().eq(index_exp));

        let value_wit = self.new_witness();
        let value_exp = self.translate_expression(value);
        // self.witness_map.insert(value_wit.1, Some(value.clone()));
        self.solver.assert(value_wit.clone().eq(value_exp));
        
        if op.is_zero() {
            // Read operation
            // index != op.value == x0
            for i in 0..len {
                let indexed_mem_value =
                    FField::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f));
                let index_value = FField::new_value(&format!("{}", i));
                let exp = index_wit.clone().eq(index_value).imp(value_wit.clone().eq(indexed_mem_value));
                self.solver.assert(exp);
            }
            f
        } else {
            // Write operation
            assert!(op == &Expression::one());
            let f_new = f + 1;
            for i in 0..len {
                self.solver.declare_const(&format!("_m_{}_{}_{}", block_id.0, i, f_new), Type::FField);
                let indexed_mem_value =
                    FField::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f_new));
                let pre_indexed_mem_value =
                    FField::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f));

                // let index_exp = self.translate_expression(index);
                // let source_exp = self.translate_expression(value);
                let index_value = FField::new_value(&format!("{}", i));
                let value_change_exp = index_wit.clone().eq(index_value.clone()).imp(value_wit.clone().eq(indexed_mem_value.clone()));
                self.solver.assert(value_change_exp);
                let other_values_remain_exp = index_wit.clone().eq(index_value).neg().imp(pre_indexed_mem_value.eq(indexed_mem_value));
                self.solver.assert(other_values_remain_exp);
            }
            f_new
        }
    }

    fn translate_memory_op_int(&mut self, block_id: BlockId, op: &MemOp<F>, len: usize, f: u32) -> u32 {
        let value = &op.value;
        let index = &op.index;
        let op = &op.operation;
        assert!(op.is_const());
        
        let index_wit = self.new_witness_int();
        let index_exp = self.translate_expression_int(index);
        // self.witness_map.insert(index_wit.1, Some(index.clone()));
        self.solver.assert(index_wit.clone().eq(index_exp));

        let value_wit = self.new_witness_int();
        let value_exp = self.translate_expression_int(value);
        // self.witness_map.insert(value_wit.1, Some(value.clone()));
        self.solver.assert(value_wit.clone().eq(value_exp));
        
        if op.is_zero() {
            // Read operation
            // index != op.value == x0
            for i in 0..len {
                let indexed_mem_value =
                    Int::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f));
                let index_value = Int::new_value(&format!("{}", i));
                let exp = index_wit.clone().eq(index_value).imp(value_wit.clone().eq(indexed_mem_value));
                self.solver.assert(exp);
            }
            f
        } else {
            // Write operation
            assert!(op == &Expression::one());
            let f_new = f + 1;
            for i in 0..len {
                self.solver.declare_const(&format!("_m_{}_{}_{}", block_id.0, i, f_new), Type::FField);
                let indexed_mem_value =
                    Int::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f_new));
                let pre_indexed_mem_value =
                    Int::new_const(&format!("_m_{}_{}_{}", block_id.0, i, f));
                // let index_exp = self.translate_expression(index);
                // let source_exp = self.translate_expression(value);
                let index_value = Int::new_value(&format!("{}", i));
                let value_change_exp = index_wit.clone().eq(index_value.clone()).imp(value_wit.clone().eq(indexed_mem_value.clone()));
                self.solver.assert(value_change_exp);
                let other_values_remain_exp = index_wit.clone().eq(index_value).neg().imp(pre_indexed_mem_value.eq(indexed_mem_value));
                self.solver.assert(other_values_remain_exp);
            }
            f_new
        }
    }

    fn translate_blackbox_call(&mut self, black_box_func_call: &BlackBoxFuncCall<F>) {
        match black_box_func_call {
            BlackBoxFuncCall::AND { lhs, rhs, output } => {
                self.translate_and(lhs, rhs, *output);
            }
            BlackBoxFuncCall::XOR { lhs, rhs, output } => {
                self.translate_xor(lhs, rhs, *output);
            }
            BlackBoxFuncCall::RANGE { input } => {
                // println!("has range");
                self.translate_range(input);
            }
            BlackBoxFuncCall::AES128Encrypt {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::Blake2s {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::Blake3 {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::EcdsaSecp256k1 {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::EcdsaSecp256r1 {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::MultiScalarMul {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::EmbeddedCurveAdd {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::Keccakf1600 {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::RecursiveAggregation {..} => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntAdd { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntSub { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntMul { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntDiv { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntFromLeBytes { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::BigIntToLeBytes { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::Poseidon2Permutation { .. } => { println!("unimplemented"); return },
            BlackBoxFuncCall::Sha256Compression { .. } => { println!("unimplemented"); return },
        }
    }

    fn translate_assert_zero(&mut self, expression: &Expression<F>) {
        if self.use_int {
            let exp = self.translate_expression_int(expression);
            let zero = self.zero_int();
            self.solver.assert(exp.eq(zero));
        } else {
            let exp = self.translate_expression(expression);
            let zero = self.zero();
            self.solver.assert(exp.eq(zero));
        }
    }

    fn translate_assert_one(&mut self, expression: &Expression<F>) {
        if self.use_int {
            let exp = self.translate_expression_int(expression);
            let one = self.one_int();
            self.solver.assert(exp.eq(one));
        } else {
            let exp = self.translate_expression(expression);
            let one = self.one();
            self.solver.assert(exp.eq(one));
        }
    }

    fn translate_expression(&mut self, expression: &Expression<F>) -> FField {
        // println!("expression: {}", expression);
        let mut exps = Vec::new();
        for mul_term in &expression.mul_terms {
            let element = self.new_element(mul_term.0);
            let wit0 = self.new_const(mul_term.1);
            let wit1 = self.new_const(mul_term.2);
            exps.push(FField::rmul(vec![element, wit0, wit1]));
        }
        for lin_term in &expression.linear_combinations {
            let element = self.new_element(lin_term.0);
            let wit = self.new_const(lin_term.1);
            exps.push(FField::rmul(vec![element, wit]));
        }
        let element = FField::new_value(&expression.q_c.to_string());
        if exps.is_empty() {
            element
        } else {
            exps.push(element);
            FField::radd(exps)
        } 
    }

    fn translate_expression_int(&mut self, expression: &Expression<F>) -> Int {
        // println!("expression: {}", expression);
        let mut exps = Vec::new();
        for mul_term in &expression.mul_terms {
            let element = self.new_element_int(mul_term.0);
            let wit0 = self.new_const_int(mul_term.1);
            let wit1 = self.new_const_int(mul_term.2);
            exps.push(Int::rmul(vec![element, wit0, wit1]));
        }
        for lin_term in &expression.linear_combinations {
            let element = self.new_element_int(lin_term.0);
            let wit = self.new_const_int(lin_term.1);
            exps.push(Int::rmul(vec![element, wit]));
        }
        let element = Int::new_value(&expression.q_c.to_string());
        if exps.is_empty() {
            element
        } else {
            exps.push(element);
            let prime = self.prime_int();
            Int::modu(Int::radd(exps), prime)
        } 
    }

    fn translate_brilling_call(
        &mut self,
        id: BrilligFunctionId,
        inputs: &[BrilligInputs<F>],
        outputs: &[BrilligOutputs],
        _predicate: Option<&Expression<F>>,
    ) {
        // predicate indicates if brillig call should be skipped
        // println!(
        //     "func ID: {}, input: {:?}, output: {:?}",
        //     id,
        //     inputs, outputs
        // );

        if let Some(func_name) = self.brillig_funcs.get(&id.0) {
            match func_name.as_str() {
                PRECONDITION_FUNC_NAME => self.translate_verify_precondition(inputs),
                POSTCONDITION_FUNC_NAME => self.translate_verify_postcondition(inputs),
                ASSERT_FUNC_NAME => self.translate_verify_assert(inputs),
                ASSUME_FUNC_NAME => self.translate_verify_assume(inputs),
                _ => {}
            }
        }
    }

    fn translate_verify_assert(
        &mut self,
        inputs: &[BrilligInputs<F>],
    ) {
        for input in inputs {
            match input {
                BrilligInputs::Single(exp) => {
                    // println!("Asserting exp: {:?}", exp);
                    let _ = self.translate_assert_zero(exp);
                }
                BrilligInputs::Array(_exps) => {
                    unimplemented!("Implement array case");
                    // for exp in exps {
                    //     let _ = self.translate_assert_zero(exp);
                    // }
                },
                BrilligInputs::MemoryArray(_id) => {
                    unimplemented!("Implement memory array case");
                    // TODO: How to use block id.
                }
            }
        }
    }

    fn translate_verify_assume(&mut self, inputs: &[BrilligInputs<F>]) {
        for input in inputs {
            match input {
                BrilligInputs::Single(exp) => {
                    let _ = self.translate_assert_one(exp);
                }
                _ => {
                    unimplemented!("assume should be simple expression")
                }
            }
        }
    }

    fn translate_verify_precondition(&mut self, inputs: &[BrilligInputs<F>]) {
        for input in inputs {
            match input {
                BrilligInputs::Single(exp) => {
                    let _ = self.translate_assert_one(exp);
                }
                _ => {
                    unimplemented!("assume should be simple expression")
                 }
            }
        }
    }

    fn translate_verify_postcondition(&mut self, inputs: &[BrilligInputs<F>]) {
        for input in inputs {
            match input {
                BrilligInputs::Single(exp) => {
                    let _ = self.translate_assert_zero(exp);
                }
                _ => {
                    unimplemented!("assume should be simple expression")
                }
            }
        }
    }

    fn translate_call(
        &self,
        _id: AcirFunctionId,
        _inputs: &[Witness],
        _outputs: &[Witness],
        _predicate: Option<&Expression<F>>,
    ) {
        { println!("unimplemented"); return }
    }

    // pub fn solver(self) -> Solver {
    //     self.solver
    // }

    fn translate_range(&mut self, input: &FunctionInput<F>) {
        // optimise to combine all ranges over the same variable
        if input.num_bits() >= 1 {
            match input.input() {
                ConstantOrWitnessEnum::Constant(_) => { println!("unimplemented"); return },
                ConstantOrWitnessEnum::Witness(witness) => {
                    if self.use_int{
                        // println!("witness {:?}, num bits {}", witness, input.num_bits());
                        self.translate_range_int(witness, input.num_bits());
                    } else {
                        self.translate_range_bitsum(witness, input.num_bits());
                    }

                    // (declare-const x (_ BitVec 32))
                    // ;; For unsigned n-bit constraint (e.g., 8 bits)
                    // (assert (bvult x (_ bv256 32))) ; unsigned max = 2^8 = 256
                    // (check-sat)
                }
            }
        }
    }

    fn translate_range_int(&mut self, witness: Witness, num_bits: u32) {
        // let num_bits = if num_bits < 4 { num_bits } else { 4 };
        let value = self.new_element_int(F::pow(&2u32.into(), &num_bits.into()));
        let wit = self.new_const_int(witness);
        // println!("witness {:?}, value {:?}", wit, value);
        self.solver.assert(wit.lt(value));
    }

    fn translate_range_bitsum(&mut self, witness: Witness, num_bits: u32) {
        // let num_bits = if num_bits < 4 { num_bits } else { 4 };
        self.encode_bitsum(witness, num_bits as usize);
    }

    // This encode the expression witness = \sum^{num_bits}_{i=0} 2^i * out[i];
    // where out[i] is a field element in \{0,1}.
    fn encode_bitsum(&mut self, witness: Witness, num_bits: usize) -> Vec<FField> {
        // assert!(num_bits > 0);
        let mut res = Vec::with_capacity(num_bits);
        for _ in 0..num_bits {
            let out_i = self.encode_bool();
            res.push(out_i);
        }
        let exp = if num_bits == 1 {
            res[0].clone()
        } else {
            FField::rbitsum(res.clone())
        };
        let wit = self.new_const(witness);
        self.solver.assert(wit.eq(exp));
        res
    }

    // Create a field constant out such that out in \{0,1}.
    fn encode_bool(&mut self) -> FField {
        let res = self.new_witness();
        let zero = FField::zero();
        self.solver.assert(res.clone().mul(res.clone().add(Self::minus_one())).eq(zero));
        res
    }

    fn translate_and(&mut self, _lhs: &FunctionInput<F>, _rhs: &FunctionInput<F>, _output: Witness) {
        // let mut bitsum_operands  = Vec::with_capacity(num_bits.try_into().unwrap());
        // for i in 0..num_bits {
        //     let out_i_name =format!("_b{}_{}",witness,i);
        //     self.solver.declare_const(&out_i_name);
        //     let out_i = self.solver.new_const(&out_i_name);
        //     let minus_one = self.solver.new_element("-1");
        //     let zero = self.zero::<F>();
        //     self.solver.assert(out_i.mul(out_i.add(minus_one)).eq(zero));
        //     bitsum_operands.push(out_i);
        // }
        // let witness = self.new_const(witness);
        // let exp = if num_bits == 1 {
        //     bitsum_operands[0]
        // } else {
        //     Field::rbitsum(&bitsum_operands).expect("num_bits should be at least 2")
        // };
        // self.solver.assert(witness.eq(exp));
        println!("unimplemented");
    }

    fn translate_xor(&mut self, _lhs: &FunctionInput<F>, _rhs: &FunctionInput<F>, _output: Witness) {
        { println!("unimplemented"); return };
    }

    fn new_const(&mut self, witness: Witness) -> FField {
        let wit_name = witness_name(witness);
        FField::new_const(&wit_name)
    }

    fn new_const_int(&mut self, witness: Witness) -> Int {
        let wit_name = witness_name(witness);
        Int::new_const(&wit_name)
    }

    fn prime_int(&self) -> Int {
        Int::new_value(self.solver.prime())
    }

    fn new_element(&mut self, element: F) -> FField {
        let element_value = element_value(element);
        FField::new_value(&element_value)
    }

    fn new_element_int(&mut self, element: F) -> Int {
        let element_value = element_value(element);
        Int::new_value(&element_value)
    }

    fn zero(&mut self) -> FField {
        // let element_value = element_value(element);
        FField::zero()
    }

    fn zero_int(&mut self) -> Int {
        // let element_value = element_value(element);
        Int::zero()
    }

    fn one(&mut self) -> FField {
        // let element_value = element_value(element);
        FField::one()
    }

    fn one_int(&mut self) -> Int {
        // let element_value = element_value(element);
        Int::one()
    }

    fn minus_one() -> FField {
        // let element_value = element_value(element);
        FField::new_value("-1")
    }

    fn new_witness(&mut self) -> FField {
        let new_wit = Witness(self.next_witness_index);
        let new_wit_name = witness_name(new_wit);
        self.next_witness_index += 1;
        self.solver.declare_const(&new_wit_name, Type::FField);
        // self.witness_map.insert(new_wit, None);
        FField::new_const(&new_wit_name)
    }

    fn new_witness_int(&mut self) -> Int {
        let new_wit = Witness(self.next_witness_index);
        let new_wit_name = witness_name(new_wit);
        self.next_witness_index += 1;
        self.solver.declare_const(&new_wit_name, Type::Int);
        // self.witness_map.insert(new_wit, None);
        Int::new_const(&new_wit_name)
    }
}

fn witness_name(wit: Witness) -> String {
    wit.to_string()
}

fn element_value<F: AcirField>(element: F) -> String {
    element.to_string()
}

#[allow(unused)]
enum VerifierFunctions {
    PreCondition,
    PostCondition,
    Assert,
    Assume,
}

impl VerifierFunctions {
    #[allow(unused)]
    fn as_str(&self) -> &'static str {
        match self {
            VerifierFunctions::Assert => "verify_assert",
            VerifierFunctions::Assume => "verify_assume",
            VerifierFunctions::PostCondition => "verify_postcondition",
            VerifierFunctions::PreCondition => "verify_precondition",
            // VerifierFunctions::Invariant => "@invariant",
        }
    }
}

const PRECONDITION_FUNC_NAME: &str = "verify_pre";

const POSTCONDITION_FUNC_NAME: &str = "verify_post";

const ASSERT_FUNC_NAME: &str = "verify_assert";

const ASSUME_FUNC_NAME: &str = "verify_assume";
