#[derive(EnumIndex, Debug, Clone, Copy)]
pub enum Opcode {
    Load(u32),          // 0x00
    Dump,               // 0x01
    Add,                // 0x02
    Sub,                // 0x03
    Mul,                // 0x04
    Div,                // 0x05
    Mod,                // 0x06
    Dup,                // 0x07
    Swp,                // 0x08
}