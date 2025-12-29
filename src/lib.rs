//!/////// START OF FILE //////////
//! better-conversions ///

pub mod prelude;

// ERROR //
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    Error(String, String),
}
impl ConversionError {
    pub(crate) fn error<F: Into<String>, M: Into<String>>(from: F, msg: M) -> Self {
        return Self::Error(from.into(), msg.into());
    }
}
// ERROR //

// TRAITS //
pub trait GoToU8 {fn goto_u8(&self) -> u8;}
pub trait GoToU16 {fn goto_u16(&self) -> u16;}
pub trait GoToU32 {fn goto_u32(&self) -> u32;}
pub trait GoToU64 {fn goto_u64(&self) -> u64;}
pub trait GoToU128 {fn goto_u128(&self) -> u128;}
pub trait GoToUsize {fn goto_usize(&self) -> usize;}
pub trait GoToF32 {fn goto_f32(&self) -> f32;}
pub trait GoToF64 {fn goto_f64(&self) -> f64;}
pub trait TryGoToU8 {fn try_goto_u8(&self) -> Result<u8, ConversionError>;}
pub trait TryGoToU16 {fn try_goto_u16(&self) -> Result<u16, ConversionError>;}
pub trait TryGoToU32 {fn try_goto_u32(&self) -> Result<u32, ConversionError>;}
pub trait TryGoToU64 {fn try_goto_u64(&self) -> Result<u64, ConversionError>;}
pub trait TryGoToU128 {fn try_goto_u128(&self) -> Result<u128, ConversionError>;}
pub trait TryGoToUsize {fn try_goto_usize(&self) -> Result<usize, ConversionError>;}
pub trait TryGoToF32 {fn try_goto_f32(&self) -> Result<f32, ConversionError>;}
pub trait TryGoToF64 {fn try_goto_f64(&self) -> Result<f64, ConversionError>;}
// TRAITS //

// U8 //
impl GoToU8 for u8 {fn goto_u8(&self) -> u8 {*self as u8}}
impl GoToU16 for u8 {fn goto_u16(&self) -> u16 {*self as u16}}
impl GoToU32 for u8 {fn goto_u32(&self) -> u32 {*self as u32}}
impl GoToU64 for u8 {fn goto_u64(&self) -> u64 {*self as u64}}
impl GoToU128 for u8 {fn goto_u128(&self) -> u128 {*self as u128}}
impl GoToUsize for u8 {fn goto_usize(&self) -> usize {*self as usize}}
impl GoToF32 for u8 {fn goto_f32(&self) -> f32 {*self as f32}}
impl GoToF64 for u8 {fn goto_f64(&self) -> f64 {*self as f64}}
// U8 //

// U16 //
impl TryGoToU8 for u16 { 
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if *self <= u8::MAX as u16 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl GoToU16 for u16 {fn goto_u16(&self) -> u16 {*self as u16}}
impl GoToU32 for u16 {fn goto_u32(&self) -> u32 {*self as u32}}
impl GoToU64 for u16 {fn goto_u64(&self) -> u64 {*self as u64}}
impl GoToU128 for u16 {fn goto_u128(&self) -> u128 {*self as u128}}
impl GoToUsize for u16 {fn goto_usize(&self) -> usize {*self as usize}}
impl GoToF32 for u16 {fn goto_f32(&self) -> f32 {*self as f32}}
impl GoToF64 for u16 {fn goto_f64(&self) -> f64 {*self as f64}}
// U16 //

// U32 //
impl TryGoToU8 for u32 { 
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if *self <= u8::MAX as u32 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl TryGoToU16 for u32 { 
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if *self <= u16::MAX as u32 {Ok(*self as u16)}
        else {Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self)))}
    }
}
impl GoToU32 for u32 {fn goto_u32(&self) -> u32 {*self as u32}}
impl GoToU64 for u32 {fn goto_u64(&self) -> u64 {*self as u64}}
impl GoToU128 for u32 {fn goto_u128(&self) -> u128 {*self as u128}}
impl GoToUsize for u32 {fn goto_usize(&self) -> usize {*self as usize}}
impl TryGoToF32 for u32 { 
    fn try_goto_f32(&self) -> Result<f32, ConversionError> {
        let converted = *self as f32;
        if converted as u32 == *self {Ok(converted)} 
        else {Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} lost precision as a f32", self)))}
    }
}
impl GoToF64 for u32 {fn goto_f64(&self) -> f64 {*self as f64}}
// U32 //

