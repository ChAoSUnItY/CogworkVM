mod vm;
use std::rc::Rc;
use vm::vm::{Stackable, VM, Process};

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
    let mut vm = Rc::new(new_vm!(Stackable::Int(10), Stackable::Long(12)));
    let mut proc = Process::new_process(vm);
    inst!(proc load 0);
    inst!(proc load 1);
    inst!(proc add);
    inst!(proc dump);
}

