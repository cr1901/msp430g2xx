#[doc = "Register `TACCR2` reader"]
pub type R = crate::R<TACCR2_SPEC>;
#[doc = "Register `TACCR2` writer"]
pub type W = crate::W<TACCR2_SPEC>;
#[doc = "Field `TACCR2` reader - Timer A Capture/Compare register 2"]
pub type TACCR2_R = crate::FieldReader<u16>;
#[doc = "Field `TACCR2` writer - Timer A Capture/Compare register 2"]
pub type TACCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&self) -> TACCR2_R {
        TACCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&mut self) -> TACCR2_W<TACCR2_SPEC> {
        TACCR2_W::new(self, 0)
    }
}
#[doc = "Timer0_A3 Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACCR2_SPEC;
impl crate::RegisterSpec for TACCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr2::R`](R) reader structure"]
impl crate::Readable for TACCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taccr2::W`](W) writer structure"]
impl crate::Writable for TACCR2_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACCR2 to value 0"]
impl crate::Resettable for TACCR2_SPEC {
    const RESET_VALUE: u16 = 0;
}
