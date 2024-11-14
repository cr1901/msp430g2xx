#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uca0ctl0: UCA0CTL0,
    uca0ctl1: UCA0CTL1,
    uca0br0: UCA0BR0,
    uca0br1: UCA0BR1,
    _reserved4: [u8; 0x01],
    uca0stat: UCA0STAT,
    uca0rxbuf: UCA0RXBUF,
    uca0txbuf: UCA0TXBUF,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0(&self) -> &UCA0CTL0 {
        &self.uca0ctl0
    }
    #[doc = "0x01 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1(&self) -> &UCA0CTL1 {
        &self.uca0ctl1
    }
    #[doc = "0x02 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0(&self) -> &UCA0BR0 {
        &self.uca0br0
    }
    #[doc = "0x03 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1(&self) -> &UCA0BR1 {
        &self.uca0br1
    }
    #[doc = "0x05 - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0stat(&self) -> &UCA0STAT {
        &self.uca0stat
    }
    #[doc = "0x06 - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf(&self) -> &UCA0RXBUF {
        &self.uca0rxbuf
    }
    #[doc = "0x07 - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf(&self) -> &UCA0TXBUF {
        &self.uca0txbuf
    }
}
#[doc = "UCA0CTL0 (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0`]
module"]
pub type UCA0CTL0 = crate::Reg<uca0ctl0::UCA0CTL0_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0CTL1 (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1`]
module"]
pub type UCA0CTL1 = crate::Reg<uca0ctl1::UCA0CTL1_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0BR0 (rw) register accessor: USCI A0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br0`]
module"]
pub type UCA0BR0 = crate::Reg<uca0br0::UCA0BR0_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 (rw) register accessor: USCI A0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br1`]
module"]
pub type UCA0BR1 = crate::Reg<uca0br1::UCA0BR1_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0STAT (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0stat`]
module"]
pub type UCA0STAT = crate::Reg<uca0stat::UCA0STAT_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
#[doc = "UCA0RXBUF (rw) register accessor: USCI A0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf`]
module"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF (rw) register accessor: USCI A0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf`]
module"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
