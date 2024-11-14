#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    caldco_16mhz: CALDCO_16MHZ,
    calbc1_16mhz: CALBC1_16MHZ,
    caldco_12mhz: CALDCO_12MHZ,
    calbc1_12mhz: CALBC1_12MHZ,
    caldco_8mhz: CALDCO_8MHZ,
    calbc1_8mhz: CALBC1_8MHZ,
    caldco_1mhz: CALDCO_1MHZ,
    calbc1_1mhz: CALBC1_1MHZ,
}
impl RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 16MHz"]
    #[inline(always)]
    pub const fn caldco_16mhz(&self) -> &CALDCO_16MHZ {
        &self.caldco_16mhz
    }
    #[doc = "0x01 - BCSCTL1 Calibration Data for 16MHz"]
    #[inline(always)]
    pub const fn calbc1_16mhz(&self) -> &CALBC1_16MHZ {
        &self.calbc1_16mhz
    }
    #[doc = "0x02 - DCOCTL Calibration Data for 12MHz"]
    #[inline(always)]
    pub const fn caldco_12mhz(&self) -> &CALDCO_12MHZ {
        &self.caldco_12mhz
    }
    #[doc = "0x03 - BCSCTL1 Calibration Data for 12MHz"]
    #[inline(always)]
    pub const fn calbc1_12mhz(&self) -> &CALBC1_12MHZ {
        &self.calbc1_12mhz
    }
    #[doc = "0x04 - DCOCTL Calibration Data for 8MHz"]
    #[inline(always)]
    pub const fn caldco_8mhz(&self) -> &CALDCO_8MHZ {
        &self.caldco_8mhz
    }
    #[doc = "0x05 - BCSCTL1 Calibration Data for 8MHz"]
    #[inline(always)]
    pub const fn calbc1_8mhz(&self) -> &CALBC1_8MHZ {
        &self.calbc1_8mhz
    }
    #[doc = "0x06 - DCOCTL Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn caldco_1mhz(&self) -> &CALDCO_1MHZ {
        &self.caldco_1mhz
    }
    #[doc = "0x07 - BCSCTL1 Calibration Data for 1MHz"]
    #[inline(always)]
    pub const fn calbc1_1mhz(&self) -> &CALBC1_1MHZ {
        &self.calbc1_1mhz
    }
}
#[doc = "CALDCO_16MHZ (rw) register accessor: DCOCTL Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_16mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_16mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_16mhz`]
module"]
pub type CALDCO_16MHZ = crate::Reg<caldco_16mhz::CALDCO_16MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 16MHz"]
pub mod caldco_16mhz;
#[doc = "CALBC1_16MHZ (rw) register accessor: BCSCTL1 Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_16mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_16mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_16mhz`]
module"]
pub type CALBC1_16MHZ = crate::Reg<calbc1_16mhz::CALBC1_16MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 16MHz"]
pub mod calbc1_16mhz;
#[doc = "CALDCO_12MHZ (rw) register accessor: DCOCTL Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_12mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_12mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_12mhz`]
module"]
pub type CALDCO_12MHZ = crate::Reg<caldco_12mhz::CALDCO_12MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 12MHz"]
pub mod caldco_12mhz;
#[doc = "CALBC1_12MHZ (rw) register accessor: BCSCTL1 Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_12mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_12mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_12mhz`]
module"]
pub type CALBC1_12MHZ = crate::Reg<calbc1_12mhz::CALBC1_12MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 12MHz"]
pub mod calbc1_12mhz;
#[doc = "CALDCO_8MHZ (rw) register accessor: DCOCTL Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_8mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_8mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_8mhz`]
module"]
pub type CALDCO_8MHZ = crate::Reg<caldco_8mhz::CALDCO_8MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 8MHz"]
pub mod caldco_8mhz;
#[doc = "CALBC1_8MHZ (rw) register accessor: BCSCTL1 Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_8mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_8mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_8mhz`]
module"]
pub type CALBC1_8MHZ = crate::Reg<calbc1_8mhz::CALBC1_8MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 8MHz"]
pub mod calbc1_8mhz;
#[doc = "CALDCO_1MHZ (rw) register accessor: DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caldco_1mhz`]
module"]
pub type CALDCO_1MHZ = crate::Reg<caldco_1mhz::CALDCO_1MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calbc1_1mhz`]
module"]
pub type CALBC1_1MHZ = crate::Reg<calbc1_1mhz::CALBC1_1MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
