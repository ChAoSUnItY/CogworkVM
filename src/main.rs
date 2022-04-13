use std::rc::Rc;
use cogwork::{vm::*, bytecode::*};

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
    let mut bytecode_builder = BytecodeBuilder::new();
    let mut constant_builder = bytecode_builder.visit_constant_pool();
    
    constant_builder.visit_constant(&1);

    constant_builder.visit_end();
    let bytecode = bytecode_builder.visit_end();

    let loader = cogwork::Loader::new(&bytecode);

    println!("{:?}", loader.load());
}

