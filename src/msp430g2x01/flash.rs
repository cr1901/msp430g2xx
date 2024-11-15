#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fctl1: FCTL1,
    fctl2: FCTL2,
    fctl3: FCTL3,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH Control 1"]
    #[inline(always)]
    pub const fn fctl1(&self) -> &FCTL1 {
        &self.fctl1
    }
    #[doc = "0x02 - FLASH Control 2"]
    #[inline(always)]
    pub const fn fctl2(&self) -> &FCTL2 {
        &self.fctl2
    }
    #[doc = "0x04 - FLASH Control 3"]
    #[inline(always)]
    pub const fn fctl3(&self) -> &FCTL3 {
        &self.fctl3
    }
}
#[doc = "FCTL1 (rw) register accessor: FLASH Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl1`]
module"]
pub type FCTL1 = crate::Reg<fctl1::FCTL1_SPEC>;
#[doc = "FLASH Control 1"]
pub mod fctl1;
#[doc = "FCTL2 (rw) register accessor: FLASH Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl2`]
module"]
pub type FCTL2 = crate::Reg<fctl2::FCTL2_SPEC>;
#[doc = "FLASH Control 2"]
pub mod fctl2;
#[doc = "FCTL3 (rw) register accessor: FLASH Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl3`]
module"]
pub type FCTL3 = crate::Reg<fctl3::FCTL3_SPEC>;
#[doc = "FLASH Control 3"]
pub mod fctl3;
