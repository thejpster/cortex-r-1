//! Code for managing PRBAR14 (*Protection Region Base Address Register 14*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR14 (*Protection Region Base Address Register 14*)
pub struct Prbar14(pub u32);
impl SysReg for Prbar14 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar14 {}
impl Prbar14 {
    #[inline]
    /// Reads PRBAR14 (*Protection Region Base Address Register 14*)
    pub fn read() -> Prbar14 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar14 {}
impl Prbar14 {
    #[inline]
    /// Writes PRBAR14 (*Protection Region Base Address Register 14*)
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
