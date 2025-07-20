use std::io::{Read, Write};
use std::mem;

/// The Serialize trait (custom)
pub trait Serialize {
  fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

/// The Deserialize trait (custom)
pub trait Deserialize: Sized {
  fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self>;
}

// Helper macro to write native-endian bytes
macro_rules! impl_for_primitive {
  ($($t:ty),*) => {
    $(
      impl Serialize for $t {
        fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
          let bytes = self.to_le_bytes(); // use little endian for consistency
          writer.write_all(&bytes)
        }
      }

      impl Deserialize for $t {
        fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
          let mut bytes = [0u8; mem::size_of::<$t>()];
          reader.read_exact(&mut bytes)?;
          Ok(<$t>::from_le_bytes(bytes))
        }
      }
    )*
  };
}

// Apply the macro to all integer types
impl_for_primitive!(
  u8, u16, u32, u64, u128, usize,
  i8, i16, i32, i64, i128, isize
);