use std;
use std::io::Write;

use bite::BiteWriteExt;
use bite::LittleEndian;


pub trait BiteWriteLeExt: Write {
    #[inline] fn write_u8     (&mut self, value: u8)                 -> Result<(), std::io::Error> { BiteWriteExt::write_u8                      (self, value)        }
    #[inline] fn write_i8     (&mut self, value: i8)                 -> Result<(), std::io::Error> { BiteWriteExt::write_i8                      (self, value)        }
    #[inline] fn write_u16    (&mut self, value: u16)                -> Result<(), std::io::Error> { BiteWriteExt::write_u16     ::<LittleEndian>(self, value)        }
    #[inline] fn write_i16    (&mut self, value: i16)                -> Result<(), std::io::Error> { BiteWriteExt::write_i16     ::<LittleEndian>(self, value)        }
    #[inline] fn write_u32    (&mut self, value: u32)                -> Result<(), std::io::Error> { BiteWriteExt::write_u32     ::<LittleEndian>(self, value)        }
    #[inline] fn write_i32    (&mut self, value: i32)                -> Result<(), std::io::Error> { BiteWriteExt::write_i32     ::<LittleEndian>(self, value)        }
    #[inline] fn write_u64    (&mut self, value: u64)                -> Result<(), std::io::Error> { BiteWriteExt::write_u64     ::<LittleEndian>(self, value)        }
    #[inline] fn write_i64    (&mut self, value: i64)                -> Result<(), std::io::Error> { BiteWriteExt::write_i64     ::<LittleEndian>(self, value)        }
    #[inline] fn write_u128   (&mut self, value: u128)               -> Result<(), std::io::Error> { BiteWriteExt::write_u128    ::<LittleEndian>(self, value)        }
    #[inline] fn write_i128   (&mut self, value: i128)               -> Result<(), std::io::Error> { BiteWriteExt::write_i128    ::<LittleEndian>(self, value)        }
    #[inline] fn write_uint   (&mut self, value: u64,  bytes: usize) -> Result<(), std::io::Error> { BiteWriteExt::write_uint    ::<LittleEndian>(self, value, bytes) }
    #[inline] fn write_int    (&mut self, value: i64,  bytes: usize) -> Result<(), std::io::Error> { BiteWriteExt::write_int     ::<LittleEndian>(self, value, bytes) }
    #[inline] fn write_uint128(&mut self, value: u128, bytes: usize) -> Result<(), std::io::Error> { BiteWriteExt::write_uint128 ::<LittleEndian>(self, value, bytes) }
    #[inline] fn write_int128 (&mut self, value: i128, bytes: usize) -> Result<(), std::io::Error> { BiteWriteExt::write_int128  ::<LittleEndian>(self, value, bytes) }
    #[inline] fn write_f32    (&mut self, value: f32)                -> Result<(), std::io::Error> { BiteWriteExt::write_f32     ::<LittleEndian>(self, value)        }
    #[inline] fn write_f64    (&mut self, value: f64)                -> Result<(), std::io::Error> { BiteWriteExt::write_f64     ::<LittleEndian>(self, value)        }
    #[inline] fn write_framed (&mut self, value: &[u8])              -> Result<(), std::io::Error> { BiteWriteExt::write_framed  ::<LittleEndian>(self, value)        }
}

impl<T> BiteWriteLeExt for T where T: Write + ?Sized { }
