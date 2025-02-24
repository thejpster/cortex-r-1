//! Code for managing PMCNTENCLR (*Performance Monitors Count Enable Clear Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PMCNTENCLR (*Performance Monitors Count Enable Clear Register*)
pub struct Pmcntenclr(pub u32);
impl SysReg for Pmcntenclr {
    const CP: u32 = 15;
    const CRN: u32 = 9;
    const OP1: u32 = 0;
    const CRM: u32 = 12;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Pmcntenclr {}
impl Pmcntenclr {
    #[inline]
    /// Reads PMCNTENCLR (*Performance Monitors Count Enable Clear Register*)
    pub fn read() -> Pmcntenclr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Pmcntenclr {}
impl Pmcntenclr {
    #[inline]
    /// Writes PMCNTENCLR (*Performance Monitors Count Enable Clear Register*)
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
