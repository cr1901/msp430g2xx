#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ie1: IE1,
    _reserved1: [u8; 0x01],
    ifg1: IFG1,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn ie1(&self) -> &IE1 {
        &self.ie1
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn ifg1(&self) -> &IFG1 {
        &self.ifg1
    }
}
#[doc = "IE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie1`]
module"]
pub type IE1 = crate::Reg<ie1::IE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg1`]
module"]
pub type IFG1 = crate::Reg<ifg1::IFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
