#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc10dtc0: Adc10dtc0,
    adc10dtc1: Adc10dtc1,
    adc10ae0: Adc10ae0,
    _reserved3: [u8; 0x0165],
    adc10ctl0: Adc10ctl0,
    adc10ctl1: Adc10ctl1,
    adc10mem: Adc10mem,
    _reserved6: [u8; 0x06],
    adc10sa: Adc10sa,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC10 Data Transfer Control 0"]
    #[inline(always)]
    pub const fn adc10dtc0(&self) -> &Adc10dtc0 {
        &self.adc10dtc0
    }
    #[doc = "0x01 - ADC10 Data Transfer Control 1"]
    #[inline(always)]
    pub const fn adc10dtc1(&self) -> &Adc10dtc1 {
        &self.adc10dtc1
    }
    #[doc = "0x02 - ADC10 Analog Enable 0"]
    #[inline(always)]
    pub const fn adc10ae0(&self) -> &Adc10ae0 {
        &self.adc10ae0
    }
    #[doc = "0x168 - ADC10 Control 0"]
    #[inline(always)]
    pub const fn adc10ctl0(&self) -> &Adc10ctl0 {
        &self.adc10ctl0
    }
    #[doc = "0x16a - ADC10 Control 1"]
    #[inline(always)]
    pub const fn adc10ctl1(&self) -> &Adc10ctl1 {
        &self.adc10ctl1
    }
    #[doc = "0x16c - ADC10 Memory"]
    #[inline(always)]
    pub const fn adc10mem(&self) -> &Adc10mem {
        &self.adc10mem
    }
    #[doc = "0x174 - ADC10 Data Transfer Start Address"]
    #[inline(always)]
    pub const fn adc10sa(&self) -> &Adc10sa {
        &self.adc10sa
    }
}
#[doc = "ADC10DTC0 (rw) register accessor: ADC10 Data Transfer Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc0`]
module"]
#[doc(alias = "ADC10DTC0")]
pub type Adc10dtc0 = crate::Reg<adc10dtc0::Adc10dtc0Spec>;
#[doc = "ADC10 Data Transfer Control 0"]
pub mod adc10dtc0;
#[doc = "ADC10DTC1 (rw) register accessor: ADC10 Data Transfer Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10dtc1`]
module"]
#[doc(alias = "ADC10DTC1")]
pub type Adc10dtc1 = crate::Reg<adc10dtc1::Adc10dtc1Spec>;
#[doc = "ADC10 Data Transfer Control 1"]
pub mod adc10dtc1;
#[doc = "ADC10AE0 (rw) register accessor: ADC10 Analog Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ae0`]
module"]
#[doc(alias = "ADC10AE0")]
pub type Adc10ae0 = crate::Reg<adc10ae0::Adc10ae0Spec>;
#[doc = "ADC10 Analog Enable 0"]
pub mod adc10ae0;
#[doc = "ADC10CTL0 (rw) register accessor: ADC10 Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl0`]
module"]
#[doc(alias = "ADC10CTL0")]
pub type Adc10ctl0 = crate::Reg<adc10ctl0::Adc10ctl0Spec>;
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10CTL1 (rw) register accessor: ADC10 Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10ctl1`]
module"]
#[doc(alias = "ADC10CTL1")]
pub type Adc10ctl1 = crate::Reg<adc10ctl1::Adc10ctl1Spec>;
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10MEM (rw) register accessor: ADC10 Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10mem`]
module"]
#[doc(alias = "ADC10MEM")]
pub type Adc10mem = crate::Reg<adc10mem::Adc10memSpec>;
#[doc = "ADC10 Memory"]
pub mod adc10mem;
#[doc = "ADC10SA (rw) register accessor: ADC10 Data Transfer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10sa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10sa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10sa`]
module"]
#[doc(alias = "ADC10SA")]
pub type Adc10sa = crate::Reg<adc10sa::Adc10saSpec>;
#[doc = "ADC10 Data Transfer Start Address"]
pub mod adc10sa;
