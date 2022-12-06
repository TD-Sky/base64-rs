#![feature(iter_array_chunks)]

type U6 = u8;

pub mod alphabet;
pub mod classic;
pub mod decode;
pub mod encode;
pub mod error;

pub use self::decode::decode;
pub use self::encode::encode;
pub use self::error::Error;
