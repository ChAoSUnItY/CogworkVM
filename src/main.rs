mod vm;
use vm::vm::{Constant, VM, Process};

macro_rules! vm_process {
    ($($constant:expr),*) => {{
        let mut c = Vec::new();
        $(
            c.push($constant);
        )*
        VM::new_vm(c)
    }};
}

macro_rules! new_proc {
    ($vm_id:ident) => {
        Process::new_process(&$vm_id)
    };
}

macro_rules! inst {
    ($id:ident load $index:expr) => {
        $id.load($index);
    };
    ($id:ident dump) => {
        $id.dump();
    };
    ($id:ident return) => {
        $id.r#return()
    };
}


fn main() {
    let vm = vm_process!(Constant::Int(10), Constant::Int(10));
    let mut proc = new_proc!(vm);
    inst!(proc load 0);
    inst!(proc dump);
}

