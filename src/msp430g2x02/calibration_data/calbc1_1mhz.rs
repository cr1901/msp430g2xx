#[doc = "Register `CALBC1_1MHZ` reader"]
pub type R = crate::R<CALBC1_1MHZ_SPEC>;
#[doc = "Register `CALBC1_1MHZ` writer"]
pub type W = crate::W<CALBC1_1MHZ_SPEC>;
#[doc = "Field `CALBC1_1MHZ` reader - BCSCTL1 Calibration Data for 1MHz register"]
pub type CALBC1_1MHZ_R = crate::FieldReader;
#[doc = "Field `CALBC1_1MHZ` writer - BCSCTL1 Calibration Data for 1MHz register"]
pub type CALBC1_1MHZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn calbc1_1mhz(&self) -> CALBC1_1MHZ_R {
        CALBC1_1MHZ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn calbc1_1mhz(&mut self) -> CALBC1_1MHZ_W<CALBC1_1MHZ_SPEC> {
        CALBC1_1MHZ_W::new(self, 0)
    }
}
#[doc = "BCSCTL1 Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`calbc1_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calbc1_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALBC1_1MHZ_SPEC;
impl crate::RegisterSpec for CALBC1_1MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calbc1_1mhz::R`](R) reader structure"]
impl crate::Readable for CALBC1_1MHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calbc1_1mhz::W`](W) writer structure"]
impl crate::Writable for CALBC1_1MHZ_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CALBC1_1MHZ to value 0"]
impl crate::Resettable for CALBC1_1MHZ_SPEC {
    const RESET_VALUE: u8 = 0;
}
