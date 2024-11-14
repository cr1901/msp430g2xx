#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tlv_checksum: TLV_CHECKSUM,
    _reserved1: [u8; 0x18],
    tlv_adc10_1_tag: TLV_ADC10_1_TAG,
    tlv_adc10_1_len: TLV_ADC10_1_LEN,
    _reserved3: [u8; 0x1a],
    tlv_dco_30_tag: TLV_DCO_30_TAG,
    tlv_dco_30_len: TLV_DCO_30_LEN,
}
impl RegisterBlock {
    #[doc = "0x00 - TLV CHECK SUM"]
    #[inline(always)]
    pub const fn tlv_checksum(&self) -> &TLV_CHECKSUM {
        &self.tlv_checksum
    }
    #[doc = "0x1a - TLV ADC10_1 TAG"]
    #[inline(always)]
    pub const fn tlv_adc10_1_tag(&self) -> &TLV_ADC10_1_TAG {
        &self.tlv_adc10_1_tag
    }
    #[doc = "0x1b - TLV ADC10_1 LEN"]
    #[inline(always)]
    pub const fn tlv_adc10_1_len(&self) -> &TLV_ADC10_1_LEN {
        &self.tlv_adc10_1_len
    }
    #[doc = "0x36 - TLV TAG_DCO30 TAG"]
    #[inline(always)]
    pub const fn tlv_dco_30_tag(&self) -> &TLV_DCO_30_TAG {
        &self.tlv_dco_30_tag
    }
    #[doc = "0x37 - TLV TAG_DCO30 LEN"]
    #[inline(always)]
    pub const fn tlv_dco_30_len(&self) -> &TLV_DCO_30_LEN {
        &self.tlv_dco_30_len
    }
}
#[doc = "TLV_ADC10_1_TAG (rw) register accessor: TLV ADC10_1 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_adc10_1_tag`]
module"]
pub type TLV_ADC10_1_TAG = crate::Reg<tlv_adc10_1_tag::TLV_ADC10_1_TAG_SPEC>;
#[doc = "TLV ADC10_1 TAG"]
pub mod tlv_adc10_1_tag;
#[doc = "TLV_ADC10_1_LEN (rw) register accessor: TLV ADC10_1 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_adc10_1_len`]
module"]
pub type TLV_ADC10_1_LEN = crate::Reg<tlv_adc10_1_len::TLV_ADC10_1_LEN_SPEC>;
#[doc = "TLV ADC10_1 LEN"]
pub mod tlv_adc10_1_len;
#[doc = "TLV_DCO_30_TAG (rw) register accessor: TLV TAG_DCO30 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_dco_30_tag`]
module"]
pub type TLV_DCO_30_TAG = crate::Reg<tlv_dco_30_tag::TLV_DCO_30_TAG_SPEC>;
#[doc = "TLV TAG_DCO30 TAG"]
pub mod tlv_dco_30_tag;
#[doc = "TLV_DCO_30_LEN (rw) register accessor: TLV TAG_DCO30 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_dco_30_len`]
module"]
pub type TLV_DCO_30_LEN = crate::Reg<tlv_dco_30_len::TLV_DCO_30_LEN_SPEC>;
#[doc = "TLV TAG_DCO30 LEN"]
pub mod tlv_dco_30_len;
#[doc = "TLV_CHECKSUM (rw) register accessor: TLV CHECK SUM\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_checksum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_checksum`]
module"]
pub type TLV_CHECKSUM = crate::Reg<tlv_checksum::TLV_CHECKSUM_SPEC>;
#[doc = "TLV CHECK SUM"]
pub mod tlv_checksum;
