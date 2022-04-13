extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
extern crate arrayvec;

pub mod bytecode;
pub mod vm;
pub mod opcode;
pub(crate) mod loader;

pub use loader::Loader;