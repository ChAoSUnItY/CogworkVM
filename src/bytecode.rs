use std::{any::Any, cell::RefCell};

use super::opcode::Opcode;

/// # Format summary: </br>
/// 
/// ## Overview: </br>
/// Header - Constant Pool - Code </br>
/// 
/// ## Header: </br>
/// \[0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B\]  <-- Magic number: `GEARWORK` </br>
/// 
/// ## Constant Pool: </br>
/// \[\[u8; 4\], \[u8; cp_size\]\] <-- First 4 bytes indicates how many constants </br>
///                                    cp_size: Size of constant pool, based on constant entries </br>
/// 
/// ### Available Constant Formats: </br>
/// \[0x00, \[u8; 4\]\] <-- Integer constant </br>
/// \[0x01, \[u8; 8\]\] <-- Long constant </br>
/// \[0x02, \[u8; 4\]\] <-- Float constant </br>
/// \[0x03, \[u8; 8\]\] <-- Double constant </br>
/// \[0x04, \[u8; 8\], \[u8; s_size\]\] <-- String constant, bytes at [1..4]/[1..8] indicates string bytes' len </br>
///                                         s_size: Size of string bytes </br>
/// 
/// ## Code: </br>
/// \[\[u8; 4\],\[u8; c_size\]\] <-- Represents instructions, the first 4 bytes indicates instruction length.
///                                  c_size: Size of instructions </br>
/// 
/// ## Instructions: </br>
/// \[opcode, \[u8; f_size\]\] <-- Instruction, as known as opcode, followed bytes size is based on instruction </br>
///                                f_size: Size of followed bytes, based on instruction </br>
/// 
/// ## Instruction Set: </br>
/// | Opcode name   | Opcode index  | Followed bytes    | Description | Note |
/// |---------------|---------------|-------------------|-------------|------|
/// | ldc           | 0x00          | u8, u8, u8, u8    | Load a constant from constant pool ||
/// | dump          | 0x01          |                   | Pop and print out top item from stack ||
/// | add           | 0x02          |                   | Consume and add 2 items from stack and push result to stack | Operands must be Int, Long, Float, Double only|
/// | sub           | 0x03          |                   | Consume and subtract 2 items from stack and push result to stack | *Ditto* |
/// | mul           | 0x04          |                   | Consume and multiply 2 items from stack and push result to stack | *Ditto* |
/// | div           | 0x05          |                   | Consume and divide 2 items from stack and push result to stack | *Ditto* |
/// | mod           | 0x06          |                   | Consume and modulo 2 items from stack and push result to stack | *Ditto* |
/// | dup           | 0x07          |                   | Duplicate top item to stack ||
/// | swp           | 0x08          |                   | Swap last top two items from stack ||
/// | store         | 0x09          | u8, u8            | Pop and store top item from stack to local variable ||
/// | load          | 0x0A          | u8, u8            | Load a local variable onto stack ||
/// | goto          | 0x0B          | u8, u8, u8, u8    | Jump to target instruction index ||
/// | nop           | 0x0C          |                   | Do nothing code ||
#[derive(Debug)]
pub struct BytecodeBuilder {
    byte_pool: Vec<u8>,
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self{
            byte_pool: vec![0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B],
        }
    }

    pub fn visit_constant_pool(&mut self) -> ConstantBuilder {
        ConstantBuilder{
            parent_builder: self,
            count: 0,
            byte_pool: vec![],
        }
    }

    pub fn visit_code(&mut self) -> InstructionBuilder {
        InstructionBuilder{
            parent_builder: self,
            labels: vec![],
            byte_pool: vec![],
            pos: 0,
        }
    }

    pub fn visit_end(self) -> Vec<u8> {
        return self.byte_pool;
    }
}

#[derive(Debug)]
pub struct ConstantBuilder<'a> {
    parent_builder: &'a mut BytecodeBuilder,
    count: u32,
    byte_pool: Vec<u8>
}

impl<'a> ConstantBuilder<'a> {
    pub fn visit_integer(&mut self, int: i32) {
        self.byte_pool.push(0x00);
        self.byte_pool.extend_from_slice(&int.to_be_bytes());
        self.count += 1;
    }

    pub fn visit_long(&mut self, long: i64) {
        self.byte_pool.push(0x01);
        self.byte_pool.extend_from_slice(&long.to_be_bytes());
        self.count += 1;
    }

    pub fn visit_float(&mut self, float: f32) {
        self.byte_pool.push(0x02);
        self.byte_pool.extend_from_slice(&float.to_be_bytes());
        self.count += 1;
    }

    pub fn visit_double(&mut self, double: f64) {
        self.byte_pool.push(0x03);
        self.byte_pool.extend_from_slice(&double.to_be_bytes());
        self.count += 1;
    }

    pub fn visit_string(&mut self, string: String) {
        let string_bytes = string.as_bytes();

        self.byte_pool.push(0x04);
        self.byte_pool.extend_from_slice(&string_bytes.len().to_be_bytes());
        self.byte_pool.extend_from_slice(&string_bytes);
        self.count += 1;
    }

