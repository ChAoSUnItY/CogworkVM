use cogwork::{bytecode::*, Loader, opcode::Opcode};

fn main() {
    let mut bytecode_builder = BytecodeBuilder::new();
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&1);
    constant_builder.visit_constant(&10i64);
    constant_builder.visit_constant(&1.0f32);
    constant_builder.visit_constant(&0.1f64);
    constant_builder.visit_constant(&"POGGER");

    constant_builder.visit_end();
    let mut instruction_builder = bytecode_builder.visit_code();

    instruction_builder.visit_load(3);
    instruction_builder.visit_load(1);
    instruction_builder.visit_sub();
    instruction_builder.visit_opcode(Opcode::Dump);

    instruction_builder.visit_max(1, 0);
    instruction_builder.visit_end();
    let bytecode = bytecode_builder.visit_end();

    let loader = Loader::new(&bytecode);
    let vm = loader.load();

    println!("{:?}", vm);

    vm.execute();
}

