#[doc = "Register `TBIV` reader"]
pub type R = crate::R<TBIV_SPEC>;
#[doc = "Register `TBIV` writer"]
pub type W = crate::W<TBIV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBIV_SPEC;
impl crate::RegisterSpec for TBIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbiv::R`](R) reader structure"]
impl crate::Readable for TBIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbiv::W`](W) writer structure"]
impl crate::Writable for TBIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBIV to value 0"]
impl crate::Resettable for TBIV_SPEC {
    const RESET_VALUE: u16 = 0;
}
