use cogwork::{bytecode::*, Loader};

fn main() {
    let mut bytecode_builder = BytecodeBuilder::new();
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&1);
    constant_builder.visit_constant(&10i64);
    constant_builder.visit_constant(&1.0f32);
    constant_builder.visit_constant(&0.1f64);

    constant_builder.visit_end();
    let bytecode = bytecode_builder.visit_end();

    let loader = Loader::new(&bytecode);

    println!("{:?}", loader.load());
}

