#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    caldco_1mhz: Caldco1mhz,
    calbc1_1mhz: Calbc1_1mhz,
}
impl RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn caldco_1mhz(&self) -> &Caldco1mhz {
        &self.caldco_1mhz
    }
    #[doc = "0x01 - BCSCTL1 Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn calbc1_1mhz(&self) -> &Calbc1_1mhz {
        &self.calbc1_1mhz
    }
}
#[doc = "CALDCO_1MHZ (rw) register accessor: DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_1mhz`]
module"]
#[doc(alias = "CALDCO_1MHZ")]
pub type Caldco1mhz = crate::Reg<caldco_1mhz::Caldco1mhzSpec>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_1mhz`]
module"]
#[doc(alias = "CALBC1_1MHZ")]
pub type Calbc1_1mhz = crate::Reg<calbc1_1mhz::Calbc1_1mhzSpec>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
