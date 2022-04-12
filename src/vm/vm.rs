use std::{vec, fmt::Debug, collections::HashMap, rc::Rc};

#[derive(Clone, Copy)]
pub enum Stackable {
    Int(i32),
}

impl Debug for Stackable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => f.write_fmt(format_args!("{}\n", i)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionSignature {
    parameter_types: Vec<String>,
    return_type: String,
}

#[derive(Clone)]
pub struct VM {
    constants: Vec<Stackable>,
}

impl<'a> VM {
    pub fn new_vm(constants: Vec<Stackable>) -> Self {
        VM{
            constants,
        }
    }
}

#[derive(Clone)]
pub struct Process {
    vm: Rc<VM>,
    functions: HashMap<FunctionSignature, Rc<dyn FnMut(&mut Process) -> Option<Stackable>>>, // local functions
    stack: Vec<Stackable>
}

impl Process {
    pub fn new_process(vm: Rc<VM>) -> Self {
        Process{
            vm,
            functions: HashMap::new(),
            stack: vec![]
        }
    }

    pub fn load(&mut self, index: usize) -> &mut Self {
        let constant = self.vm.constants.get(index);

        if let Some(c) = constant {
            self.stack.push(c.clone());
        } else {
            panic!("Unable to load constant at index {}", index);
        }

        self
    }

    pub fn add(&mut self) -> &mut Self {
        if self.stack.len() < 2 {
            panic!("Unable to perform addition, requires 2+ items on stack but got {}", self.stack.len());
        }

        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        self
    }

    pub fn dump(&mut self) -> &mut Self {
        let item = self.stack.pop();

        if let Some(i) = item {
            println!("{:?}", i);
        } else {
            panic!("Unable to pop an empty stack");
        }

        return self;
    }

    pub fn r#return(&mut self) -> Option<Stackable> {
        return self.stack.pop();
    }
}