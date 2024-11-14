#[doc = "Register `TBCCR2` reader"]
pub type R = crate::R<TBCCR2_SPEC>;
#[doc = "Register `TBCCR2` writer"]
pub type W = crate::W<TBCCR2_SPEC>;
#[doc = "Field `TBCCR2` reader - Timer B Capture/Compare register 2"]
pub type TBCCR2_R = crate::FieldReader<u16>;
#[doc = "Field `TBCCR2` writer - Timer B Capture/Compare register 2"]
pub type TBCCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 2"]
    #[inline(always)]
    pub fn tbccr2(&self) -> TBCCR2_R {
        TBCCR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 2"]
    #[inline(always)]
    pub fn tbccr2(&mut self) -> TBCCR2_W<TBCCR2_SPEC> {
        TBCCR2_W::new(self, 0)
    }
}
#[doc = "Timer B Capture/Compare 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBCCR2_SPEC;
impl crate::RegisterSpec for TBCCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr2::R`](R) reader structure"]
impl crate::Readable for TBCCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbccr2::W`](W) writer structure"]
impl crate::Writable for TBCCR2_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBCCR2 to value 0"]
impl crate::Resettable for TBCCR2_SPEC {
    const RESET_VALUE: u16 = 0;
}
