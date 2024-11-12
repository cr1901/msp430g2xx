#[doc = "Register `CALBC1_16MHZ` reader"]
pub type R = crate::R<Calbc1_16mhzSpec>;
#[doc = "Register `CALBC1_16MHZ` writer"]
pub type W = crate::W<Calbc1_16mhzSpec>;
#[doc = "Field `CALBC1_16MHZ` reader - BCSCTL1 Calibration Data for 16MHz register"]
pub type Calbc1_16mhzR = crate::FieldReader;
#[doc = "Field `CALBC1_16MHZ` writer - BCSCTL1 Calibration Data for 16MHz register"]
pub type Calbc1_16mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn calbc1_16mhz(&self) -> Calbc1_16mhzR {
        Calbc1_16mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn calbc1_16mhz(&mut self) -> Calbc1_16mhzW<Calbc1_16mhzSpec> {
        Calbc1_16mhzW::new(self, 0)
    }
}
#[doc = "BCSCTL1 Calibration Data for 16MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_16mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_16mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calbc1_16mhzSpec;
impl crate::RegisterSpec for Calbc1_16mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_16mhz::R`](R) reader structure"]
impl crate::Readable for Calbc1_16mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`calbc1_16mhz::W`](W) writer structure"]
impl crate::Writable for Calbc1_16mhzSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALBC1_16MHZ to value 0"]
impl crate::Resettable for Calbc1_16mhzSpec {
    const RESET_VALUE: u8 = 0;
}
