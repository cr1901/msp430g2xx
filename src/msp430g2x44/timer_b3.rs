#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tbiv: TBIV,
    _reserved1: [u8; 0x60],
    tbctl: TBCTL,
    tbcctl0: TBCCTL0,
    tbcctl1: TBCCTL1,
    tbcctl2: TBCCTL2,
    _reserved5: [u8; 0x08],
    tbr: TBR,
    tbccr0: TBCCR0,
    tbccr1: TBCCR1,
    tbccr2: TBCCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer B Interrupt Vector Word"]
    #[inline(always)]
    pub const fn tbiv(&self) -> &TBIV {
        &self.tbiv
    }
    #[doc = "0x62 - Timer B Control"]
    #[inline(always)]
    pub const fn tbctl(&self) -> &TBCTL {
        &self.tbctl
    }
    #[doc = "0x64 - Timer B Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tbcctl0(&self) -> &TBCCTL0 {
        &self.tbcctl0
    }
    #[doc = "0x66 - Timer B Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tbcctl1(&self) -> &TBCCTL1 {
        &self.tbcctl1
    }
    #[doc = "0x68 - Timer B Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tbcctl2(&self) -> &TBCCTL2 {
        &self.tbcctl2
    }
    #[doc = "0x72 - Timer B Counter Register"]
    #[inline(always)]
    pub const fn tbr(&self) -> &TBR {
        &self.tbr
    }
    #[doc = "0x74 - Timer B Capture/Compare 0"]
    #[inline(always)]
    pub const fn tbccr0(&self) -> &TBCCR0 {
        &self.tbccr0
    }
    #[doc = "0x76 - Timer B Capture/Compare 1"]
    #[inline(always)]
    pub const fn tbccr1(&self) -> &TBCCR1 {
        &self.tbccr1
    }
    #[doc = "0x78 - Timer B Capture/Compare 2"]
    #[inline(always)]
    pub const fn tbccr2(&self) -> &TBCCR2 {
        &self.tbccr2
    }
}
#[doc = "TBIV (rw) register accessor: Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbiv`]
module"]
pub type TBIV = crate::Reg<tbiv::TBIV_SPEC>;
#[doc = "Timer B Interrupt Vector Word"]
pub mod tbiv;
#[doc = "TBCTL (rw) register accessor: Timer B Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl`]
module"]
pub type TBCTL = crate::Reg<tbctl::TBCTL_SPEC>;
#[doc = "Timer B Control"]
pub mod tbctl;
#[doc = "TBCCTL0 (rw) register accessor: Timer B Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl0`]
module"]
pub type TBCCTL0 = crate::Reg<tbcctl0::TBCCTL0_SPEC>;
#[doc = "Timer B Capture/Compare Control 0"]
pub mod tbcctl0;
#[doc = "TBCCTL1 (rw) register accessor: Timer B Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl1`]
module"]
pub type TBCCTL1 = crate::Reg<tbcctl1::TBCCTL1_SPEC>;
#[doc = "Timer B Capture/Compare Control 1"]
pub mod tbcctl1;
#[doc = "TBCCTL2 (rw) register accessor: Timer B Capture/Compare Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl2`]
module"]
pub type TBCCTL2 = crate::Reg<tbcctl2::TBCCTL2_SPEC>;
#[doc = "Timer B Capture/Compare Control 2"]
pub mod tbcctl2;
#[doc = "TBR (rw) register accessor: Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`]
module"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "Timer B Counter Register"]
pub mod tbr;
#[doc = "TBCCR0 (rw) register accessor: Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr0`]
module"]
pub type TBCCR0 = crate::Reg<tbccr0::TBCCR0_SPEC>;
#[doc = "Timer B Capture/Compare 0"]
pub mod tbccr0;
#[doc = "TBCCR1 (rw) register accessor: Timer B Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr1`]
module"]
pub type TBCCR1 = crate::Reg<tbccr1::TBCCR1_SPEC>;
#[doc = "Timer B Capture/Compare 1"]
pub mod tbccr1;
#[doc = "TBCCR2 (rw) register accessor: Timer B Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr2`]
module"]
pub type TBCCR2 = crate::Reg<tbccr2::TBCCR2_SPEC>;
#[doc = "Timer B Capture/Compare 2"]
pub mod tbccr2;
