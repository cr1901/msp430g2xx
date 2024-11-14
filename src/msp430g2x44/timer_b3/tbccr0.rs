#[doc = "Register `TBCCR0` reader"]
pub type R = crate::R<TBCCR0_SPEC>;
#[doc = "Register `TBCCR0` writer"]
pub type W = crate::W<TBCCR0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Capture/Compare 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tbccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBCCR0_SPEC;
impl crate::RegisterSpec for TBCCR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbccr0::R`](R) reader structure"]
impl crate::Readable for TBCCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbccr0::W`](W) writer structure"]
impl crate::Writable for TBCCR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBCCR0 to value 0"]
impl crate::Resettable for TBCCR0_SPEC {
    const RESET_VALUE: u16 = 0;
}
