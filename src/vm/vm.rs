use std::{vec, fmt::Debug, collections::HashMap, borrow::BorrowMut, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl From<StackItem> for Constant {
    fn from(item: StackItem) -> Self {
        match item {
            StackItem::Int(i) => Constant::Int(i),
            StackItem::Long(l) => Constant::Long(l),
            StackItem::Float(f) => Constant::Float(f),
            StackItem::Double(d) => Constant::Double(d),
        }
    }
}

impl Into<StackItem> for Constant {
    fn into(self) -> StackItem {
        match self {
            Constant::Int(i) => StackItem::Int(i),
            Constant::Long(l) => StackItem::Long(l),
            Constant::Float(f) => StackItem::Float(f),
            Constant::Double(d) => StackItem::Double(d),
        }
    }
}

#[derive(Clone, Copy)]
pub enum StackItem {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl Debug for StackItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0) => f.write_str(&arg0.to_string()),
            Self::Long(arg0) => f.write_str(&arg0.to_string()),
            Self::Float(arg0) => f.write_str(&arg0.to_string()),
            Self::Double(arg0) => f.write_str(&arg0.to_string()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionSignature {
    parameter_types: Vec<String>,
    return_type: String,
}

#[derive(Clone)]
pub struct VM<'a> {
    constants: Vec<Constant>,
    process: Option<Process<'a>>
}

impl<'a> VM<'a> {
    pub fn new_vm(constants: Vec<Constant>) -> Self {
        VM{
            constants,
            process: None,
        }
    }
}

#[derive(Clone)]
pub struct Process<'a> {
    vm: &'a VM<'a>,
    functions: HashMap<FunctionSignature, Rc<dyn FnMut(&mut Process) -> Option<StackItem>>>, // local functions
    stack: Vec<StackItem>
}

impl<'a> Process<'a> {
    pub fn new_process(vm: &'a VM) -> Self {
        Process{
            vm,
            functions: HashMap::new(),
            stack: vec![]
        }
    }

    pub fn load(&mut self, index: usize) -> &mut Self {
        let constant = self.vm.constants.get(index);

        if let Some(c) = constant {
            self.stack.push(c.clone().into());
        } else {
            panic!("Unable to load constant at index {}", index);
        }

        return self;
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

    pub fn r#return(&mut self) -> Option<StackItem> {
        return self.stack.pop();
    }
}