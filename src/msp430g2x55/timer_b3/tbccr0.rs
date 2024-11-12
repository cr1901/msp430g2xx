#[doc = "Register `TBCCR0` reader"]
pub type R = crate::R<Tbccr0Spec>;
#[doc = "Register `TBCCR0` writer"]
pub type W = crate::W<Tbccr0Spec>;
#[doc = "Field `TBCCR0` reader - Timer B Capture/Compare register 0"]
pub type Tbccr0R = crate::FieldReader<u16>;
#[doc = "Field `TBCCR0` writer - Timer B Capture/Compare register 0"]
pub type Tbccr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 0"]
    #[inline(always)]
    pub fn tbccr0(&self) -> Tbccr0R {
        Tbccr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer B Capture/Compare register 0"]
    #[inline(always)]
    pub fn tbccr0(&mut self) -> Tbccr0W<Tbccr0Spec> {
        Tbccr0W::new(self, 0)
    }
}
#[doc = "Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbccr0Spec;
impl crate::RegisterSpec for Tbccr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr0::R`](R) reader structure"]
impl crate::Readable for Tbccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tbccr0::W`](W) writer structure"]
impl crate::Writable for Tbccr0Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBCCR0 to value 0"]
impl crate::Resettable for Tbccr0Spec {
    const RESET_VALUE: u16 = 0;
}
