use std::{vec, fmt::Debug, collections::HashMap, rc::Rc};

macro_rules! make_stackable {
    ($precedence:expr, $expr:expr) => {
        match $precedence {
            0 => Stackable::Int($expr as i32),
            1 => Stackable::Long($expr as i64),
            _ => unreachable!()
        }
    };

    ($precedence:expr, $expr1:expr, $expr2:expr) => {
        match $precedence {
            0 => (Stackable::Int($expr1 as i32), Stackable::Int($expr2 as i32), $precedence),
            1 => (Stackable::Long($expr1 as i64), Stackable::Long($expr2 as i64), $precedence),
            _ => unreachable!()
        }
    };
}

macro_rules! get_value {
    ($expr:expr) => {
        match $expr {
            Stackable::Int(i) => i as f64,
            Stackable::Long(l) => l as f64,
        }
    };
}

#[derive(Clone, Copy)]
pub enum Stackable {
    Int(i32),
    Long(i64),
}

impl Stackable {
    pub(crate) fn promotion_precedence(self) -> i8 {
        match self {
            Self::Int(_) => 0,
            Self::Long(_) => 1,
        }
    }

    pub(crate) fn promote(stackable1: Stackable, stackable2: Stackable) -> (Stackable, Stackable, i8) {
        let (left_precedence, right_precedence) = (stackable1.promotion_precedence(), stackable2.promotion_precedence());
        
        if left_precedence == right_precedence {
            make_stackable!(left_precedence, get_value!(stackable1), get_value!(stackable2))
        } else {
            let max_precedence = std::cmp::max(left_precedence, right_precedence);

            make_stackable!(max_precedence, get_value!(stackable1), get_value!(stackable2))
        }
    }
}

impl Debug for Stackable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => f.write_fmt(format_args!("{}\n", i)),
            Self::Long(l) => f.write_fmt(format_args!("{}\n", l)),
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

impl VM {
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
        let (promoted_right, promoted_left, precedence) = Stackable::promote(right, left);
        let result_value = get_value!(promoted_left) + get_value!(promoted_right);

        self.stack.push(make_stackable!(precedence, result_value));

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