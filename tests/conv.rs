//!/////// START OF FILE //////////
//! better-conversions ///

use better_conversions::prelude::*;

#[test]
fn main() {
    println!("Test Start"); 
    
    let val_u8: u8 = 123;
    let max_u8: u8 = u8::MAX;
    assert_eq!(val_u8.goto_u16(), 123u16);
    assert_eq!(max_u8.goto_u16(), 255u16);
    assert_eq!(val_u8.goto_u32(), 123u32);
    assert_eq!(max_u8.goto_u32(), 255u32);
    assert_eq!(val_u8.goto_u64(), 123u64);
    assert_eq!(max_u8.goto_u64(), 255u64);
    assert_eq!(val_u8.goto_u128(), 123u128);
    assert_eq!(max_u8.goto_u128(), 255u128);
    assert_eq!(val_u8.goto_usize(), 123usize);
    assert_eq!(max_u8.goto_usize(), 255usize);
    assert_eq!(val_u8.goto_u8(), 123u8);
    assert_eq!(max_u8.goto_u8(), 255u8);
    assert_eq!(val_u8.goto_f32(), 123.0f32);
    assert_eq!(max_u8.goto_f32(), 255.0f32);
    assert_eq!(val_u8.goto_f64(), 123.0f64);
    assert_eq!(max_u8.goto_f64(), 255.0f64);
    
    let val_u16: u16 = 1000;
    let max_u16: u16 = u16::MAX;
    let too_large_for_u8: u16 = 257;
    assert_eq!(val_u16.goto_u32(), 1000u32);
    assert_eq!(max_u16.goto_u32(), 65535u32);
    assert_eq!(val_u16.goto_u64(), 1000u64);
    assert_eq!(max_u16.goto_u64(), 65535u64);
    assert_eq!(val_u16.goto_u128(), 1000u128);
    assert_eq!(max_u16.goto_u128(), 65535u128);
    assert_eq!(val_u16.goto_usize(), 1000usize);
    assert_eq!(max_u16.goto_usize(), 65535usize);
    assert_eq!(val_u16.goto_u16(), 1000u16);
    assert_eq!(max_u16.goto_u16(), 65535u16);
    assert_eq!(val_u16.goto_f32(), 1000.0f32);
    assert_eq!(max_u16.goto_f32(), 65535.0f32);
    assert_eq!(val_u16.goto_f64(), 1000.0f64);
    assert_eq!(max_u16.goto_f64(), 65535.0f64);
    let val_u16_small: u16 = 150;
    assert_eq!(val_u16_small.try_goto_u8().unwrap(), 150u8);
    assert_eq!(u16::from(u8::MAX).try_goto_u8().unwrap(), 255u8);
    assert!(too_large_for_u8.try_goto_u8().is_err());
    
    let val_u32: u32 = 70_000;
    let max_u32: u32 = u32::MAX;
    let too_large_for_u8_u32: u32 = u8::MAX as u32 + 1;
    let too_large_for_u16_u32: u32 = u16::MAX as u32 + 1;
    let too_large_for_f32_u32: u32 = 16_777_217;
    assert_eq!(val_u32.goto_u64(), 70_000u64);
    assert_eq!(max_u32.goto_u64(), 4294967295u64);
    assert_eq!(val_u32.goto_u128(), 70_000u128);
    assert_eq!(max_u32.goto_u128(), 4294967295u128);
    assert_eq!(val_u32.goto_usize(), 70_000usize);
    assert_eq!(max_u32.goto_usize(), 4294967295usize); 
    assert_eq!(val_u32.try_goto_f32().unwrap(), 70_000.0f32);
    assert!(too_large_for_f32_u32.try_goto_f32().is_err());
    assert_eq!(val_u32.goto_f64(), 70_000.0f64);
    assert_eq!(max_u32.goto_f64(), 4294967295.0f64);
    assert_eq!(val_u32.goto_u32(), 70_000u32);
    assert_eq!(max_u32.goto_u32(), 4294967295u32);
    let val_u32_small: u32 = 200;
    assert_eq!(val_u32_small.try_goto_u8().unwrap(), 200u8);
    assert_eq!((u8::MAX as u32).try_goto_u8().unwrap(), 255u8);
    assert!(too_large_for_u8_u32.try_goto_u8().is_err());
    let val_u32_medium: u32 = 30_000;
    assert_eq!(val_u32_medium.try_goto_u16().unwrap(), 30_000u16);
    assert_eq!((u16::MAX as u32).try_goto_u16().unwrap(), 65535u16);
    assert!(too_large_for_u16_u32.try_goto_u16().is_err());

    let val_u64: u64 = 1_000_000_000_000;
    let max_u64: u64 = u64::MAX;
    let too_large_for_u8_u64: u64 = u8::MAX as u64 + 1;
    let too_large_for_u16_u64: u64 = u16::MAX as u64 + 1;
    let too_large_for_u32_u64: u64 = u32::MAX as u64 + 1;
    let too_large_for_f32_u64: u64 = 16_777_217;
    let too_large_for_f64_u64: u64 = 9_007_199_254_740_993;
    assert_eq!(val_u64.goto_u128(), 1_000_000_000_000u128);
    assert_eq!(max_u64.goto_u128(), 18446744073709551615u128);
    assert_eq!(val_u64.goto_u64(), 1_000_000_000_000u64);
    let val_u64_small: u64 = 250;
    assert_eq!(val_u64_small.try_goto_u8().unwrap(), 250u8);
    assert_eq!((u8::MAX as u64).try_goto_u8().unwrap(), 255u8);
    assert!(too_large_for_u8_u64.try_goto_u8().is_err());
    let val_u64_medium: u64 = 60_000;
    assert_eq!(val_u64_medium.try_goto_u16().unwrap(), 60_000u16);
    assert_eq!((u16::MAX as u64).try_goto_u16().unwrap(), 65535u16);
    assert!(too_large_for_u16_u64.try_goto_u16().is_err());
    let val_u64_large: u64 = 4_000_000_000;
    assert_eq!(val_u64_large.try_goto_u32().unwrap(), 4_000_000_000u32);
    assert_eq!((u32::MAX as u64).try_goto_u32().unwrap(), 4294967295u32);
    assert!(too_large_for_u32_u64.try_goto_u32().is_err());
    if cfg!(target_pointer_width = "64") {
        assert_eq!(val_u64.try_goto_usize().unwrap(), 1_000_000_000_000usize);
        assert_eq!(max_u64.try_goto_usize().unwrap(), usize::MAX);
    } else { 
        let too_large_for_usize_u64: u64 = usize::MAX as u64 + 1;
        assert_eq!(val_u64_large.try_goto_usize().unwrap(), 4_000_000_000usize);
        assert!(val_u64.try_goto_usize().is_err());
        assert!(too_large_for_usize_u64.try_goto_usize().is_err());
    }
    let val_u64_exact_f32: u64 = 16_777_216;
    assert_eq!(val_u64_exact_f32.try_goto_f32().unwrap(), 16_777_216.0f32);
    assert!(too_large_for_f32_u64.try_goto_f32().is_err());
    let val_u64_exact_f64: u64 = 9_007_199_254_740_992;
    assert_eq!(val_u64_exact_f64.try_goto_f64().unwrap(), 9_007_199_254_740_992.0f64);
    assert!(too_large_for_f64_u64.try_goto_f64().is_err());

    let val_u128: u128 = u64::MAX as u128 + 100;
    let max_u128: u128 = u128::MAX;
    let too_large_for_u8_u128: u128 = u8::MAX as u128 + 1;
    let too_large_for_u16_u128: u128 = u16::MAX as u128 + 1;
    let too_large_for_u32_u128: u128 = u32::MAX as u128 + 1;
    let too_large_for_u64_u128: u128 = u64::MAX as u128 + 1;
    let too_large_for_f32_u128: u128 = 16_777_217;
    let too_large_for_f64_u128: u128 = 9_007_199_254_740_993;
    assert_eq!(val_u128.goto_u128(), val_u128);
    assert_eq!(max_u128.goto_u128(), u128::MAX);
    let val_u128_small: u128 = 100;
    assert_eq!(val_u128_small.try_goto_u8().unwrap(), 100u8);
    assert!(too_large_for_u8_u128.try_goto_u8().is_err());
    let val_u128_medium: u128 = 60_000;
    assert_eq!(val_u128_medium.try_goto_u16().unwrap(), 60_000u16);
    assert!(too_large_for_u16_u128.try_goto_u16().is_err());
    let val_u128_large: u128 = 4_000_000_000;
    assert_eq!(val_u128_large.try_goto_u32().unwrap(), 4_000_000_000u32);
    assert!(too_large_for_u32_u128.try_goto_u32().is_err());
    let val_u128_huge: u128 = 1_000_000_000_000_000_000;
    assert_eq!(val_u128_huge.try_goto_u64().unwrap(), 1_000_000_000_000_000_000u64);
    assert!(too_large_for_u64_u128.try_goto_u64().is_err());
    if cfg!(target_pointer_width = "64") {
        let too_large_for_usize_u128: u128 = u64::MAX as u128 + 1;
        assert_eq!(val_u128_huge.try_goto_usize().unwrap(), val_u128_huge as usize);
        assert!(too_large_for_usize_u128.try_goto_usize().is_err());
    } else { // 32-bit system
        let too_large_for_usize_u128: u128 = u32::MAX as u128 + 1;
        assert_eq!(val_u128_large.try_goto_usize().unwrap(), val_u128_large as usize);
        assert!(too_large_for_usize_u128.try_goto_usize().is_err());
    }
    let val_u128_exact_f32: u128 = 16_777_216;
    assert_eq!(val_u128_exact_f32.try_goto_f32().unwrap(), 16_777_216.0f32);
    assert!(too_large_for_f32_u128.try_goto_f32().is_err());
    let val_u128_exact_f64: u128 = 9_007_199_254_740_992;
    assert_eq!(val_u128_exact_f64.try_goto_f64().unwrap(), 9_007_199_254_740_992.0f64);
    assert!(too_large_for_f64_u128.try_goto_f64().is_err());

    println!("Test End"); 
}

////////// END OF FILE //////////