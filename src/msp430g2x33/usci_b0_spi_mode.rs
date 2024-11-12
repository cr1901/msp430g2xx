#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb0ctl0: Ucb0ctl0,
    ucb0ctl1: Ucb0ctl1,
    ucb0br0: Ucb0br0,
    ucb0br1: Ucb0br1,
    _reserved4: [u8; 0x01],
    ucb0stat: Ucb0stat,
    ucb0rxbuf: Ucb0rxbuf,
    ucb0txbuf: Ucb0txbuf,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0(&self) -> &Ucb0ctl0 {
        &self.ucb0ctl0
    }
    #[doc = "0x01 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1(&self) -> &Ucb0ctl1 {
        &self.ucb0ctl1
    }
    #[doc = "0x02 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0(&self) -> &Ucb0br0 {
        &self.ucb0br0
    }
    #[doc = "0x03 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1(&self) -> &Ucb0br1 {
        &self.ucb0br1
    }
    #[doc = "0x05 - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat(&self) -> &Ucb0stat {
        &self.ucb0stat
    }
    #[doc = "0x06 - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &Ucb0rxbuf {
        &self.ucb0rxbuf
    }
    #[doc = "0x07 - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &Ucb0txbuf {
        &self.ucb0txbuf
    }
}
#[doc = "UCB0CTL0 (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0`]
module"]
#[doc(alias = "UCB0CTL0")]
pub type Ucb0ctl0 = crate::Reg<ucb0ctl0::Ucb0ctl0Spec>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0CTL1 (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1`]
module"]
#[doc(alias = "UCB0CTL1")]
pub type Ucb0ctl1 = crate::Reg<ucb0ctl1::Ucb0ctl1Spec>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0BR0 (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0`]
module"]
#[doc(alias = "UCB0BR0")]
pub type Ucb0br0 = crate::Reg<ucb0br0::Ucb0br0Spec>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1`]
module"]
#[doc(alias = "UCB0BR1")]
pub type Ucb0br1 = crate::Reg<ucb0br1::Ucb0br1Spec>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat`]
module"]
#[doc(alias = "UCB0STAT")]
pub type Ucb0stat = crate::Reg<ucb0stat::Ucb0statSpec>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`]
module"]
#[doc(alias = "UCB0RXBUF")]
pub type Ucb0rxbuf = crate::Reg<ucb0rxbuf::Ucb0rxbufSpec>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`]
module"]
#[doc(alias = "UCB0TXBUF")]
pub type Ucb0txbuf = crate::Reg<ucb0txbuf::Ucb0txbufSpec>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
