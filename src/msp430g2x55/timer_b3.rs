#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tbiv: Tbiv,
    _reserved1: [u8; 0x60],
    tbctl: Tbctl,
    tbcctl0: Tbcctl0,
    tbcctl1: Tbcctl1,
    tbcctl2: Tbcctl2,
    _reserved5: [u8; 0x08],
    tbr: Tbr,
    tbccr0: Tbccr0,
    tbccr1: Tbccr1,
    tbccr2: Tbccr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer B Interrupt Vector Word"]
    #[inline(always)]
    pub const fn tbiv(&self) -> &Tbiv {
        &self.tbiv
    }
    #[doc = "0x62 - Timer B Control"]
    #[inline(always)]
    pub const fn tbctl(&self) -> &Tbctl {
        &self.tbctl
    }
    #[doc = "0x64 - Timer B Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tbcctl0(&self) -> &Tbcctl0 {
        &self.tbcctl0
    }
    #[doc = "0x66 - Timer B Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tbcctl1(&self) -> &Tbcctl1 {
        &self.tbcctl1
    }
    #[doc = "0x68 - Timer B Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tbcctl2(&self) -> &Tbcctl2 {
        &self.tbcctl2
    }
    #[doc = "0x72 - Timer B Counter Register"]
    #[inline(always)]
    pub const fn tbr(&self) -> &Tbr {
        &self.tbr
    }
    #[doc = "0x74 - Timer B Capture/Compare 0"]
    #[inline(always)]
    pub const fn tbccr0(&self) -> &Tbccr0 {
        &self.tbccr0
    }
    #[doc = "0x76 - Timer B Capture/Compare 1"]
    #[inline(always)]
    pub const fn tbccr1(&self) -> &Tbccr1 {
        &self.tbccr1
    }
    #[doc = "0x78 - Timer B Capture/Compare 2"]
    #[inline(always)]
    pub const fn tbccr2(&self) -> &Tbccr2 {
        &self.tbccr2
    }
}
#[doc = "TBIV (rw) register accessor: Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbiv`]
module"]
#[doc(alias = "TBIV")]
pub type Tbiv = crate::Reg<tbiv::TbivSpec>;
#[doc = "Timer B Interrupt Vector Word"]
pub mod tbiv;
#[doc = "TBCTL (rw) register accessor: Timer B Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl`]
module"]
#[doc(alias = "TBCTL")]
pub type Tbctl = crate::Reg<tbctl::TbctlSpec>;
#[doc = "Timer B Control"]
pub mod tbctl;
#[doc = "TBCCTL0 (rw) register accessor: Timer B Capture/Compare Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl0`]
module"]
#[doc(alias = "TBCCTL0")]
pub type Tbcctl0 = crate::Reg<tbcctl0::Tbcctl0Spec>;
#[doc = "Timer B Capture/Compare Control 0"]
pub mod tbcctl0;
#[doc = "TBCCTL1 (rw) register accessor: Timer B Capture/Compare Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl1`]
module"]
#[doc(alias = "TBCCTL1")]
pub type Tbcctl1 = crate::Reg<tbcctl1::Tbcctl1Spec>;
#[doc = "Timer B Capture/Compare Control 1"]
pub mod tbcctl1;
#[doc = "TBCCTL2 (rw) register accessor: Timer B Capture/Compare Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbcctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbcctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbcctl2`]
module"]
#[doc(alias = "TBCCTL2")]
pub type Tbcctl2 = crate::Reg<tbcctl2::Tbcctl2Spec>;
#[doc = "Timer B Capture/Compare Control 2"]
pub mod tbcctl2;
#[doc = "TBR (rw) register accessor: Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbr`]
module"]
#[doc(alias = "TBR")]
pub type Tbr = crate::Reg<tbr::TbrSpec>;
#[doc = "Timer B Counter Register"]
pub mod tbr;
#[doc = "TBCCR0 (rw) register accessor: Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr0`]
module"]
#[doc(alias = "TBCCR0")]
pub type Tbccr0 = crate::Reg<tbccr0::Tbccr0Spec>;
#[doc = "Timer B Capture/Compare 0"]
pub mod tbccr0;
#[doc = "TBCCR1 (rw) register accessor: Timer B Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr1`]
module"]
#[doc(alias = "TBCCR1")]
pub type Tbccr1 = crate::Reg<tbccr1::Tbccr1Spec>;
#[doc = "Timer B Capture/Compare 1"]
pub mod tbccr1;
#[doc = "TBCCR2 (rw) register accessor: Timer B Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbccr2`]
module"]
#[doc(alias = "TBCCR2")]
pub type Tbccr2 = crate::Reg<tbccr2::Tbccr2Spec>;
#[doc = "Timer B Capture/Compare 2"]
pub mod tbccr2;
