#[doc = "Register `CALDCO_12MHZ` reader"]
pub type R = crate::R<Caldco12mhzSpec>;
#[doc = "Register `CALDCO_12MHZ` writer"]
pub type W = crate::W<Caldco12mhzSpec>;
#[doc = "Field `CALDCO_12MHZ` reader - DCOCTL Calibration Data for 12MHz register"]
pub type Caldco12mhzR = crate::FieldReader;
#[doc = "Field `CALDCO_12MHZ` writer - DCOCTL Calibration Data for 12MHz register"]
pub type Caldco12mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn caldco_12mhz(&self) -> Caldco12mhzR {
        Caldco12mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn caldco_12mhz(&mut self) -> Caldco12mhzW<Caldco12mhzSpec> {
        Caldco12mhzW::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 12MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_12mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_12mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco12mhzSpec;
impl crate::RegisterSpec for Caldco12mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_12mhz::R`](R) reader structure"]
impl crate::Readable for Caldco12mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_12mhz::W`](W) writer structure"]
impl crate::Writable for Caldco12mhzSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALDCO_12MHZ to value 0"]
impl crate::Resettable for Caldco12mhzSpec {
    const RESET_VALUE: u8 = 0;
}
