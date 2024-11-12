#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ie1: Ie1,
    _reserved1: [u8; 0x01],
    ifg1: Ifg1,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn ie1(&self) -> &Ie1 {
        &self.ie1
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn ifg1(&self) -> &Ifg1 {
        &self.ifg1
    }
}
#[doc = "IE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie1`]
module"]
#[doc(alias = "IE1")]
pub type Ie1 = crate::Reg<ie1::Ie1Spec>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg1`]
module"]
#[doc(alias = "IFG1")]
pub type Ifg1 = crate::Reg<ifg1::Ifg1Spec>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