    pub fn visit_constant(&mut self, value: &dyn Any) {
        if let Some(int) = value.downcast_ref::<i32>() {
            self.visit_integer(*int);
        } else if let Some(long) = value.downcast_ref::<i64>() {
            self.visit_long(*long);
        } else if let Some(float) = value.downcast_ref::<f32>() {
            self.visit_float(*float);
        } else if let Some(double) = value.downcast_ref::<f64>() {
            self.visit_double(*double);
        } else if let Some(string) = value.downcast_ref::<&str>() {
            self.visit_string(string.to_string());
        } else if let Some(string) = value.downcast_ref::<String>() {
            self.visit_string(string.clone());
        } else {
            panic!("Unexpected constant value. Constant value can only be i32, i64, f32, or f64");
        }
    }

    pub fn visit_end(mut self) {
        self.parent_builder.byte_pool.extend_from_slice(&self.count.to_be_bytes()[..]);
        self.parent_builder.byte_pool.append(&mut self.byte_pool);
    }
}

pub struct InstructionBuilder<'a> {
    parent_builder: &'a mut BytecodeBuilder,
    labels: Vec<(u32, &'a RefCell<Label>)>,
    byte_pool: Vec<u8>,
    pos: u32,
}

impl<'a> InstructionBuilder<'a> {
    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn visit_ldc(&mut self, index: u32) {
        self.byte_pool.push(0x00);
        self.byte_pool.extend_from_slice(&index.to_be_bytes());
        self.advance();
    }

    pub fn visit_dump(&mut self) {
        self.byte_pool.push(0x01);
        self.advance();
    }

    pub fn visit_add(&mut self) {
        self.byte_pool.push(0x02);
        self.advance();
    }

    pub fn visit_sub(&mut self) {
        self.byte_pool.push(0x03);
        self.advance();
    }

    pub fn visit_mul(&mut self) {
        self.byte_pool.push(0x04);
        self.advance();
    }

    pub fn visit_div(&mut self) {
        self.byte_pool.push(0x05);
        self.advance();
    }

    pub fn visit_mod(&mut self) {
        self.byte_pool.push(0x06);
        self.advance();
    }

    pub fn visit_dup(&mut self) {
        self.byte_pool.push(0x07);
        self.advance();
    }

    pub fn visit_swp(&mut self) {
        self.byte_pool.push(0x08);
        self.advance();
    }

    pub fn visit_store(&mut self, index: u16) {
        self.byte_pool.push(0x09);
        self.byte_pool.extend_from_slice(&index.to_be_bytes());
        self.advance();
    }

    pub fn visit_load(&mut self, index: u16) {
        self.byte_pool.push(0x0A);
        self.byte_pool.extend_from_slice(&index.to_be_bytes());
        self.advance();
    }

    pub fn visit_label(&mut self, label: &'a RefCell<Label>) {
        *label.borrow_mut() = Label{
            pos: self.pos,
        }
    }

    pub fn make_label(&self) -> RefCell<Label> {
        RefCell::new(Label {
            pos: 0
        })
    }

    pub fn visit_goto(&mut self, label: &'a RefCell<Label>) {
        self.byte_pool.push(0x0B);
        self.labels.push((self.byte_pool.len() as u32, label));
        self.advance();
    }

    fn visit_goto_labeled(&mut self, label: Label) {
        self.byte_pool.push(0x0B);
        self.byte_pool.extend_from_slice(&label.pos.to_be_bytes());
        self.advance();
    }

    pub fn visit_nop(&mut self) {
        self.byte_pool.push(0x0C);
        self.advance();
    }

    pub fn visit_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Ldc(index) => self.visit_ldc(index),
            Opcode::Dump => self.visit_dump(),
            Opcode::Add => self.visit_add(),
            Opcode::Sub => self.visit_sub(),
            Opcode::Mul => self.visit_mul(),
            Opcode::Div => self.visit_div(),
            Opcode::Mod => self.visit_mod(),
            Opcode::Dup => self.visit_dup(),
            Opcode::Swp => self.visit_swp(),
            Opcode::Store(index) => self.visit_store(index),
            Opcode::Load(index) => self.visit_load(index),
            Opcode::Goto(index) => self.visit_goto_labeled(Label{
                pos: index
            }),
            Opcode::Nop => self.visit_nop(),
        }
    }

    pub fn visit_end(self) {
        let byte_pool = self.byte_pool;
        let mut final_byte_pool = vec![];
        let mut previous_index = 0;
        let labels = self.labels;

        for (index, (start_index, label)) in labels.iter().enumerate() {
            let range = if index == 0 {
                0..*start_index as usize
            } else {
                previous_index..*start_index as usize
            };
            let mut seg = byte_pool[range].to_vec();
            seg.extend_from_slice(&label.to_owned().borrow().pos.to_be_bytes());
            final_byte_pool.extend_from_slice(&seg);
            previous_index = *start_index as usize;
        }

        final_byte_pool.extend_from_slice(&byte_pool[previous_index..]);
        
        self.parent_builder.byte_pool.extend_from_slice(&self.pos.to_be_bytes());
        self.parent_builder.byte_pool.append(&mut final_byte_pool);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Label {
    pos: u32
}