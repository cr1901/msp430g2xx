#[doc = "Register `TACCR0` reader"]
pub type R = crate::R<TACCR0_SPEC>;
#[doc = "Register `TACCR0` writer"]
pub type W = crate::W<TACCR0_SPEC>;
#[doc = "Field `TACCR0` reader - Timer A Capture/Compare register 0"]
pub type TACCR0_R = crate::FieldReader<u16>;
#[doc = "Field `TACCR0` writer - Timer A Capture/Compare register 0"]
pub type TACCR0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&self) -> TACCR0_R {
        TACCR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&mut self) -> TACCR0_W<TACCR0_SPEC> {
        TACCR0_W::new(self, 0)
    }
}
#[doc = "Timer0_A3 Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`taccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACCR0_SPEC;
impl crate::RegisterSpec for TACCR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taccr0::R`](R) reader structure"]
impl crate::Readable for TACCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`taccr0::W`](W) writer structure"]
impl crate::Writable for TACCR0_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACCR0 to value 0"]
impl crate::Resettable for TACCR0_SPEC {
    const RESET_VALUE: u16 = 0;
}
