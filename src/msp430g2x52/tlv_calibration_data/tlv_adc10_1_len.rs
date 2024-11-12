#[doc = "Register `TLV_ADC10_1_LEN` reader"]
pub type R = crate::R<TlvAdc10_1LenSpec>;
#[doc = "Register `TLV_ADC10_1_LEN` writer"]
pub type W = crate::W<TlvAdc10_1LenSpec>;
#[doc = "Field `TLV_ADC10_1_LEN` reader - TLV ADC10_1 LEN register"]
pub type TlvAdc10_1LenR = crate::FieldReader;
#[doc = "Field `TLV_ADC10_1_LEN` writer - TLV ADC10_1 LEN register"]
pub type TlvAdc10_1LenW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - TLV ADC10_1 LEN register"]
    #[inline(always)]
    pub fn tlv_adc10_1_len(&self) -> TlvAdc10_1LenR {
        TlvAdc10_1LenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV ADC10_1 LEN register"]
    #[inline(always)]
    pub fn tlv_adc10_1_len(&mut self) -> TlvAdc10_1LenW<TlvAdc10_1LenSpec> {
        TlvAdc10_1LenW::new(self, 0)
    }
}
#[doc = "TLV ADC10_1 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvAdc10_1LenSpec;
impl crate::RegisterSpec for TlvAdc10_1LenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_adc10_1_len::R`](R) reader structure"]
impl crate::Readable for TlvAdc10_1LenSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_adc10_1_len::W`](W) writer structure"]
impl crate::Writable for TlvAdc10_1LenSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_ADC10_1_LEN to value 0"]
impl crate::Resettable for TlvAdc10_1LenSpec {
    const RESET_VALUE: u8 = 0;
}
