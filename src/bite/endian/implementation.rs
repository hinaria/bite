use std;

use bite::BigEndian;
use bite::Endianness;
use bite::LittleEndian;


macro_rules! read_num_bytes {
    ($ty: ty, $length: expr, $source: expr, $convert: ident) => ({
        assert!($length == std::mem::size_of::<$ty>());
        assert!($length <= $source.len());

        let mut data: $ty = 0;

        unsafe {
            std::ptr::copy_nonoverlapping(
                $source.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $length);
        }

        data.$convert()
    });
}

macro_rules! write_num_bytes {
    ($ty: ty, $length: expr, $value: expr, $destination: expr, $convert: ident) => ({
        assert!($length <= $destination.len());

        unsafe {
            let bytes = std::mem::transmute::<_, [u8; $length]>($value.$convert());
            std::ptr::copy_nonoverlapping((&bytes).as_ptr(), $destination.as_mut_ptr(), $length);
        }
    });
}


impl Endianness for BigEndian {
    #[inline] fn read_u16 (data: &[u8]) -> u16  { read_num_bytes!(u16, 2, data, to_be)   }
    #[inline] fn read_u32 (data: &[u8]) -> u32  { read_num_bytes!(u32, 4, data, to_be)   }
    #[inline] fn read_u64 (data: &[u8]) -> u64  { read_num_bytes!(u64, 8, data, to_be)   }
    #[inline] fn read_u128(data: &[u8]) -> u128 { read_num_bytes!(u128, 16, data, to_be) }

    #[inline]
    fn read_uint(data: &[u8], bytes: usize) -> u64 {
        assert!(
            true
            && bytes >= 1
            && bytes <= 8
            && bytes <= data.len());

        let mut temp = [0u8; 8];
        let temp     = temp.as_mut_ptr();

        unsafe {
            let offset = (8 - bytes) as isize;
            std::ptr::copy_nonoverlapping(data.as_ptr(), temp.offset(offset), bytes);
            let value  = *(temp as *const u64);

            value.to_be()
        }
    }

    #[inline]
    fn read_uint128(data: &[u8], bytes: usize) -> u128 {
        assert!(
            true
            && bytes >= 1
            && bytes <= 16
            && bytes <= data.len());

        let mut temp = [0u8; 16];
        let temp     = temp.as_mut_ptr();

        unsafe {
            let offset = (16 - bytes) as isize;
            std::ptr::copy_nonoverlapping(data.as_ptr(), temp.offset(offset), bytes);
            let value  = *(temp as *const u128);

            value.to_be()
        }
    }

    #[inline] fn write_u16 (data: &mut [u8], value: u16)  { write_num_bytes!(u16, 2, value, data, to_be);   }
    #[inline] fn write_u32 (data: &mut [u8], value: u32)  { write_num_bytes!(u32, 4, value, data, to_be);   }
    #[inline] fn write_u64 (data: &mut [u8], value: u64)  { write_num_bytes!(u64, 8, value, data, to_be);   }
    #[inline] fn write_u128(data: &mut [u8], value: u128) { write_num_bytes!(u128, 16, value, data, to_be); }

    #[inline]
    fn write_uint(data: &mut [u8], value: u64, bytes: usize) {
        assert!(
            true
            && bytes >= pack_size(value)
            && bytes <= 8
            && bytes <= data.len());

        unsafe {
            let source: [u8; 8] = std::mem::transmute(value.to_be());
            let offset          = (8 - bytes) as isize;

            std::ptr::copy_nonoverlapping(source.as_ptr().offset(offset), data.as_mut_ptr(), bytes);
        }
    }

    #[inline]
    fn write_uint128(data: &mut [u8], value: u128, bytes: usize) {
        assert!(
            true
            && bytes >= pack_size128(value)
            && bytes <= 16
            && bytes <= data.len());

        unsafe {
            let source: [u8; 16] = std::mem::transmute(value.to_be());
            let offset           = (16 - bytes) as isize;

            std::ptr::copy_nonoverlapping(source.as_ptr().offset(offset), data.as_mut_ptr(), bytes);
        }
    }
}