// U64 //
impl TryGoToU8 for u64 { 
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if *self <= u8::MAX as u64 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl TryGoToU16 for u64 { 
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if *self <= u16::MAX as u64 {Ok(*self as u16)}
        else {Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self)))}
    }
}
impl TryGoToU32 for u64 { 
    fn try_goto_u32(&self) -> Result<u32, ConversionError> {
        if *self <= u32::MAX as u64 {Ok(*self as u32)}
        else {Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} out of u32 range", self)))}
    }
}
impl GoToU64 for u64 {fn goto_u64(&self) -> u64 {*self as u64}}
impl GoToU128 for u64 {fn goto_u128(&self) -> u128 {*self as u128}}
impl TryGoToUsize for u64 { 
    fn try_goto_usize(&self) -> Result<usize, ConversionError> {
        if *self <= usize::MAX as u64 {Ok(*self as usize)}
        else {Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} out of usize range", self)))}
    }
}
impl TryGoToF32 for u64 { 
    fn try_goto_f32(&self) -> Result<f32, ConversionError> {
        let converted = *self as f32;
        if converted as u64 == *self {Ok(converted)} 
        else {Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} lost precision as a f32", self)))}
    }
}
impl TryGoToF64 for u64 { 
    fn try_goto_f64(&self) -> Result<f64, ConversionError> {
        let converted = *self as f64;
        if converted as u64 == *self {Ok(converted)} 
        else {Err(ConversionError::error("better-conversions::TryGoToF64::try_goto_f64", format!("{} lost precision as a f64", self)))}
    }
}
// U64 //

// U128 //
impl TryGoToU8 for u128 { 
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if *self <= u8::MAX as u128 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl TryGoToU16 for u128 { 
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if *self <= u16::MAX as u128 {Ok(*self as u16)}
        else {Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self)))}
    }
}
impl TryGoToU32 for u128 { 
    fn try_goto_u32(&self) -> Result<u32, ConversionError> {
        if *self <= u32::MAX as u128 {Ok(*self as u32)}
        else {Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} out of u32 range", self)))}
    }
}
impl TryGoToU64 for u128 { 
    fn try_goto_u64(&self) -> Result<u64, ConversionError> {
        if *self <= u64::MAX as u128 {Ok(*self as u64)}
        else {Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} out of u64 range", self)))}
    }
}
impl GoToU128 for u128 {fn goto_u128(&self) -> u128 {*self as u128}}
impl TryGoToUsize for u128 { 
    fn try_goto_usize(&self) -> Result<usize, ConversionError> {
        if *self <= usize::MAX as u128 {Ok(*self as usize)}
        else {Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} out of usize range", self)))}
    }
}
impl TryGoToF32 for u128 { 
    fn try_goto_f32(&self) -> Result<f32, ConversionError> {
        let converted = *self as f32;
        if converted as u128 == *self {Ok(converted)} 
        else {Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} lost precision as a f32", self)))}
    }
}
impl TryGoToF64 for u128 { 
    fn try_goto_f64(&self) -> Result<f64, ConversionError> {
        let converted = *self as f64;
        if converted as u128 == *self {Ok(converted)} 
        else {Err(ConversionError::error("better-conversions::TryGoToF64::try_goto_f64", format!("{} lost precision as a f64", self)))}
    }
}
// U128 //

// usize //
impl TryGoToU8 for usize {
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if *self <= u8::MAX as usize { Ok(*self as u8) }
        else { Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self))) }
    }
}
impl TryGoToU16 for usize {
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if *self <= u16::MAX as usize { Ok(*self as u16) }
        else { Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self))) }
    }
}
impl TryGoToU32 for usize {
    fn try_goto_u32(&self) -> Result<u32, ConversionError> {
        if *self <= u32::MAX as usize { Ok(*self as u32) }
        else { Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} out of u32 range", self))) }
    }
}
impl TryGoToU64 for usize {
    fn try_goto_u64(&self) -> Result<u64, ConversionError> {
        if *self <= u64::MAX as usize { Ok(*self as u64) }
        else { Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} out of u64 range", self))) }
    }
}
impl GoToU128 for usize { fn goto_u128(&self) -> u128 { *self as u128 } }
impl GoToUsize for usize { fn goto_usize(&self) -> usize { *self } }
impl TryGoToF32 for usize {
    fn try_goto_f32(&self) -> Result<f32, ConversionError> {
        let converted = *self as f32;
        if converted as usize == *self { Ok(converted) }
        else { Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} lost precision as a f32", self))) }
    }
}
impl TryGoToF64 for usize {
    fn try_goto_f64(&self) -> Result<f64, ConversionError> {
        let converted = *self as f64;
        if converted as usize == *self { Ok(converted) }
        else { Err(ConversionError::error("better-conversions::TryGoToF64::try_goto_f64", format!("{} lost precision as a f64", self))) }
    }
}
// usize //

