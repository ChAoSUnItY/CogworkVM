use std::{slice::Iter, str};

use crate::{
    opcode::Opcode,
    vm::{Code, Stackable, VM},
};

trait ConvertibleData<const COUNT: usize> {
    fn take_convert<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> Self
    where
        Self: Sized,
    {
        let mut container = [0u8; COUNT];
        let sliced_bits = &iter.by_ref().take(COUNT).map(|u| *u).collect::<Vec<u8>>()[..];
        container.copy_from_slice(sliced_bits);
        Self::from_be_bytes(container)
    }

    fn from_be_bytes(from: [u8; COUNT]) -> Self;
}

impl ConvertibleData<4> for i32 {
    fn from_be_bytes(from: [u8; 4]) -> Self {
        i32::from_be_bytes(from)
    }
}

impl ConvertibleData<8> for i64 {
    fn from_be_bytes(from: [u8; 8]) -> Self {
        i64::from_be_bytes(from)
    }
}

impl ConvertibleData<4> for f32 {
    fn from_be_bytes(from: [u8; 4]) -> Self {
        f32::from_be_bytes(from)
    }
}

impl ConvertibleData<8> for f64 {
    fn from_be_bytes(from: [u8; 8]) -> Self {
        f64::from_be_bytes(from)
    }
}

impl ConvertibleData<1> for u8 {
    fn from_be_bytes(from: [u8; 1]) -> Self {
        u8::from_be_bytes(from)
    }
}

impl ConvertibleData<2> for u16 {
    fn from_be_bytes(from: [u8; 2]) -> Self {
        u16::from_be_bytes(from)
    }
}

impl ConvertibleData<4> for u32 {
    fn from_be_bytes(from: [u8; 4]) -> Self {
        u32::from_be_bytes(from)
    }
}

impl ConvertibleData<8> for u64 {
    fn from_be_bytes(from: [u8; 8]) -> Self {
        u64::from_be_bytes(from)
    }
}

pub struct Loader<'a> {
    bytecode: Iter<'a, u8>,
}

impl<'a> Loader<'a> {
    pub fn new(bytecode: &'a Vec<u8>) -> Self {
        Self {
            bytecode: bytecode.iter(),
        }
    }

    pub fn load(mut self) -> VM {
        // Validate header first
        self.validate_header();

        // Load constants
        let constant_pool_size = self.read_data::<u32, 4>() as usize;
        let mut constants = Vec::with_capacity(constant_pool_size);

        for _ in 0..constant_pool_size {
            match self.next() {
                0x00 => {
                    // Integer constant
                    let integer = self.read_data::<i32, 4>();

                    constants.push(Stackable::Int(integer));
                }
                0x01 => {
                    // Long constant
                    let long = self.read_data::<i64, 8>();

                    constants.push(Stackable::Long(long));
                }
                0x02 => {
                    // Float constant
                    let float = self.read_data::<f32, 4>();

                    constants.push(Stackable::Float(float));
                }
                0x03 => {
                    // Double constant
                    let double = self.read_data::<f64, 8>();

                    constants.push(Stackable::Double(double));
                }
                0x04 => {
                    // String constant
                    let string_size = self.read_data::<u64, 8>() as usize;
                    let string_bytes = self.read(string_size);

                    match str::from_utf8(&string_bytes) {
                        Ok(string) => constants.push(Stackable::String(string.to_string())),
                        Err(err) => panic!("{}", err),
                    }
                }
                tag @ _ => panic!("Unexpected constant tag {}", tag),
            }
        }

        let instructions_size = self.read_data::<u32, 4>() as usize;
        let mut instructions = Vec::with_capacity(instructions_size);

        for _ in 0..instructions_size {
            match self.next() {
                0x00 => {
                    // ldc
                    let index = self.read_data::<u32, 4>();

                    instructions.push(Opcode::Ldc(index));
                }
                0x01 => {
                    // dump
                    instructions.push(Opcode::Dump);
                }
                0x02 => {
                    // add
                    instructions.push(Opcode::Add);
                }
                0x03 => {
                    // sub
                    instructions.push(Opcode::Sub);
                }
                0x04 => {
                    // mul
                    instructions.push(Opcode::Mul);
                }
                0x05 => {
                    // div
                    instructions.push(Opcode::Add);
                }
                0x06 => {
                    // mod
                    instructions.push(Opcode::Div);
                }
                0x07 => {
                    // dup
                    instructions.push(Opcode::Dup);
                }
                0x08 => {
                    // swp
                    instructions.push(Opcode::Swp);
                }
                0x09 => {
                    // store
                    let index = self.read_data::<u16, 2>();

                    instructions.push(Opcode::Store(index));
                }
                0x0A => {
                    // load
                    let index = self.read_data::<u16, 2>();

                    instructions.push(Opcode::Load(index));
                }
                0x0B => {
                    // goto
                    let index = self.read_data::<u32, 4>();

                    instructions.push(Opcode::Goto(index));
                }
                0x0C => {
                    // nop
                    instructions.push(Opcode::Nop);
                }
                0x0D => {
                    // func
                    let function_name_index = self.read_data::<u32, 4>();
                    let parameter_size = self.read_data::<u8, 1>();

                    instructions.push(Opcode::Func(function_name_index, parameter_size));
                }
                0x0E => {
                    // return
                    instructions.push(Opcode::Return);
                }
                0x0F => {
                    // invoke
                    let function_name = self.read_data::<u32, 4>();
                    let parameter_size = self.read_data::<u8, 1>();

                    instructions.push(Opcode::Invoke(function_name, parameter_size));
                }
                opcode @ _ => panic!("Unexpected opcode {:#04X?}", opcode),
            }
        }

        VM::new_vm(constants, Code::new(instructions))
    }

    fn validate_header(&mut self) {
        let header = &self.read(8);
        if header != &[0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B] {
            panic!(
                "Invalid header, should be `GEARWORK` (ascii form), but got `{}` (ascii form)",
                header.iter().map(|u| *u as char).collect::<String>()
            );
        }
    }

    fn next(&mut self) -> &u8 {
        self.bytecode.by_ref().next().unwrap()
    }

    fn read(&mut self, n: usize) -> Vec<u8> {
        self.bytecode.by_ref().take(n).map(|u| *u).collect()
    }

    fn read_data<CD, const COUNT: usize>(&mut self) -> CD
    where
        CD: ConvertibleData<COUNT>,
    {
        CD::take_convert(&mut self.bytecode)
    }
}
