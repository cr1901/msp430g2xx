#[doc = "Register `TACCR2` reader"]
pub type R = crate::R<Taccr2Spec>;
#[doc = "Register `TACCR2` writer"]
pub type W = crate::W<Taccr2Spec>;
#[doc = "Field `TACCR2` reader - Timer A Capture/Compare register 2"]
pub type Taccr2R = crate::FieldReader<u16>;
#[doc = "Field `TACCR2` writer - Timer A Capture/Compare register 2"]
pub type Taccr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&self) -> Taccr2R {
        Taccr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&mut self) -> Taccr2W<Taccr2Spec> {
        Taccr2W::new(self, 0)
    }
}
#[doc = "Timer A Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Taccr2Spec;
impl crate::RegisterSpec for Taccr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr2::R`](R) reader structure"]
impl crate::Readable for Taccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`taccr2::W`](W) writer structure"]
impl crate::Writable for Taccr2Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACCR2 to value 0"]
impl crate::Resettable for Taccr2Spec {
    const RESET_VALUE: u16 = 0;
}
