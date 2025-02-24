//! Code for managing HPRSELR (*Hyp Protection Region Selection Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRSELR (*Hyp Protection Region Selection Register*)
pub struct Hprselr(pub u32);
impl SysReg for Hprselr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprselr {}
impl Hprselr {
    #[inline]
    /// Reads HPRSELR (*Hyp Protection Region Selection Register*)
    pub fn read() -> Hprselr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprselr {}
impl Hprselr {
    #[inline]
    /// Writes HPRSELR (*Hyp Protection Region Selection Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
