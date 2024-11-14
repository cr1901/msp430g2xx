#[doc = "Register `TLV_CHECKSUM` reader"]
pub type R = crate::R<TLV_CHECKSUM_SPEC>;
#[doc = "Register `TLV_CHECKSUM` writer"]
pub type W = crate::W<TLV_CHECKSUM_SPEC>;
#[doc = "Field `TLV_CHECKSUM` reader - TLV CHECK SUM register"]
pub type TLV_CHECKSUM_R = crate::FieldReader<u16>;
#[doc = "Field `TLV_CHECKSUM` writer - TLV CHECK SUM register"]
pub type TLV_CHECKSUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&self) -> TLV_CHECKSUM_R {
        TLV_CHECKSUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&mut self) -> TLV_CHECKSUM_W<TLV_CHECKSUM_SPEC> {
        TLV_CHECKSUM_W::new(self, 0)
    }
}
#[doc = "TLV CHECK SUM\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlv_checksum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLV_CHECKSUM_SPEC;
impl crate::RegisterSpec for TLV_CHECKSUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tlv_checksum::R`](R) reader structure"]
impl crate::Readable for TLV_CHECKSUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tlv_checksum::W`](W) writer structure"]
impl crate::Writable for TLV_CHECKSUM_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TLV_CHECKSUM to value 0"]
impl crate::Resettable for TLV_CHECKSUM_SPEC {
    const RESET_VALUE: u16 = 0;
}
