#[doc = "Register `TLV_ADC10_1_TAG` reader"]
pub type R = crate::R<TlvAdc10_1TagSpec>;
#[doc = "Register `TLV_ADC10_1_TAG` writer"]
pub type W = crate::W<TlvAdc10_1TagSpec>;
#[doc = "Field `TLV_ADC10_1_TAG` reader - TLV ADC10_1 TAG register"]
pub type TlvAdc10_1TagR = crate::FieldReader;
#[doc = "Field `TLV_ADC10_1_TAG` writer - TLV ADC10_1 TAG register"]
pub type TlvAdc10_1TagW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&self) -> TlvAdc10_1TagR {
        TlvAdc10_1TagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&mut self) -> TlvAdc10_1TagW<TlvAdc10_1TagSpec> {
        TlvAdc10_1TagW::new(self, 0)
    }
}
#[doc = "TLV ADC10_1 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_adc10_1_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_adc10_1_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvAdc10_1TagSpec;
impl crate::RegisterSpec for TlvAdc10_1TagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_adc10_1_tag::R`](R) reader structure"]
impl crate::Readable for TlvAdc10_1TagSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_adc10_1_tag::W`](W) writer structure"]
impl crate::Writable for TlvAdc10_1TagSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_ADC10_1_TAG to value 0"]
impl crate::Resettable for TlvAdc10_1TagSpec {
    const RESET_VALUE: u8 = 0;
}
