#[doc = "Register `CALDCO_16MHZ` reader"]
pub type R = crate::R<Caldco16mhzSpec>;
#[doc = "Register `CALDCO_16MHZ` writer"]
pub type W = crate::W<Caldco16mhzSpec>;
#[doc = "Field `CALDCO_16MHZ` reader - DCOCTL Calibration Data for 16MHz register"]
pub type Caldco16mhzR = crate::FieldReader;
#[doc = "Field `CALDCO_16MHZ` writer - DCOCTL Calibration Data for 16MHz register"]
pub type Caldco16mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&self) -> Caldco16mhzR {
        Caldco16mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&mut self) -> Caldco16mhzW<Caldco16mhzSpec> {
        Caldco16mhzW::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_16mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_16mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco16mhzSpec;
impl crate::RegisterSpec for Caldco16mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_16mhz::R`](R) reader structure"]
impl crate::Readable for Caldco16mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_16mhz::W`](W) writer structure"]
impl crate::Writable for Caldco16mhzSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALDCO_16MHZ to value 0"]
impl crate::Resettable for Caldco16mhzSpec {
    const RESET_VALUE: u8 = 0;
}
