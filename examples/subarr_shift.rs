use clap::Parser;
use halo2_base::gates::{ GateChip, GateInstructions};
use halo2_base::utils::ScalarField;
use halo2_base::{AssignedValue, Context,QuantumCell:: Constant
};
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub arr: Vec<String>, 
    pub start: usize,
    pub end: usize
}

fn some_algorithm_in_zk<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");
    // `Context` can roughly be thought of as a single-threaded execution trace of a program we want to ZK prove. We do some post-processing on `Context` to optimally divide the execution trace into multiple columns in a PLONKish arithmetization
    // More advanced usage with multi-threaded witness generation is possible, but we do not explain it here

    let bytes = ctx.assign_witnesses(input.arr.iter().map(|b| F::from_str_vartime(b).expect("deserialize field element should not fail")));
    // constraint length of input array 
    assert_eq!(bytes.len(), 1000);
    for byte in &bytes{
        // make it public
        make_public.push(*byte);
    }
    
    // create a gate chip that contains methods for basic arithmetic operations
    let gate = GateChip::<F>::default();

    let mut output:Vec<AssignedValue<F>> = Vec::new();
    // let (c, _) = range.div_mod(ctx,x, 32u32, 16 );
    for idx in input.start..=input.end {
        output.push(gate.select_from_idx(ctx, bytes.clone(), Constant(F::from(idx as u64))));
    }
    let left_over = 1000-(input.end-input.start+1);
    let zero_const = ctx.load_witness(F::from(0));
    for _idx in 0..left_over{
        output.push(zero_const);
    }
    for out_byte in &output{
        // make it public
        make_public.push(*out_byte);
    }
    for idx in 0..1000{
        println!("Index {:?}: {:?}",idx, output.get(idx).unwrap().value());
    }
    
    // println!("c: {:?}", c.value());
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    // run different zk commands based on the command line arguments
    run(some_algorithm_in_zk, args);
}
