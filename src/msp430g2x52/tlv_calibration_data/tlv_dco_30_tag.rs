#[doc = "Register `TLV_DCO_30_TAG` reader"]
pub type R = crate::R<TlvDco30TagSpec>;
#[doc = "Register `TLV_DCO_30_TAG` writer"]
pub type W = crate::W<TlvDco30TagSpec>;
#[doc = "Field `TLV_DCO_30_TAG` reader - TLV TAG_DCO30 TAG register"]
pub type TlvDco30TagR = crate::FieldReader;
#[doc = "Field `TLV_DCO_30_TAG` writer - TLV TAG_DCO30 TAG register"]
pub type TlvDco30TagW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&self) -> TlvDco30TagR {
        TlvDco30TagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&mut self) -> TlvDco30TagW<TlvDco30TagSpec> {
        TlvDco30TagW::new(self, 0)
    }
}
#[doc = "TLV TAG_DCO30 TAG\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvDco30TagSpec;
impl crate::RegisterSpec for TlvDco30TagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_dco_30_tag::R`](R) reader structure"]
impl crate::Readable for TlvDco30TagSpec {}
#[doc = "`write(|w| ..)` method takes [`tlv_dco_30_tag::W`](W) writer structure"]
impl crate::Writable for TlvDco30TagSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_DCO_30_TAG to value 0"]
impl crate::Resettable for TlvDco30TagSpec {
    const RESET_VALUE: u8 = 0;
}