impl Endianness for LittleEndian {
    #[inline] fn read_u16 (data: &[u8]) -> u16  { read_num_bytes!(u16, 2, data, to_le)   }
    #[inline] fn read_u32 (data: &[u8]) -> u32  { read_num_bytes!(u32, 4, data, to_le)   }
    #[inline] fn read_u64 (data: &[u8]) -> u64  { read_num_bytes!(u64, 8, data, to_le)   }
    #[inline] fn read_u128(data: &[u8]) -> u128 { read_num_bytes!(u128, 16, data, to_le) }

    #[inline]
    fn read_uint(data: &[u8], bytes: usize) -> u64 {
        assert!(
            true
            && bytes >= 1
            && bytes <= 8
            && bytes <= data.len());

        let mut temp = [0u8; 8];
        let temp     = temp.as_mut_ptr();

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), temp, bytes);
            let value = *(temp as *const u64);

            value.to_le()
        }
    }

    #[inline]
    fn read_uint128(data: &[u8], bytes: usize) -> u128 {
        assert!(
            true
            && bytes >= 1
            && bytes <= 16
            && bytes <= data.len());

        let mut temp = [0u8; 16];
        let temp     = temp.as_mut_ptr();

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), temp, bytes);
            let value = *(temp as *const u128);

            value.to_le()
        }
    }

    #[inline] fn write_u16 (data: &mut [u8], value: u16)  { write_num_bytes!(u16, 2, value, data, to_le);   }
    #[inline] fn write_u32 (data: &mut [u8], value: u32)  { write_num_bytes!(u32, 4, value, data, to_le);   }
    #[inline] fn write_u64 (data: &mut [u8], value: u64)  { write_num_bytes!(u64, 8, value, data, to_le);   }
    #[inline] fn write_u128(data: &mut [u8], value: u128) { write_num_bytes!(u128, 16, value, data, to_le); }

    #[inline]
    fn write_uint(data: &mut [u8], value: u64, bytes: usize) {
        assert!(
            true
            && bytes >= pack_size(value)
            && bytes <= 8
            && bytes <= data.len());

        unsafe {
            let source: [u8; 8] = std::mem::transmute(value.to_le());

            std::ptr::copy_nonoverlapping(source.as_ptr(), data.as_mut_ptr(), bytes);
        }
    }

    #[inline]
    fn write_uint128(data: &mut [u8], value: u128, bytes: usize) {
        assert!(
            true
            && bytes >= pack_size128(value)
            && bytes <= 16
            && bytes <= data.len());

        unsafe {
            let source: [u8; 16] = std::mem::transmute(value.to_le());

            std::ptr::copy_nonoverlapping(source.as_ptr(), data.as_mut_ptr(), bytes);
        }
    }
}



#[inline]
fn pack_size(n: u64) -> usize {
    match true {
        _ if n < 1 << 8  => 1,
        _ if n < 1 << 16 => 2,
        _ if n < 1 << 24 => 3,
        _ if n < 1 << 32 => 4,
        _ if n < 1 << 40 => 5,
        _ if n < 1 << 48 => 6,
        _ if n < 1 << 56 => 7,
        _                => 8,
    }
}

#[inline]
fn pack_size128(n: u128) -> usize {
    match true {
        _ if n < 1 <<   8 => 1,
        _ if n < 1 <<  16 => 2,
        _ if n < 1 <<  24 => 3,
        _ if n < 1 <<  32 => 4,
        _ if n < 1 <<  40 => 5,
        _ if n < 1 <<  48 => 6,
        _ if n < 1 <<  56 => 7,
        _ if n < 1 <<  64 => 8,
        _ if n < 1 <<  72 => 9,
        _ if n < 1 <<  80 => 10,
        _ if n < 1 <<  88 => 11,
        _ if n < 1 <<  96 => 12,
        _ if n < 1 << 104 => 13,
        _ if n < 1 << 112 => 14,
        _ if n < 1 << 120 => 15,
        _                 => 16,
    }
}
