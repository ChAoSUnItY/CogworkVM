use cogwork::{bytecode::*, vm::Stackable, Loader};

fn main() {
    // Emit bytecode
    let mut bytecode_builder = BytecodeBuilder::new();

    // Emit instructions
    let mut instruction_builder = bytecode_builder.visit_code();

    instruction_builder.visit_ldc(Stackable::Int(10));
    instruction_builder.visit_dup();
    instruction_builder.visit_func("add", 2);
    instruction_builder.visit_func("mul", 1);
    instruction_builder.visit_ldc(Stackable::Int(90));
    instruction_builder.visit_mul();
    instruction_builder.visit_return();
    instruction_builder.visit_add();
    instruction_builder.visit_invoke("mul", 1);
    instruction_builder.visit_return();
    instruction_builder.visit_invoke("add", 2);
    instruction_builder.visit_dump();
    instruction_builder.visit_return();

    instruction_builder.visit_end();
    // Build bytecode
    let bytecode = bytecode_builder.visit_end();

    // Load bytecode to vm and load
    let loader = Loader::new(&bytecode);
    let vm = loader.load();

    vm.execute();
}
