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

    pub fn visit_end(mut self) {
        self.parent_builder.byte_pool.append(&mut self.byte_pool);
    }
}