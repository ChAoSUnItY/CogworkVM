use std::any::Any;

use super::opcode::Opcode;

// TODO: Let builder compute max stack & max local

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
/// \[\[u8; 2\], \[u8; 2\], \[u8; 4\], \[u8; c_size\]\] <-- First 2 bytes indicates max stack size, </br>
///                                                         the latter 2 bytes indicates max local variable size, </br>
///                                                         the latter 4 bytes indicates the followed code's instruction size. </br>
///                                                         c_size: Size of instructions </br>
/// 
/// ## Instructions: </br>
/// \[opcode, \[u8; f_size\]\] <-- Instruction, as known as opcode, followed bytes size is based on instruction </br>
///                                f_size: Size of followed bytes, based on instruction </br>
/// 
/// ## Instruction Set: </br>
/// | Opcode name   | Opcode index  | Followed bytes    | Description |
/// |---------------|---------------|-------------------|-------------|
/// | load          | 0x00          | u8, u8, u8, u8    | Loads a constant from constant pool |
/// | dump          | 0x01          |                   | Pop and print out top item from stack |
pub struct BytecodeBuilder {
    byte_pool: Vec<u8>
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self{
            byte_pool: vec![0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B]
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
            max_stack: 0,
            max_local: 0,
            count: 0,
            byte_pool: vec![],
        }
    }

    pub fn visit_end(&self) -> Vec<u8> {
        return self.byte_pool.clone();
    }
}

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
    max_stack: u16,
    max_local: u16,
    count: u32,
    byte_pool: Vec<u8>,
}

impl<'a> InstructionBuilder<'a> {
    pub fn visit_load(&mut self, index: u32) {
        self.byte_pool.push(0x00);
        self.byte_pool.extend_from_slice(&index.to_be_bytes());
        self.count += 1;
    }

    pub fn visit_dump(&mut self) {
        self.byte_pool.push(0x01);
        self.count += 1;
    }

    pub fn visit_add(&mut self) {
        self.byte_pool.push(0x02);
        self.count += 1;
    }

    pub fn visit_sub(&mut self) {
        self.byte_pool.push(0x03);
        self.count += 1;
    }

    pub fn visit_mul(&mut self) {
        self.byte_pool.push(0x04);
        self.count += 1;
    }

    pub fn visit_div(&mut self) {
        self.byte_pool.push(0x05);
        self.count += 1;
    }

    pub fn visit_mod(&mut self) {
        self.byte_pool.push(0x06);
        self.count += 1;
    }

    pub fn visit_dup(&mut self) {
        self.byte_pool.push(0x07);
        self.count += 1;
    }

    pub fn visit_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Load(index) => self.visit_load(index),
            Opcode::Dump => self.visit_dump(),
            Opcode::Add => self.visit_add(),
            Opcode::Sub => self.visit_sub(),
            Opcode::Mul => self.visit_mul(),
            Opcode::Div => self.visit_div(),
            Opcode::Mod => self.visit_mod(),
            Opcode::Dup => self.visit_dup(),
        }
    }

    pub fn visit_max(&mut self, max_stack: u16, max_local: u16) {
        self.max_stack = max_stack;
        self.max_local = max_local;
    }

    pub fn visit_end(mut self) {
        self.parent_builder.byte_pool.extend_from_slice(&self.max_stack.to_be_bytes());
        self.parent_builder.byte_pool.extend_from_slice(&self.max_local.to_be_bytes());
        self.parent_builder.byte_pool.extend_from_slice(&self.count.to_be_bytes());
        self.parent_builder.byte_pool.append(&mut self.byte_pool);
    }
}