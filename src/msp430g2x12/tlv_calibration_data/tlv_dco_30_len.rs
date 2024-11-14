#[doc = "Register `TLV_DCO_30_LEN` reader"]
pub type R = crate::R<TLV_DCO_30_LEN_SPEC>;
#[doc = "Register `TLV_DCO_30_LEN` writer"]
pub type W = crate::W<TLV_DCO_30_LEN_SPEC>;
#[doc = "Field `TLV_DCO_30_LEN` reader - TLV TAG_DCO30 LEN register"]
pub type TLV_DCO_30_LEN_R = crate::FieldReader;
#[doc = "Field `TLV_DCO_30_LEN` writer - TLV TAG_DCO30 LEN register"]
pub type TLV_DCO_30_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 LEN register"]
    #[inline(always)]
    pub fn tlv_dco_30_len(&self) -> TLV_DCO_30_LEN_R {
        TLV_DCO_30_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 LEN register"]
    #[inline(always)]
    pub fn tlv_dco_30_len(&mut self) -> TLV_DCO_30_LEN_W<TLV_DCO_30_LEN_SPEC> {
        TLV_DCO_30_LEN_W::new(self, 0)
    }
}
#[doc = "TLV TAG_DCO30 LEN\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_dco_30_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_dco_30_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLV_DCO_30_LEN_SPEC;
impl crate::RegisterSpec for TLV_DCO_30_LEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tlv_dco_30_len::R`](R) reader structure"]
impl crate::Readable for TLV_DCO_30_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tlv_dco_30_len::W`](W) writer structure"]
impl crate::Writable for TLV_DCO_30_LEN_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TLV_DCO_30_LEN to value 0"]
impl crate::Resettable for TLV_DCO_30_LEN_SPEC {
    const RESET_VALUE: u8 = 0;
}
