#[doc = "Register `TACCR1` reader"]
pub type R = crate::R<TACCR1_SPEC>;
#[doc = "Register `TACCR1` writer"]
pub type W = crate::W<TACCR1_SPEC>;
#[doc = "Field `TACCR1` reader - Timer A Capture/Compare register 1"]
pub type TACCR1_R = crate::FieldReader<u16>;
#[doc = "Field `TACCR1` writer - Timer A Capture/Compare register 1"]
pub type TACCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&self) -> TACCR1_R {
        TACCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&mut self) -> TACCR1_W<TACCR1_SPEC> {
        TACCR1_W::new(self, 0)
    }
}
#[doc = "Timer A Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACCR1_SPEC;
impl crate::RegisterSpec for TACCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr1::R`](R) reader structure"]
impl crate::Readable for TACCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taccr1::W`](W) writer structure"]
impl crate::Writable for TACCR1_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACCR1 to value 0"]
impl crate::Resettable for TACCR1_SPEC {
    const RESET_VALUE: u16 = 0;
}
