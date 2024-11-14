#[doc = "Register `CALDCO_1MHZ` reader"]
pub type R = crate::R<CALDCO_1MHZ_SPEC>;
#[doc = "Register `CALDCO_1MHZ` writer"]
pub type W = crate::W<CALDCO_1MHZ_SPEC>;
#[doc = "Field `CALDCO_1MHZ` reader - DCOCTL Calibration Data register"]
pub type CALDCO_1MHZ_R = crate::FieldReader;
#[doc = "Field `CALDCO_1MHZ` writer - DCOCTL Calibration Data register"]
pub type CALDCO_1MHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data register"]
    #[inline(always)]
    pub fn caldco_1mhz(&self) -> CALDCO_1MHZ_R {
        CALDCO_1MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data register"]
    #[inline(always)]
    pub fn caldco_1mhz(&mut self) -> CALDCO_1MHZ_W<CALDCO_1MHZ_SPEC> {
        CALDCO_1MHZ_W::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALDCO_1MHZ_SPEC;
impl crate::RegisterSpec for CALDCO_1MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_1mhz::R`](R) reader structure"]
impl crate::Readable for CALDCO_1MHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`caldco_1mhz::W`](W) writer structure"]
impl crate::Writable for CALDCO_1MHZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALDCO_1MHZ to value 0"]
impl crate::Resettable for CALDCO_1MHZ_SPEC {
    const RESET_VALUE: u8 = 0;
}
