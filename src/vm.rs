use std::{vec, fmt::{Debug, Write}, collections::HashMap, rc::Rc};

use arrayvec::ArrayVec;
use enum_index::EnumIndex;

use crate::opcode::Opcode;

macro_rules! make_stackable {
    ($precedence:expr, $expr:expr) => {
        match $precedence {
            0 => Stackable::Int($expr as i32),
            1 => Stackable::Long($expr as i64),
            2 => Stackable::Float($expr as f32),
            3 => Stackable::Double($expr as f64),
            _ => unreachable!(),
        }
    };

    ($precedence:expr, $expr1:expr, $expr2:expr) => {
        (make_stackable!($precedence, $expr1), make_stackable!($precedence, $expr2), $precedence)
    };
}

macro_rules! get_value {
    ($expr:expr) => {
        match $expr {
            Stackable::Int(i) => i as f64,
            Stackable::Long(l) => l as f64,
            Stackable::Float(f) => f as f64,
            Stackable::Double(d) => d,
            Stackable::String(_) => unreachable!(),
        }
    };
}

#[derive(EnumIndex, Clone)]
pub enum Stackable {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
}

impl Stackable {
    pub(crate) fn promotion_precedence(&self) -> i8 {
        match self {
            Self::Int(_) => 0,
            Self::Long(_) => 1,
            Self::Float(_) => 2,
            Self::Double(_) => 3,
            Self::String(_) => panic!("String cannot be promoted.")
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
            Self::Int(i) => f.write_fmt(format_args!("{}", i)),
            Self::Long(l) => f.write_fmt(format_args!("{}L", l)),
            Self::Float(fl) => f.write_fmt(format_args!("{}F", fl)),
            Self::Double(d) => f.write_fmt(format_args!("{}D", d)),
            Self::String(s) => f.write_str(s),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionSignature {
    parameter_types: Vec<String>,
    return_type: String,
}

#[derive(Debug)]
pub struct VM {
    constants: Vec<Stackable>,
    code: Code,
}

impl VM {
    pub fn new_vm(constants: Vec<Stackable>, code: Code) -> Self {
        VM{
            constants,
            code
        }
    }

    pub fn execute(self) {
        let main_proc = Process::new_process(self, 0);

        main_proc.run();
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    max_stack: u16,
    max_local: u16,
    instructions: Vec<Opcode>
}

impl Code {
    pub fn new(max_stack: u16, max_local: u16, instructions: Vec<Opcode>) -> Self {
        Self{
            max_stack,
            max_local,
            instructions,
        }
    }
}

#[derive(Clone)]
pub struct Process {
    vm: Rc<VM>,
    functions: HashMap<FunctionSignature, Rc<dyn FnMut(&mut Process) -> Option<Stackable>>>, // local functions
    stack: Vec<Stackable>,
    pos: usize,
}

impl Process {
    pub fn new_process(vm: VM, pos: usize) -> Self {
        let code = vm.code.clone();

        Self{
            vm: Rc::new(vm),
            functions: HashMap::new(),
            stack: Vec::with_capacity(code.max_stack as usize),
            pos,
        }
    }

    fn get_instruction(&self) -> Option<&Opcode> {
        self.vm.code.instructions.get(self.pos)
    } 

    pub fn run(mut self) {
        while let Some(opcode) = self.get_instruction() {
            match opcode {
                Opcode::Ldc(index) => {
                    self.load(*index as usize);
                }
                Opcode::Dump => {
                    self.dump();
                }
                Opcode::Add => {
                    self.add();
                }
                Opcode::Sub => {
                    self.sub();
                }
                Opcode::Mul => {
                    self.mul();
                }
                Opcode::Div => {
                    self.div();
                }
                Opcode::Mod => {
                    self.r#mod();
                }
                Opcode::Dup => {
                    self.dup();
                }
                &Opcode::Swp => {
                    self.swp();
                }
            }

            self.pos += 1;
        }
    }

    pub fn load(&mut self, index: usize) {
        let constant = self.vm.constants.get(index);

        if let Some(c) = constant {
            self.stack.push(c.clone());
        } else {
            panic!("Unable to load constant at index {}", index);
        }
    }

    pub fn dump(&mut self) {
        let item = self.stack.pop();

        if let Some(i) = item {
            println!("{:?}", i);
        } else {
            panic!("Unable to pop an empty stack");
        }
    }

    pub fn add(&mut self) {
        if let [right, left] = &self.pop(2)[..] {
            let (promoted_right, promoted_left, precedence) = 
                Stackable::promote(right.clone(), left.clone());
            let result_value = get_value!(promoted_left) + get_value!(promoted_right);
    
            self.stack.push(make_stackable!(precedence, result_value));
        }
    }

    pub fn sub(&mut self) {
        if let [right, left] = &self.pop(2)[..] {
            let (promoted_right, promoted_left, precedence) = 
                Stackable::promote(right.clone(), left.clone());
            let result_value = get_value!(promoted_left) - get_value!(promoted_right);
    
            self.stack.push(make_stackable!(precedence, result_value));
        }
    }

    pub fn mul(&mut self) {
        if let [right, left] = &self.pop(2)[..] {
            let (promoted_right, promoted_left, precedence) = 
                Stackable::promote(right.clone(), left.clone());
            let result_value = get_value!(promoted_left) * get_value!(promoted_right);
    
            self.stack.push(make_stackable!(precedence, result_value));
        }
    }

    pub fn div(&mut self) {
        if let [right, left] = &self.pop(2)[..] {
            let (promoted_right, promoted_left, precedence) = 
                Stackable::promote(right.clone(), left.clone());
            let result_value = get_value!(promoted_left) / get_value!(promoted_right);
    
            self.stack.push(make_stackable!(precedence, result_value));
        }
    }

    pub fn r#mod(&mut self) {
        if let [right, left] = &self.pop(2)[..] {
            let (promoted_right, promoted_left, precedence) = 
                Stackable::promote(right.clone(), left.clone());
            let result_value = get_value!(promoted_left) % get_value!(promoted_right);
    
            self.stack.push(make_stackable!(precedence, result_value));
        }
    }

    pub fn dup(&mut self) {
        self.check_stack_size(1);

        let stackable = self.stack.last().unwrap();
        self.stack.push(stackable.clone());    
    }

    pub fn swp(&mut self) {
        self.check_stack_size(2);

        if let [top1, top2] = &self.pop(2)[..] {
            self.push(&[top2.clone(), top1.clone()]);
        }
    }

    pub fn r#return(&mut self) -> Option<Stackable> {
        return self.stack.pop();
    }

    fn pop(&mut self, pop_size: usize) -> Vec<Stackable> {
        self.check_stack_size(2);

        self.stack.drain(self.stack.len() - pop_size..).collect()
    }

    fn push(&mut self, items: &[Stackable]) {
        self.stack.extend_from_slice(items);
    }

    fn check_stack_size(&self, required_size: usize) {
        if self.stack.len() < required_size {
            panic!("Unable to perform addition, requires {}+ items on stack but got {}", required_size, self.stack.len());
        }
    }
}