#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    cactl1: CACTL1,
    cactl2: CACTL2,
    capd: CAPD,
}
impl RegisterBlock {
    #[doc = "0x01 - Comparator A Control 1"]
    #[inline(always)]
    pub const fn cactl1(&self) -> &CACTL1 {
        &self.cactl1
    }
    #[doc = "0x02 - Comparator A Control 2"]
    #[inline(always)]
    pub const fn cactl2(&self) -> &CACTL2 {
        &self.cactl2
    }
    #[doc = "0x03 - Comparator A Port Disable"]
    #[inline(always)]
    pub const fn capd(&self) -> &CAPD {
        &self.capd
    }
}
#[doc = "CACTL1 (rw) register accessor: Comparator A Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cactl1`]
module"]
pub type CACTL1 = crate::Reg<cactl1::CACTL1_SPEC>;
#[doc = "Comparator A Control 1"]
pub mod cactl1;
#[doc = "CACTL2 (rw) register accessor: Comparator A Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cactl2`]
module"]
pub type CACTL2 = crate::Reg<cactl2::CACTL2_SPEC>;
#[doc = "Comparator A Control 2"]
pub mod cactl2;
#[doc = "CAPD (rw) register accessor: Comparator A Port Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`capd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capd`]
module"]
pub type CAPD = crate::Reg<capd::CAPD_SPEC>;
#[doc = "Comparator A Port Disable"]
pub mod capd;
