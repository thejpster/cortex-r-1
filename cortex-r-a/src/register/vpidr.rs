//! Code for managing VPIDR (*Virtualization Processor ID Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// VPIDR (*Virtualization Processor ID Register*)
pub struct Vpidr(pub u32);
impl SysReg for Vpidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Vpidr {}
impl Vpidr {
    #[inline]
    /// Reads VPIDR (*Virtualization Processor ID Register*)
    pub fn read() -> Vpidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Vpidr {}
impl Vpidr {
    #[inline]
    /// Writes VPIDR (*Virtualization Processor ID Register*)
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
