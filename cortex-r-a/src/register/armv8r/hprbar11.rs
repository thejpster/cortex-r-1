//! Code for managing HPRBAR11 (*Hyp Protection Region Base Address Register 11*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR11 (*Hyp Protection Region Base Address Register 11*)
pub struct Hprbar11(pub u32);
impl SysReg for Hprbar11 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 13;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Hprbar11 {}
impl Hprbar11 {
    #[inline]
    /// Reads HPRBAR11 (*Hyp Protection Region Base Address Register 11*)
    pub fn read() -> Hprbar11 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar11 {}
impl Hprbar11 {
    #[inline]
    /// Writes HPRBAR11 (*Hyp Protection Region Base Address Register 11*)
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
