use std;
use std::io::Read;

use bite::BiteReadExt;
use bite::LittleEndian;


pub trait BiteReadLeExt: Read {
    #[inline] fn read_u8        (&mut self)                 -> Result<    u8,  std::io::Error> { BiteReadExt::read_u8                        (self)          }
    #[inline] fn read_i8        (&mut self)                 -> Result<    i8,  std::io::Error> { BiteReadExt::read_i8                        (self)          }
    #[inline] fn read_u16       (&mut self)                 -> Result<   u16,  std::io::Error> { BiteReadExt::read_u16       ::<LittleEndian>(self)          }
    #[inline] fn read_i16       (&mut self)                 -> Result<   i16,  std::io::Error> { BiteReadExt::read_i16       ::<LittleEndian>(self)          }
    #[inline] fn read_u32       (&mut self)                 -> Result<   u32,  std::io::Error> { BiteReadExt::read_u32       ::<LittleEndian>(self)          }
    #[inline] fn read_i32       (&mut self)                 -> Result<   i32,  std::io::Error> { BiteReadExt::read_i32       ::<LittleEndian>(self)          }
    #[inline] fn read_u64       (&mut self)                 -> Result<   u64,  std::io::Error> { BiteReadExt::read_u64       ::<LittleEndian>(self)          }
    #[inline] fn read_i64       (&mut self)                 -> Result<   i64,  std::io::Error> { BiteReadExt::read_i64       ::<LittleEndian>(self)          }
    #[inline] fn read_u128      (&mut self)                 -> Result<  u128,  std::io::Error> { BiteReadExt::read_u128      ::<LittleEndian>(self)          }
    #[inline] fn read_i128      (&mut self)                 -> Result<  i128,  std::io::Error> { BiteReadExt::read_i128      ::<LittleEndian>(self)          }
    #[inline] fn read_uint      (&mut self, bytes: usize)   -> Result<   u64,  std::io::Error> { BiteReadExt::read_uint      ::<LittleEndian>(self, bytes)   }
    #[inline] fn read_int       (&mut self, bytes: usize)   -> Result<   i64,  std::io::Error> { BiteReadExt::read_int       ::<LittleEndian>(self, bytes)   }
    #[inline] fn read_uint128   (&mut self, bytes: usize)   -> Result<  u128,  std::io::Error> { BiteReadExt::read_uint128   ::<LittleEndian>(self, bytes)   }
    #[inline] fn read_int128    (&mut self, bytes: usize)   -> Result<  i128,  std::io::Error> { BiteReadExt::read_int128    ::<LittleEndian>(self, bytes)   }
    #[inline] fn read_f32       (&mut self)                 -> Result<   f32,  std::io::Error> { BiteReadExt::read_f32       ::<LittleEndian>(self)          }
    #[inline] fn read_f64       (&mut self)                 -> Result<   f64,  std::io::Error> { BiteReadExt::read_f64       ::<LittleEndian>(self)          }
    #[inline] fn read_framed    (&mut self)                 -> Result<Vec<u8>, std::io::Error> { BiteReadExt::read_framed    ::<LittleEndian>(self)          }
    #[inline] fn read_framed_max(&mut self, maximum: usize) -> Result<Vec<u8>, std::io::Error> { BiteReadExt::read_framed_max::<LittleEndian>(self, maximum) }
}

impl<T> BiteReadLeExt for T where T: Read + ?Sized { }
