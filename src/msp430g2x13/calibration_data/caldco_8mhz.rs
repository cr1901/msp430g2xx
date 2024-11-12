#[doc = "Register `CALDCO_8MHZ` reader"]
pub type R = crate::R<Caldco8mhzSpec>;
#[doc = "Register `CALDCO_8MHZ` writer"]
pub type W = crate::W<Caldco8mhzSpec>;
#[doc = "Field `CALDCO_8MHZ` reader - DCOCTL Calibration Data for 8MHz register"]
pub type Caldco8mhzR = crate::FieldReader;
#[doc = "Field `CALDCO_8MHZ` writer - DCOCTL Calibration Data for 8MHz register"]
pub type Caldco8mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn caldco_8mhz(&self) -> Caldco8mhzR {
        Caldco8mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn caldco_8mhz(&mut self) -> Caldco8mhzW<Caldco8mhzSpec> {
        Caldco8mhzW::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_8mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_8mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco8mhzSpec;
impl crate::RegisterSpec for Caldco8mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_8mhz::R`](R) reader structure"]
impl crate::Readable for Caldco8mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_8mhz::W`](W) writer structure"]
impl crate::Writable for Caldco8mhzSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALDCO_8MHZ to value 0"]
impl crate::Resettable for Caldco8mhzSpec {
    const RESET_VALUE: u8 = 0;
}