// f32 //
impl TryGoToU8 for f32 {
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u8::MAX as f32 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl TryGoToU16 for f32 {
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u16::MAX as f32 {Ok(*self as u16)}
        else {Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self)))}
    }
}
impl TryGoToU32 for f32 {
    fn try_goto_u32(&self) -> Result<u32, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u32::MAX as f32 {Ok(*self as u32)}
        else {Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} out of u32 range", self)))}
    }
}
impl TryGoToU64 for f32 {
    fn try_goto_u64(&self) -> Result<u64, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u64::MAX as f32 {Ok(*self as u64)}
        else {Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} out of u64 range", self)))}
    }
}
impl TryGoToU128 for f32 {
    fn try_goto_u128(&self) -> Result<u128, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u128::MAX as f32 {Ok(*self as u128)}
        else {Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} out of u128 range", self)))}
    }
}
impl TryGoToUsize for f32 {
    fn try_goto_usize(&self) -> Result<usize, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} has a fractional", self)));
        }
        if *self >= 0.0 && *self <= usize::MAX as f32 {Ok(*self as usize)} 
        else {Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} out of usize range", self)))}
    }
}
impl GoToF32 for f32 {fn goto_f32(&self) -> f32 {*self as f32}}
impl GoToF64 for f32 {fn goto_f64(&self) -> f64 {*self as f64}}
// f32 //

// f64 //
impl TryGoToU8 for f64 {
    fn try_goto_u8(&self) -> Result<u8, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u8::MAX as f64 {Ok(*self as u8)}
        else {Err(ConversionError::error("better-conversions::TryGoToU8::try_goto_u8", format!("{} out of u8 range", self)))}
    }
}
impl TryGoToU16 for f64 {
    fn try_goto_u16(&self) -> Result<u16, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u16::MAX as f64 {Ok(*self as u16)}
        else {Err(ConversionError::error("better-conversions::TryGoToU16::try_goto_u16", format!("{} out of u16 range", self)))}
    }
}
impl TryGoToU32 for f64 {
    fn try_goto_u32(&self) -> Result<u32, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u32::MAX as f64 {Ok(*self as u32)}
        else {Err(ConversionError::error("better-conversions::TryGoToU32::try_goto_u32", format!("{} out of u32 range", self)))}
    }
}
impl TryGoToU64 for f64 {
    fn try_goto_u64(&self) -> Result<u64, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u64::MAX as f64 {Ok(*self as u64)}
        else {Err(ConversionError::error("better-conversions::TryGoToU64::try_goto_u64", format!("{} out of u64 range", self)))}
    }
}
impl TryGoToU128 for f64 {
    fn try_goto_u128(&self) -> Result<u128, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= u128::MAX as f64 {Ok(*self as u128)}
        else {Err(ConversionError::error("better-conversions::TryGoToU128::try_goto_u128", format!("{} out of u128 range", self)))}
    }
}
impl TryGoToUsize for f64 {
    fn try_goto_usize(&self) -> Result<usize, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} is not finite", self)));}
        if self.fract() != 0.0 {return Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} has a fractional", self)));}
        if *self >= 0.0 && *self <= usize::MAX as f64 {Ok(*self as usize)}
        else {Err(ConversionError::error("better-conversions::TryGoToUsize::try_goto_usize", format!("{} out of usize range", self)))}
    }
}
impl TryGoToF32 for f64 {
    fn try_goto_f32(&self) -> Result<f32, ConversionError> {
        if !self.is_finite() {return Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} is not finite", self)));}
        if *self >= f32::MIN as f64 && *self <= f32::MAX as f64 {Ok(*self as f32)}
        else {Err(ConversionError::error("better-conversions::TryGoToF32::try_goto_f32", format!("{} out of f32 range", self)))}
    }
}
impl GoToF64 for f64 {fn goto_f64(&self) -> f64 {*self as f64}}
// f64 //

////////// END OF FILE //////////