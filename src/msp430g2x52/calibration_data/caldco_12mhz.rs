#[doc = "Register `CALDCO_12MHZ` reader"]
pub type R = crate::R<CALDCO_12MHZ_SPEC>;
#[doc = "Register `CALDCO_12MHZ` writer"]
pub type W = crate::W<CALDCO_12MHZ_SPEC>;
#[doc = "Field `CALDCO_12MHZ` reader - DCOCTL Calibration Data for 12MHz register"]
pub type CALDCO_12MHZ_R = crate::FieldReader;
#[doc = "Field `CALDCO_12MHZ` writer - DCOCTL Calibration Data for 12MHz register"]
pub type CALDCO_12MHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn caldco_12mhz(&self) -> CALDCO_12MHZ_R {
        CALDCO_12MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn caldco_12mhz(&mut self) -> CALDCO_12MHZ_W<CALDCO_12MHZ_SPEC> {
        CALDCO_12MHZ_W::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_12mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_12mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALDCO_12MHZ_SPEC;
impl crate::RegisterSpec for CALDCO_12MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_12mhz::R`](R) reader structure"]
impl crate::Readable for CALDCO_12MHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`caldco_12mhz::W`](W) writer structure"]
impl crate::Writable for CALDCO_12MHZ_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALDCO_12MHZ to value 0"]
impl crate::Resettable for CALDCO_12MHZ_SPEC {
    const RESET_VALUE: u8 = 0;
}
