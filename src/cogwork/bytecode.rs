use std::{any::Any, fmt::Debug};

/// Format summary:
/// 
/// Header:
/// \[0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B\]  <-- Magic number: `GEARWORK`
/// 
/// Constant Pool:
/// \[u8; cp_size\] <-- cp_size: Size of constant pool, based on constant entries
/// 
/// Available Constant Formats:
/// \[0x00, \[u8; 4\]\] <-- Integer constants
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

    pub fn visit_constant(&mut self, value: &dyn Any) {
        if let Some(int) = value.downcast_ref::<i32>() {
            self.visit_integer(*int);
        } else if let Some(long) = value.downcast_ref::<i64>() {
            self.visit_long(*long);
        } else if let Some(float) = value.downcast_ref::<f32>() {
            self.visit_float(*float);
        } else if let Some(double) = value.downcast_ref::<f64>() {
            self.visit_double(*double);
        } else {
            panic!("Unexpected constant value. Constant value can only be i32, i64, f32, or f64");
        }
    }

    pub fn visit_end(mut self) {
        self.parent_builder.byte_pool.append(&mut self.byte_pool);
    }
}