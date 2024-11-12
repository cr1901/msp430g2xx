#[doc = "Register `TLV_ADC10_1_LEN` reader"]
pub type R = crate::R<TlvAdc10_1LenSpec>;
#[doc = "Register `TLV_ADC10_1_LEN` writer"]
pub type W = crate::W<TlvAdc10_1LenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TLV ADC10_1 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvAdc10_1LenSpec;
impl crate::RegisterSpec for TlvAdc10_1LenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_adc10_1_len::R`](R) reader structure"]
impl crate::Readable for TlvAdc10_1LenSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_adc10_1_len::W`](W) writer structure"]
impl crate::Writable for TlvAdc10_1LenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_ADC10_1_LEN to value 0"]
impl crate::Resettable for TlvAdc10_1LenSpec {
    const RESET_VALUE: u8 = 0;
}
