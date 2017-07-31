use std;
use std::io::Read;

use bite::Endianness;


pub trait BiteReadExt: Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut data = [0; 1];
        self.read_exact(&mut data)?;

        let v = data[0];
        Ok(v)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, std::io::Error> {
        let mut data = [0; 1];
        self.read_exact(&mut data)?;

        let v = data[0] as i8;
        Ok(v)
    }

    #[inline]
    fn read_u16<T: Endianness>(&mut self) -> Result<u16, std::io::Error> {
        let mut data = [0; 2];
        self.read_exact(&mut data)?;

        let v = T::read_u16(&data);
        Ok(v)
    }

    #[inline]
    fn read_i16<T: Endianness>(&mut self) -> Result<i16, std::io::Error> {
        let mut data = [0; 2];
        self.read_exact(&mut data)?;

        let v = T::read_i16(&data);
        Ok(v)
    }

    #[inline]
    fn read_u32<T: Endianness>(&mut self) -> Result<u32, std::io::Error> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;

        let v = T::read_u32(&data);
        Ok(v)
    }

    #[inline]
    fn read_i32<T: Endianness>(&mut self) -> Result<i32, std::io::Error> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;

        let v = T::read_i32(&data);
        Ok(v)
    }

    #[inline]
    fn read_u64<T: Endianness>(&mut self) -> Result<u64, std::io::Error> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;

        let v = T::read_u64(&data);
        Ok(v)
    }

    #[inline]
    fn read_i64<T: Endianness>(&mut self) -> Result<i64, std::io::Error> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;

        let v = T::read_i64(&data);
        Ok(v)
    }

    #[inline]
    fn read_u128<T: Endianness>(&mut self) -> Result<u128, std::io::Error> {
        let mut data = [0; 16];
        self.read_exact(&mut data)?;

        let v = T::read_u128(&data);
        Ok(v)
    }

    #[inline]
    fn read_i128<T: Endianness>(&mut self) -> Result<i128, std::io::Error> {
        let mut data = [0; 16];
        self.read_exact(&mut data)?;

        let v = T::read_i128(&data);
        Ok(v)
    }

    #[inline]
    fn read_uint<T: Endianness>(&mut self, bytes: usize) -> Result<u64, std::io::Error> {
        let mut data = [0; 8];
        self.read_exact(&mut data[..bytes])?;

        let v = T::read_uint(&data[..bytes], bytes);
        Ok(v)
    }

    #[inline]
    fn read_int<T: Endianness>(&mut self, bytes: usize) -> Result<i64, std::io::Error> {
        let mut data = [0; 8];
        self.read_exact(&mut data[..bytes])?;

        let v = T::read_int(&data[..bytes], bytes);
        Ok(v)
    }

    #[inline]
    fn read_uint128<T: Endianness>(&mut self, bytes: usize) -> Result<u128, std::io::Error> {
        let mut data = [0; 16];
        self.read_exact(&mut data[..bytes])?;

        let v = T::read_uint128(&data[..bytes], bytes);
        Ok(v)
    }

    #[inline]
    fn read_int128<T: Endianness>(&mut self, bytes: usize) -> Result<i128, std::io::Error> {
        let mut data = [0; 16];
        self.read_exact(&mut data[..bytes])?;

        let v = T::read_int128(&data[..bytes], bytes);
        Ok(v)
    }

    #[inline]
    fn read_f32<T: Endianness>(&mut self) -> Result<f32, std::io::Error> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;

        let v = T::read_f32(&data);
        Ok(v)
    }

    #[inline]
    fn read_f64<T: Endianness>(&mut self) -> Result<f64, std::io::Error> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;

        let v = T::read_f64(&data);
        Ok(v)
    }

    #[inline]
    fn read_framed<T: Endianness>(&mut self) -> Result<Vec<u8>, std::io::Error> {
        self.read_framed_max::<T>(std::usize::MAX)
    }

    #[inline]
    fn read_framed_max<T: Endianness>(&mut self, maximum: usize) -> Result<Vec<u8>, std::io::Error> {
        let length = match self.read_u32::<T>()? as usize {
            x if x <= maximum => x,
            _                 => return Err(std::io::ErrorKind::InvalidData.into()),
        };

        unsafe {
            let mut data = Vec::with_capacity(length);
            let slice    = std::slice::from_raw_parts_mut(data.as_mut_ptr(), length);

            self.read_exact(slice)?;
            data.set_len(length);

            Ok(data)
        }
    }
}

impl<T> BiteReadExt for T where T: std::io::Read + ?Sized { }
