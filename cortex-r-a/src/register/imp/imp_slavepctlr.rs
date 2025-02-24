//! Code for managing IMP_SLAVEPCTLR (*Slave Port Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IMP_SLAVEPCTLR (*Slave Port Control Register*)
pub struct ImpSlavepctlr(pub u32);
impl SysReg for ImpSlavepctlr {
    const CP: u32 = 15;
    const CRN: u32 = 11;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for ImpSlavepctlr {}
impl ImpSlavepctlr {
    #[inline]
    /// Reads IMP_SLAVEPCTLR (*Slave Port Control Register*)
    pub fn read() -> ImpSlavepctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for ImpSlavepctlr {}
impl ImpSlavepctlr {
    #[inline]
    /// Writes IMP_SLAVEPCTLR (*Slave Port Control Register*)
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
