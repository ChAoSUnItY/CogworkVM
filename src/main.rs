use cogwork::{bytecode::*, Loader, opcode::Opcode};

fn main() {
    // Emit bytecode
    let mut bytecode_builder = BytecodeBuilder::new();
    // Emit constants
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&1);
    constant_builder.visit_constant(&10i64);
    constant_builder.visit_constant(&1.0f32);
    constant_builder.visit_constant(&0.1f64);
    constant_builder.visit_constant(&"POGGER");

    constant_builder.visit_end();
    // Emit instructions
    let mut instruction_builder = bytecode_builder.visit_code();

    // Make labels
    let label_a = instruction_builder.make_label();
    let label_b = instruction_builder.make_label();
    let label_c = instruction_builder.make_label();
    
    instruction_builder.visit_ldc(4);
    instruction_builder.visit_goto(&label_a);
    instruction_builder.visit_label(&label_b);
    instruction_builder.visit_store(0);
    instruction_builder.visit_load(0);
    instruction_builder.visit_dup();
    instruction_builder.visit_dump();
    instruction_builder.visit_goto(&label_c);
    instruction_builder.visit_label(&label_a);
    instruction_builder.visit_goto(&label_b);
    instruction_builder.visit_label(&label_c);
    instruction_builder.visit_opcode(Opcode::Dump);

    instruction_builder.visit_end();
    // Build bytecode
    let bytecode = bytecode_builder.visit_end();

    // Load bytecode to vm and load
    let loader = Loader::new(&bytecode);
    let vm = loader.load();

    vm.execute();
}

