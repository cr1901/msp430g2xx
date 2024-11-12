#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usictl0: Usictl0,
    usictl1: Usictl1,
    usickctl: Usickctl,
    usicnt: Usicnt,
    usisrl: Usisrl,
    usisrh: Usisrh,
}
impl RegisterBlock {
    #[doc = "0x00 - USI Control Register 0"]
    #[inline(always)]
    pub const fn usictl0(&self) -> &Usictl0 {
        &self.usictl0
    }
    #[doc = "0x01 - USI Control Register 1"]
    #[inline(always)]
    pub const fn usictl1(&self) -> &Usictl1 {
        &self.usictl1
    }
    #[doc = "0x02 - USI Clock Control Register"]
    #[inline(always)]
    pub const fn usickctl(&self) -> &Usickctl {
        &self.usickctl
    }
    #[doc = "0x03 - USI Bit Counter Register"]
    #[inline(always)]
    pub const fn usicnt(&self) -> &Usicnt {
        &self.usicnt
    }
    #[doc = "0x04 - USI Low Byte Shift Register"]
    #[inline(always)]
    pub const fn usisrl(&self) -> &Usisrl {
        &self.usisrl
    }
    #[doc = "0x05 - USI High Byte Shift Register"]
    #[inline(always)]
    pub const fn usisrh(&self) -> &Usisrh {
        &self.usisrh
    }
}
#[doc = "USICTL0 (rw) register accessor: USI Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usictl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usictl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usictl0`]
module"]
#[doc(alias = "USICTL0")]
pub type Usictl0 = crate::Reg<usictl0::Usictl0Spec>;
#[doc = "USI Control Register 0"]
pub mod usictl0;
#[doc = "USICTL1 (rw) register accessor: USI Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`usictl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usictl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usictl1`]
module"]
#[doc(alias = "USICTL1")]
pub type Usictl1 = crate::Reg<usictl1::Usictl1Spec>;
#[doc = "USI Control Register 1"]
pub mod usictl1;
#[doc = "USICKCTL (rw) register accessor: USI Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usickctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usickctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usickctl`]
module"]
#[doc(alias = "USICKCTL")]
pub type Usickctl = crate::Reg<usickctl::UsickctlSpec>;
#[doc = "USI Clock Control Register"]
pub mod usickctl;
#[doc = "USICNT (rw) register accessor: USI Bit Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usicnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usicnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usicnt`]
module"]
#[doc(alias = "USICNT")]
pub type Usicnt = crate::Reg<usicnt::UsicntSpec>;
#[doc = "USI Bit Counter Register"]
pub mod usicnt;
#[doc = "USISRL (rw) register accessor: USI Low Byte Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usisrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usisrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usisrl`]
module"]
#[doc(alias = "USISRL")]
pub type Usisrl = crate::Reg<usisrl::UsisrlSpec>;
#[doc = "USI Low Byte Shift Register"]
pub mod usisrl;
#[doc = "USISRH (rw) register accessor: USI High Byte Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usisrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usisrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usisrh`]
module"]
#[doc(alias = "USISRH")]
pub type Usisrh = crate::Reg<usisrh::UsisrhSpec>;
#[doc = "USI High Byte Shift Register"]
pub mod usisrh;
