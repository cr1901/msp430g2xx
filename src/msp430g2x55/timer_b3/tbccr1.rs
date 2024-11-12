#[doc = "Register `TBCCR1` reader"]
pub type R = crate::R<Tbccr1Spec>;
#[doc = "Register `TBCCR1` writer"]
pub type W = crate::W<Tbccr1Spec>;
#[doc = "Field `TBCCR1` reader - Timer B Capture/Compare register 1"]
pub type Tbccr1R = crate::FieldReader<u16>;
#[doc = "Field `TBCCR1` writer - Timer B Capture/Compare register 1"]
pub type Tbccr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 1"]
    #[inline(always)]
    pub fn tbccr1(&self) -> Tbccr1R {
        Tbccr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 1"]
    #[inline(always)]
    pub fn tbccr1(&mut self) -> Tbccr1W<Tbccr1Spec> {
        Tbccr1W::new(self, 0)
    }
}
#[doc = "Timer B Capture/Compare 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr1Spec;
impl crate::RegisterSpec for Tbccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr1::R`](R) reader structure"]
impl crate::Readable for Tbccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr1::W`](W) writer structure"]
impl crate::Writable for Tbccr1Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBCCR1 to value 0"]
impl crate::Resettable for Tbccr1Spec {
    const RESET_VALUE: u16 = 0;
}
