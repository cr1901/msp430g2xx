#[doc = "Register `TACCR0` reader"]
pub type R = crate::R<Taccr0Spec>;
#[doc = "Register `TACCR0` writer"]
pub type W = crate::W<Taccr0Spec>;
#[doc = "Field `TACCR0` reader - Timer A Capture/Compare register 0"]
pub type Taccr0R = crate::FieldReader<u16>;
#[doc = "Field `TACCR0` writer - Timer A Capture/Compare register 0"]
pub type Taccr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&self) -> Taccr0R {
        Taccr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&mut self) -> Taccr0W<Taccr0Spec> {
        Taccr0W::new(self, 0)
    }
}
#[doc = "Timer A Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Taccr0Spec;
impl crate::RegisterSpec for Taccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr0::R`](R) reader structure"]
impl crate::Readable for Taccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`taccr0::W`](W) writer structure"]
impl crate::Writable for Taccr0Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACCR0 to value 0"]
impl crate::Resettable for Taccr0Spec {
    const RESET_VALUE: u16 = 0;
}
