use std::array::TryFromSliceError;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
pub enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if let (Ok(red), Ok(green), Ok(blue))
            = (u8::try_from(tuple.0), u8::try_from(tuple.1), u8::try_from(tuple.2)) {
            Result::Ok(Color { red, green, blue })
        } else {
            Result::Err(IntoColorError::IntConversion)
        }

    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        if (arr.len() != 3) {
            return Result::Err(IntoColorError::BadLen)
        }
        if let (Some(&red), Some(&green), Some(&blue)) = (arr.get(0), arr.get(1), arr.get(2)) {
            Color::try_from((red, green, blue))
        } else {
            Result::Err(IntoColorError::IntConversion)
        }
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        match TryInto::<[i16; 3]>::try_into(slice) {
            Ok(arr) => Color::try_from(arr),
            Err(_) => Result::Err(IntoColorError::BadLen)
        }
    }
}


