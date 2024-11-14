#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc10dtc0: ADC10DTC0,
    adc10dtc1: ADC10DTC1,
    adc10ae0: ADC10AE0,
    adc10ae1: ADC10AE1,
    _reserved4: [u8; 0x0164],
    adc10ctl0: ADC10CTL0,
    adc10ctl1: ADC10CTL1,
    adc10mem: ADC10MEM,
    _reserved7: [u8; 0x06],
    adc10sa: ADC10SA,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC10 Data Transfer Control 0"]
    #[inline(always)]
    pub const fn adc10dtc0(&self) -> &ADC10DTC0 {
        &self.adc10dtc0
    }
    #[doc = "0x01 - ADC10 Data Transfer Control 1"]
    #[inline(always)]
    pub const fn adc10dtc1(&self) -> &ADC10DTC1 {
        &self.adc10dtc1
    }
    #[doc = "0x02 - ADC10 Analog Enable 0"]
    #[inline(always)]
    pub const fn adc10ae0(&self) -> &ADC10AE0 {
        &self.adc10ae0
    }
    #[doc = "0x03 - ADC10 Analog Enable 1"]
    #[inline(always)]
    pub const fn adc10ae1(&self) -> &ADC10AE1 {
        &self.adc10ae1
    }
    #[doc = "0x168 - ADC10 Control 0"]
    #[inline(always)]
    pub const fn adc10ctl0(&self) -> &ADC10CTL0 {
        &self.adc10ctl0
    }
    #[doc = "0x16a - ADC10 Control 1"]
    #[inline(always)]
    pub const fn adc10ctl1(&self) -> &ADC10CTL1 {
        &self.adc10ctl1
    }
    #[doc = "0x16c - ADC10 Memory"]
    #[inline(always)]
    pub const fn adc10mem(&self) -> &ADC10MEM {
        &self.adc10mem
    }
    #[doc = "0x174 - ADC10 Data Transfer Start Address"]
    #[inline(always)]
    pub const fn adc10sa(&self) -> &ADC10SA {
        &self.adc10sa
    }
}
#[doc = "ADC10DTC0 (rw) register accessor: ADC10 Data Transfer Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc0`]
module"]
pub type ADC10DTC0 = crate::Reg<adc10dtc0::ADC10DTC0_SPEC>;
#[doc = "ADC10 Data Transfer Control 0"]
pub mod adc10dtc0;
#[doc = "ADC10DTC1 (rw) register accessor: ADC10 Data Transfer Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc1`]
module"]
pub type ADC10DTC1 = crate::Reg<adc10dtc1::ADC10DTC1_SPEC>;
#[doc = "ADC10 Data Transfer Control 1"]
pub mod adc10dtc1;
#[doc = "ADC10AE0 (rw) register accessor: ADC10 Analog Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ae0`]
module"]
pub type ADC10AE0 = crate::Reg<adc10ae0::ADC10AE0_SPEC>;
#[doc = "ADC10 Analog Enable 0"]
pub mod adc10ae0;
#[doc = "ADC10AE1 (rw) register accessor: ADC10 Analog Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ae1`]
module"]
pub type ADC10AE1 = crate::Reg<adc10ae1::ADC10AE1_SPEC>;
#[doc = "ADC10 Analog Enable 1"]
pub mod adc10ae1;
#[doc = "ADC10CTL0 (rw) register accessor: ADC10 Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl0`]
module"]
pub type ADC10CTL0 = crate::Reg<adc10ctl0::ADC10CTL0_SPEC>;
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10CTL1 (rw) register accessor: ADC10 Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl1`]
module"]
pub type ADC10CTL1 = crate::Reg<adc10ctl1::ADC10CTL1_SPEC>;
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10MEM (rw) register accessor: ADC10 Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10mem`]
module"]
pub type ADC10MEM = crate::Reg<adc10mem::ADC10MEM_SPEC>;
#[doc = "ADC10 Memory"]
pub mod adc10mem;
#[doc = "ADC10SA (rw) register accessor: ADC10 Data Transfer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10sa`]
module"]
pub type ADC10SA = crate::Reg<adc10sa::ADC10SA_SPEC>;
#[doc = "ADC10 Data Transfer Start Address"]
pub mod adc10sa;
