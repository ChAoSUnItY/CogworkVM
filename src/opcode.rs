#[derive(EnumIndex, Debug, Clone, Copy)]
pub enum Opcode {
    Ldc(u32),           // 0x00
    Dump,               // 0x01
    Add,                // 0x02
    Sub,                // 0x03
    Mul,                // 0x04
    Div,                // 0x05
    Mod,                // 0x06
    Dup,                // 0x07
    Swp,                // 0x08
    Store(u16),         // 0x09
    Load(u16),          // 0x0A
    Goto(u32),          // 0x0B
    Nop,                // 0x0C
    Func(u32, u8, u8),  // 0x0D
    Return,             // 0x0E
}