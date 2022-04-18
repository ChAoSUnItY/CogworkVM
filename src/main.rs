use cogwork::{bytecode::*, Loader, opcode::Opcode};

fn main() {
    // Emit bytecode
    let mut bytecode_builder = BytecodeBuilder::new();
    
    // Emit instructions
    let mut instruction_builder = bytecode_builder.visit_code();



    instruction_builder.visit_end();
    // Build bytecode
    let bytecode = bytecode_builder.visit_end();

    // Load bytecode to vm and load
    let loader = Loader::new(&bytecode);
    let vm = loader.load();

    vm.execute();
}

