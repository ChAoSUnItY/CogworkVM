mod cogwork;
use std::rc::Rc;
use cogwork::{vm::{Stackable, VM, Process}, bytecode::BytecodeBuilder};

macro_rules! new_vm {
    ($($constant:expr),*) => {{
        let mut c = Vec::<Stackable>::new();
        $(
            c.push($constant);
        )*
        VM::new_vm(c)
    }};
}

macro_rules! inst {
    ($id:ident load $index:expr) => {
        $id.load($index);
    };
    ($id:ident add) => {
        $id.add();
    };
    ($id:ident dump) => {
        $id.dump();
    };
    ($id:ident return) => {
        $id.r#return()
    };
}


fn main() {
    let mut vm = Rc::new(new_vm!(Stackable::Int(10), Stackable::Double(12.)));
    let mut proc = Process::new_process(vm);
    inst!(proc load 0);
    inst!(proc load 1);
    inst!(proc add);
    inst!(proc dump);

    let mut bytecode_builder = BytecodeBuilder::new();
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&10);
    constant_builder.visit_constant(&20i64);
    constant_builder.visit_constant(&1.1f32);
    constant_builder.visit_constant(&1.2f64);

    constant_builder.visit_end();
    let bytecode = bytecode_builder.visit_end();

    println!("{:?}", bytecode);
}

