use std::slice::Iter;

use crate::{vm::{VM, Stackable, Code}, opcode::Opcode};

trait ConvertibleData<const COUNT: usize> {
    fn take_convert<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> Self where Self: Sized {
        let mut container = [0u8; COUNT];
        let sliced_bits = &iter.by_ref()
            .take(COUNT)
            .map(|u| *u)
            .collect::<Vec<u8>>()[..];
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

impl ConvertibleData<8> for u64 {
    fn from_be_bytes(from: [u8; 8]) -> Self {
        u64::from_be_bytes(from)
    }
}

pub struct Loader<'a> {
    bytecode: Iter<'a, u8>
}

impl<'a> Loader<'a> {
    pub fn new(bytecode: &'a Vec<u8>) -> Self {
        Self{
            bytecode: bytecode.iter()
        }
    }

    pub fn load(mut self) -> VM {
        // Validate header first
        self.validate_header();

        // Load constants
        let constant_pool_size = &(self.read_data::<u64, 8>() as usize);
        let mut constants: Vec<Stackable> = Vec::with_capacity(*constant_pool_size);

        for i in 0..*constant_pool_size {
            match self.next() {
                0x00 => {
                    let integer = self.read_data::<i32, 4>();

                    constants.push(Stackable::Int(integer));
                }
                _ => {

                }
            }
        }

        let instructions = Vec::new();

        VM::new_vm(constants, Code::new(instructions))
    }

    fn validate_header(&mut self) {
        let header = &self.bytecode.by_ref().take(8).map(|u| *u).collect::<Vec<u8>>()[..];
        if header != &[0x47, 0x45, 0x41, 0x52, 0x57, 0x4F, 0x52, 0x4B] {
            panic!("Invalid header, should be `GEARWORK` (ascii form), but got `{}` (ascii form)", 
                header.iter()
                    .map(|u| *u as char)
                    .collect::<String>()
            );
        }
    }

    fn next(&mut self) -> &u8 {
        self.bytecode.by_ref().next().unwrap()
    }

    fn read_data<CD, const COUNT: usize>(&mut self) -> CD where CD: ConvertibleData<COUNT> {
        CD::take_convert(&mut self.bytecode)
    }
}