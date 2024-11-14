#[doc = "Register `TBR` reader"]
pub type R = crate::R<TBR_SPEC>;
#[doc = "Register `TBR` writer"]
pub type W = crate::W<TBR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbr::W`](W) writer structure"]
impl crate::Writable for TBR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TBR_SPEC {
    const RESET_VALUE: u16 = 0;
}
