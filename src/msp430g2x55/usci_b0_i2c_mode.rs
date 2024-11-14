#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucb0ctl0: UCB0CTL0,
    ucb0ctl1: UCB0CTL1,
    ucb0br0: UCB0BR0,
    ucb0br1: UCB0BR1,
    ucb0i2cie: UCB0I2CIE,
    ucb0stat: UCB0STAT,
    ucb0rxbuf: UCB0RXBUF,
    ucb0txbuf: UCB0TXBUF,
    _reserved8: [u8; 0xa8],
    ucb0i2coa: UCB0I2COA,
    ucb0i2csa: UCB0I2CSA,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0(&self) -> &UCB0CTL0 {
        &self.ucb0ctl0
    }
    #[doc = "0x01 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1(&self) -> &UCB0CTL1 {
        &self.ucb0ctl1
    }
    #[doc = "0x02 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0(&self) -> &UCB0BR0 {
        &self.ucb0br0
    }
    #[doc = "0x03 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1(&self) -> &UCB0BR1 {
        &self.ucb0br1
    }
    #[doc = "0x04 - USCI B0 I2C Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0i2cie(&self) -> &UCB0I2CIE {
        &self.ucb0i2cie
    }
    #[doc = "0x05 - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat(&self) -> &UCB0STAT {
        &self.ucb0stat
    }
    #[doc = "0x06 - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &UCB0RXBUF {
        &self.ucb0rxbuf
    }
    #[doc = "0x07 - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &UCB0TXBUF {
        &self.ucb0txbuf
    }
    #[doc = "0xb0 - USCI B0 I2C Own Address"]
    #[inline(always)]
    pub const fn ucb0i2coa(&self) -> &UCB0I2COA {
        &self.ucb0i2coa
    }
    #[doc = "0xb2 - USCI B0 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb0i2csa(&self) -> &UCB0I2CSA {
        &self.ucb0i2csa
    }
}
#[doc = "UCB0CTL0 (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0`]
module"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0CTL1 (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1`]
module"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0BR0 (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0`]
module"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1`]
module"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0I2CIE (rw) register accessor: USCI B0 I2C Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2cie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2cie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2cie`]
module"]
pub type UCB0I2CIE = crate::Reg<ucb0i2cie::UCB0I2CIE_SPEC>;
#[doc = "USCI B0 I2C Interrupt Enable Register"]
pub mod ucb0i2cie;
#[doc = "UCB0STAT (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat`]
module"]
pub type UCB0STAT = crate::Reg<ucb0stat::UCB0STAT_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`]
module"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`]
module"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0I2COA (rw) register accessor: USCI B0 I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa`]
module"]
pub type UCB0I2COA = crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>;
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "UCB0I2CSA (rw) register accessor: USCI B0 I2C Slave Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2csa`]
module"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
