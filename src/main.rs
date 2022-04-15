use cogwork::{bytecode::*, Loader, opcode::Opcode};

fn main() {
    let mut bytecode_builder = BytecodeBuilder::new(COMPUTE_LOCAL | COMPUTE_STACK);
    println!("{:?}", bytecode_builder);
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&1);
    constant_builder.visit_constant(&10i64);
    constant_builder.visit_constant(&1.0f32);
    constant_builder.visit_constant(&0.1f64);
    constant_builder.visit_constant(&"POGGER");

    constant_builder.visit_end();
    let mut instruction_builder = bytecode_builder.visit_code();

    instruction_builder.visit_ldc(4);
    instruction_builder.visit_store(0);
    instruction_builder.visit_load(0);
    instruction_builder.visit_dup();
    instruction_builder.visit_dump();
    instruction_builder.visit_opcode(Opcode::Dump);

    instruction_builder.visit_max(2, 0);
    instruction_builder.visit_end();
    let bytecode = bytecode_builder.visit_end();

    let loader = Loader::new(&bytecode);
    let vm = loader.load();

    println!("{:?}", vm);

    vm.execute();
}

