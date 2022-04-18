extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
extern crate arrayvec;

pub mod bytecode;
pub(crate) mod loader;
pub mod opcode;
pub mod vm;

pub use loader::Loader;
