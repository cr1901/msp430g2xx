#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ie1: IE1,
    ie2: IE2,
    ifg1: IFG1,
    ifg2: IFG2,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    #[inline(always)]
    pub const fn ie1(&self) -> &IE1 {
        &self.ie1
    }
    #[doc = "0x01 - Interrupt Enable 2"]
    #[inline(always)]
    pub const fn ie2(&self) -> &IE2 {
        &self.ie2
    }
    #[doc = "0x02 - Interrupt Flag 1"]
    #[inline(always)]
    pub const fn ifg1(&self) -> &IFG1 {
        &self.ifg1
    }
    #[doc = "0x03 - Interrupt Flag 2"]
    #[inline(always)]
    pub const fn ifg2(&self) -> &IFG2 {
        &self.ifg2
    }
}
#[doc = "IE1 (rw) register accessor: Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie1`]
module"]
pub type IE1 = crate::Reg<ie1::IE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IE2 (rw) register accessor: Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ie2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie2`]
module"]
pub type IE2 = crate::Reg<ie2::IE2_SPEC>;
#[doc = "Interrupt Enable 2"]
pub mod ie2;
#[doc = "IFG1 (rw) register accessor: Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg1`]
module"]
pub type IFG1 = crate::Reg<ifg1::IFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
#[doc = "IFG2 (rw) register accessor: Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifg2`]
module"]
pub type IFG2 = crate::Reg<ifg2::IFG2_SPEC>;
#[doc = "Interrupt Flag 2"]
pub mod ifg2;
