use std::any::Any;

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
/// \[0x04, \[u8; usize\], \[u8; s_size\]\] <-- String constant, bytes at [1..4]/[1..8] indicates string bytes' len </br>
///                                             usize:  32-bits will be 4, 64-bits will be 8 </br>
///                                             s_size: Size of string bytes </br>
/// 
/// ## Code: </br>
/// \[\[u8; 4\], \[u8; c_size\]\] <-- First 4 bytes indicates max stack size (User needs to compute it) </br>
///                                   c_size: Size of instructions </br>
/// 
/// ## Instructions: </br>
/// \[opcode, \[u8; f_size\]\] <-- Instruction, as known as opcode, followed bytes size is based on instruction </br>
///                                f_size: Size of followed bytes, based on instruction </br>
/// 
/// ## Instruction Set: </br>
/// | Opcode name   | Opcode index  | Followed bytes    | Description |
/// |---------------|---------------|-------------------|-------------|
/// | load          | 0x00          | u8, u8, u8, u8    | Loads a constant from constant pool |
pub struct BytecodeBuilder {
    byte_pool: Vec<u8>
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self{
            byte_pool: vec![0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B]
        }
    }

    pub fn visit_constant_pool<'a>(&mut self) -> ConstantBuilder {
        ConstantBuilder{
            parent_builder: self,
            byte_pool: vec![],
        }
    }

    pub fn visit_code<'a>(&mut self) -> InstructionBuilder {
        InstructionBuilder{
            parent_builder: self,
            byte_pool: vec![],
        }
    }

    pub fn visit_end(&self) -> Vec<u8> {
        return self.byte_pool.clone();
    }
}

pub struct ConstantBuilder<'a> {
    parent_builder: &'a mut BytecodeBuilder,
    byte_pool: Vec<u8>
}

impl<'a> ConstantBuilder<'a> {
    pub fn visit_integer(&mut self, int: i32) {
        self.byte_pool.push(0x00);
        self.byte_pool.extend_from_slice(&int.to_be_bytes());
    }

    pub fn visit_long(&mut self, long: i64) {
        self.byte_pool.push(0x01);
        self.byte_pool.extend_from_slice(&long.to_be_bytes());
    }

    pub fn visit_float(&mut self, float: f32) {
        self.byte_pool.push(0x02);
        self.byte_pool.extend_from_slice(&float.to_be_bytes());
    }

    pub fn visit_double(&mut self, double: f64) {
        self.byte_pool.push(0x03);
        self.byte_pool.extend_from_slice(&double.to_be_bytes());
    }

    pub fn visit_string(&mut self, string: String) {
        let string_bytes = string.as_bytes();

        self.byte_pool.push(0x04);
        self.byte_pool.extend_from_slice(&string_bytes.len().to_be_bytes());
        self.byte_pool.extend_from_slice(&string_bytes);
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
        self.parent_builder.byte_pool.append(&mut self.byte_pool);
    }
}

pub struct InstructionBuilder<'a> {
    parent_builder: &'a BytecodeBuilder,
    byte_pool: Vec<u8>,
}

impl<'a> InstructionBuilder<'a> {
    pub fn visit_load(&mut self, index: i32) {
        self.byte_pool.push(0x00);
        self.byte_pool.extend_from_slice(&index.to_be_bytes());
    }
}