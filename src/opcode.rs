#[derive(EnumIndex, Debug, Clone, Copy)]
pub enum Opcode {
    Load(u32),          // 0x00
    Dump,               // 0x01
}