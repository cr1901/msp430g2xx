#[doc = "Register `TLV_ADC10_1_TAG` reader"]
pub type R = crate::R<TLV_ADC10_1_TAG_SPEC>;
#[doc = "Register `TLV_ADC10_1_TAG` writer"]
pub type W = crate::W<TLV_ADC10_1_TAG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TLV ADC10_1 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLV_ADC10_1_TAG_SPEC;
impl crate::RegisterSpec for TLV_ADC10_1_TAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_adc10_1_tag::R`](R) reader structure"]
impl crate::Readable for TLV_ADC10_1_TAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tlv_adc10_1_tag::W`](W) writer structure"]
impl crate::Writable for TLV_ADC10_1_TAG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_ADC10_1_TAG to value 0"]
impl crate::Resettable for TLV_ADC10_1_TAG_SPEC {
    const RESET_VALUE: u8 = 0;
}
