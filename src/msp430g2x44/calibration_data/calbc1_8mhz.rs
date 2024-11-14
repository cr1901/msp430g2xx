#[doc = "Register `CALBC1_8MHZ` reader"]
pub type R = crate::R<CALBC1_8MHZ_SPEC>;
#[doc = "Register `CALBC1_8MHZ` writer"]
pub type W = crate::W<CALBC1_8MHZ_SPEC>;
#[doc = "Field `CALBC1_8MHZ` reader - BCSCTL1 Calibration Data for 8MHz register"]
pub type CALBC1_8MHZ_R = crate::FieldReader;
#[doc = "Field `CALBC1_8MHZ` writer - BCSCTL1 Calibration Data for 8MHz register"]
pub type CALBC1_8MHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn calbc1_8mhz(&self) -> CALBC1_8MHZ_R {
        CALBC1_8MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 8MHz register"]
    #[inline(always)]
    pub fn calbc1_8mhz(&mut self) -> CALBC1_8MHZ_W<CALBC1_8MHZ_SPEC> {
        CALBC1_8MHZ_W::new(self, 0)
    }
}
#[doc = "BCSCTL1 Calibration Data for 8MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_8mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_8mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALBC1_8MHZ_SPEC;
impl crate::RegisterSpec for CALBC1_8MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_8mhz::R`](R) reader structure"]
impl crate::Readable for CALBC1_8MHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calbc1_8mhz::W`](W) writer structure"]
impl crate::Writable for CALBC1_8MHZ_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALBC1_8MHZ to value 0"]
impl crate::Resettable for CALBC1_8MHZ_SPEC {
    const RESET_VALUE: u8 = 0;
}
