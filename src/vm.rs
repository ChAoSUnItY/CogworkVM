use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

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
        (
            make_stackable!($precedence, $expr1),
            make_stackable!($precedence, $expr2),
            $precedence,
        )
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

#[derive(EnumIndex, Clone, PartialEq)]
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
            Self::String(_) => panic!("String cannot be promoted."),
        }
    }

    pub(crate) fn promote(
        stackable1: Stackable,
        stackable2: Stackable,
    ) -> (Stackable, Stackable, i8) {
        let (left_precedence, right_precedence) = (
            stackable1.promotion_precedence(),
            stackable2.promotion_precedence(),
        );

        if left_precedence == right_precedence {
            make_stackable!(
                left_precedence,
                get_value!(stackable1),
                get_value!(stackable2)
            )
        } else {
            let max_precedence = std::cmp::max(left_precedence, right_precedence);

            make_stackable!(
                max_precedence,
                get_value!(stackable1),
                get_value!(stackable2)
            )
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
    function_name_index: u32,
    parameter_size: u8,
}

#[derive(Debug)]
pub struct VM {
    constants: Vec<Stackable>,
    code: Code,
}

impl VM {
    pub fn new_vm(constants: Vec<Stackable>, code: Code) -> Self {
        VM { constants, code }
    }

    pub fn execute(self) {
        let main_proc = Process::new_process(self, 0);

        main_proc.run();
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    instructions: Vec<Opcode>,
}

impl Code {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        Self { instructions }
    }
}

#[derive(Clone)]
pub struct Process {
    vm: Rc<VM>,
    code: Code,
    functions: HashMap<FunctionSignature, u32>,
    stack: Vec<Stackable>,
    local_variable: BTreeMap<u16, Stackable>,
    pos: u32,
}

impl Process {
    pub fn new_process(vm: VM, pos: u32) -> Self {
        let code = vm.code.clone();

        Self {
            vm: Rc::new(vm),
            code,
            functions: HashMap::new(),
            stack: Vec::new(),
            local_variable: BTreeMap::new(),
            pos,
        }
    }

    pub fn subprocess(&mut self, pos: u32, parameters: Vec<Stackable>) -> Self {
        Self {
            vm: self.vm.clone(),
            code: self.vm.code.clone(),
            functions: self.functions.clone(),
            stack: parameters,
            local_variable: self.local_variable.clone(),
            pos,
        }
    }

    fn get_instruction(&self) -> Option<&Opcode> {
        self.vm.code.instructions.get(self.pos as usize)
    }

    pub fn run(mut self) -> Vec<Stackable> {
        while let Some(opcode) = self.get_instruction() {
            let opcode = *opcode;

            match opcode {
                Opcode::Ldc(index) => {
                    self.ldc(index as usize);
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
                Opcode::Swp => {
                    self.swp();
                }
                Opcode::Store(index) => {
                    self.store(index as usize);
                }
                Opcode::Load(index) => {
                    self.load(index as usize);
                }
                Opcode::Goto(index) => {
                    self.goto(index);
                }
                Opcode::Nop => {
                    // Do nothing code
                }
                Opcode::Func(function_name_index, parameter_size) => {
                    self.func(function_name_index, parameter_size);
                }
                Opcode::Return => {
                    return self.stack;
                }
                Opcode::Invoke(function_name_index, parameter_size) => {
                    self.invoke(function_name_index, parameter_size);
                }
            }

            if opcode.enum_index() != 0x0B {
                // Don't move instruction's pos
                self.pos += 1;
            }
        }

        vec![]
    }

    pub fn ldc(&mut self, index: usize) {
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

        let stackable = self.stack.last().unwrap().clone();
        self.stack.push(stackable);
    }

    pub fn swp(&mut self) {
        self.check_stack_size(2);

        if let [top1, top2] = &self.pop(2)[..] {
            self.push(&[top2.clone(), top1.clone()]);
        }
    }

    pub fn store(&mut self, index: usize) {
        self.check_stack_size(1);

        if index <= u16::MAX.into() {
            let stackable = self.stack.pop().unwrap();
            self.local_variable.insert(index as u16, stackable);
        } else {
            panic!("Unable to store variable: index exceeded VM's hard limit.\n    Got: {}\n    Limit: 65535", index);
        }
    }

    pub fn load(&mut self, index: usize) {
        if index <= u16::MAX.into() {
            let stackable = self.local_variable.get(&(index as u16)).unwrap().clone();
            self.stack.push(stackable);
        } else {
            panic!("Unable to load variable: index exceeded VM's hard limit.\n    Got: {}\n    Limit: 65535", index);
        }
    }

    pub fn goto(&mut self, index: u32) {
        self.pos = index;
    }

    pub fn func(&mut self, function_name_index: u32, parameter_size: u8) {
        self.functions.insert(
            FunctionSignature {
                function_name_index,
                parameter_size,
            },
            self.pos + 1,
        );

        let mut func_level = 0;

        self.pos += 1;

        // Set current pos to nearest paired return opcode
        while let Some(opcode) = self.get_instruction() {
            match opcode {
                Opcode::Func(_, _) => {
                    func_level += 1;
                }
                Opcode::Return => {
                    if func_level == 0 {
                        break;
                    } else {
                        func_level -= 1;
                    }
                }
                _ => {}
            }

            self.pos += 1;
        }
    }

    pub fn r#return(&mut self) -> Option<Stackable> {
        return self.stack.pop();
    }

    pub fn invoke(&mut self, function_name_index: u32, parameter_size: u8) {
        let function_initial_pos = self
            .functions
            .get(&FunctionSignature {
                function_name_index,
                parameter_size,
            })
            .cloned();

        if let Some(pos) = function_initial_pos {
            self.check_stack_size(parameter_size as usize);

            let enter_pos = self.pos;
            let parameter_stack = self.pop(parameter_size as usize);

            let proc = self.subprocess(pos, parameter_stack);

            let mut return_value = proc.run();

            self.stack.append(&mut return_value);

            self.pos = enter_pos;
        } else {
            panic!(
                "Unknown function {:?} with {} parameters",
                self.vm
                    .constants
                    .get(function_name_index as usize)
                    .unwrap_or(&Stackable::String("<Unknown function name>".to_string())),
                parameter_size
            );
        }
    }

    fn pop(&mut self, pop_size: usize) -> Vec<Stackable> {
        self.check_stack_size(pop_size);

        self.stack.drain(self.stack.len() - pop_size..).collect()
    }

    fn push(&mut self, items: &[Stackable]) {
        self.stack.extend_from_slice(items);
    }

    fn check_stack_size(&self, required_size: usize) {
        if self.stack.len() < required_size {
            panic!(
                "Unable to perform addition, requires {}+ items on stack but got {}",
                required_size,
                self.stack.len()
            );
        }
    }
}
