#[doc = "Register `USISRH` reader"]
pub type R = crate::R<USISRH_SPEC>;
#[doc = "Register `USISRH` writer"]
pub type W = crate::W<USISRH_SPEC>;
#[doc = "Field `USISRH` reader - USI High Byte Shift Register"]
pub type USISRH_R = crate::FieldReader;
#[doc = "Field `USISRH` writer - USI High Byte Shift Register"]
pub type USISRH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USI High Byte Shift Register"]
    #[inline(always)]
    pub fn usisrh(&self) -> USISRH_R {
        USISRH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USI High Byte Shift Register"]
    #[inline(always)]
    pub fn usisrh(&mut self) -> USISRH_W<USISRH_SPEC> {
        USISRH_W::new(self, 0)
    }
}
#[doc = "USI High Byte Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usisrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usisrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USISRH_SPEC;
impl crate::RegisterSpec for USISRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usisrh::R`](R) reader structure"]
impl crate::Readable for USISRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usisrh::W`](W) writer structure"]
impl crate::Writable for USISRH_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USISRH to value 0"]
impl crate::Resettable for USISRH_SPEC {
    const RESET_VALUE: u8 = 0;
}
