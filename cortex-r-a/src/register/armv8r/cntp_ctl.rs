//! Code for managing CNTP_CTL (*Physical Counter-timer Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTP_CTL (*Physical Counter-timer Control Register*)
#[bitbybit::bitfield(u32)]
pub struct CntpCtl {
    /// The status of the timer interrupt.
    #[bits(2..=2, r)]
    istatus: bool,
    /// Timer interrupt mask bit.
    ///
    /// * true: masked
    /// * false: not masked
    #[bits(1..=1, rw)]
    imask: bool,
    /// Enables the timer.
    #[bits(0..=0, rw)]
    enable: bool,
}

impl core::fmt::Debug for CntpCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CntpCtl")
            .field("istatus", &self.istatus())
            .field("imask", &self.imask())
            .field("enable", &self.enable())
            .finish()
    }
}

impl SysReg for CntpCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl SysRegRead for CntpCtl {}

impl CntpCtl {
    #[inline]
    /// Reads CNTP_CTL (*Physical Counter-timer Control Register*)
    pub fn read() -> CntpCtl {
        unsafe { Self::new_with_raw_value(<Self as SysRegRead>::read_raw()) }
    }
}

impl SysRegWrite for CntpCtl {}

impl CntpCtl {
    #[inline]
    /// Writes CNTP_CTL (*Physical Counter-timer Control Register*)
    pub fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.raw_value());
        }
    }

    #[inline]
    /// Modifies CNTP_CTL (*Physical Counter-timer Control Register*)
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}
