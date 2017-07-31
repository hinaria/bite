use std::fmt::Debug;
use std::hash::Hash;
use std::mem::transmute;


pub trait Endianness: Clone + Copy + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
    fn read_u16     (data: &[u8])               -> u16;
    fn read_u32     (data: &[u8])               -> u32;
    fn read_u64     (data: &[u8])               -> u64;
    fn read_u128    (data: &[u8])               -> u128;
    fn read_uint    (data: &[u8], bytes: usize) -> u64;
    fn read_uint128 (data: &[u8], bytes: usize) -> u128;

    fn write_u16    (data: &mut [u8], value: u16);
    fn write_u32    (data: &mut [u8], value: u32);
    fn write_u64    (data: &mut [u8], value: u64);
    fn write_u128   (data: &mut [u8], value: u128);
    fn write_uint   (data: &mut [u8], value: u64, bytes: usize);
    fn write_uint128(data: &mut [u8], value: u128, bytes: usize);


    #[inline] fn read_i16    (data: &[u8])               -> i16           { Self::read_u16(data)  as i16                                     }
    #[inline] fn read_i32    (data: &[u8])               -> i32           { Self::read_u32(data)  as i32                                     }
    #[inline] fn read_i64    (data: &[u8])               -> i64           { Self::read_u64(data)  as i64                                     }
    #[inline] fn read_i128   (data: &[u8])               -> i128          { Self::read_u128(data) as i128                                    }
    #[inline] fn read_int    (data: &[u8], bytes: usize) -> i64           { extend_sign(Self::read_uint(data, bytes), bytes)                 }
    #[inline] fn read_int128 (data: &[u8], bytes: usize) -> i128          { extend_sign128(Self::read_uint128(data, bytes), bytes)           }
    #[inline] fn read_f32    (data: &[u8])               -> f32           { unsafe { transmute(Self::read_u32(data)) }                       }
    #[inline] fn read_f64    (data: &[u8])               -> f64           { unsafe { transmute(Self::read_u64(data)) }                       }

    #[inline] fn write_i16   (data: &mut [u8], value: i16)                { Self::write_u16    (data, value as u16)                          }
    #[inline] fn write_i32   (data: &mut [u8], value: i32)                { Self::write_u32    (data, value as u32)                          }
    #[inline] fn write_i64   (data: &mut [u8], value: i64)                { Self::write_u64    (data, value as u64)                          }
    #[inline] fn write_i128  (data: &mut [u8], value: i128)               { Self::write_u128   (data, value as u128)                         }
    #[inline] fn write_int   (data: &mut [u8], value: i64,  bytes: usize) { Self::write_uint   (data, unextend_sign(value, bytes), bytes)    }
    #[inline] fn write_int128(data: &mut [u8], value: i128, bytes: usize) { Self::write_uint128(data, unextend_sign128(value, bytes), bytes) }
    #[inline] fn write_f32   (data: &mut [u8], value: f32)                { Self::write_u32    (data, unsafe { transmute(value) })           }
    #[inline] fn write_f64   (data: &mut [u8], value: f64)                { Self::write_u64    (data, unsafe { transmute(value) })           }
}



#[inline]
fn extend_sign(val: u64, nbytes: usize) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

#[inline]
fn extend_sign128(val: u128, nbytes: usize) -> i128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as i128 >> shift
}

#[inline]
fn unextend_sign(val: i64, nbytes: usize) -> u64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as u64 >> shift
}

#[inline]
fn unextend_sign128(val: i128, nbytes: usize) -> u128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as u128 >> shift
}
