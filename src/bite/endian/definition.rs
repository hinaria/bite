#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian { }

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian { }

pub type NetworkEndian = BigEndian;

#[cfg(target_endian = "big")]    pub type NativeEndian = BigEndian;
#[cfg(target_endian = "little")] pub type NativeEndian = LittleEndian;
