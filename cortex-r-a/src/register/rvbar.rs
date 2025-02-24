//! Code for managing RVBAR (*Reset Vector Base Address Register*)

use crate::register::{SysReg, SysRegRead};

/// RVBAR (*Reset Vector Base Address Register*)
pub struct Rvbar(pub u32);
impl SysReg for Rvbar {
    const CP: u32 = 15;
    const CRN: u32 = 12;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Rvbar {}
impl Rvbar {
    #[inline]
    /// Reads RVBAR (*Reset Vector Base Address Register*)
    pub fn read() -> Rvbar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
