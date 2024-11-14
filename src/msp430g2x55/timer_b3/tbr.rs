#[doc = "Register `TBR` reader"]
pub type R = crate::R<TBR_SPEC>;
#[doc = "Register `TBR` writer"]
pub type W = crate::W<TBR_SPEC>;
#[doc = "Field `TBR` reader - Timer B Counter Register"]
pub type TBR_R = crate::FieldReader<u16>;
#[doc = "Field `TBR` writer - Timer B Counter Register"]
pub type TBR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer B Counter Register"]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer B Counter Register"]
    #[inline(always)]
    pub fn tbr(&mut self) -> TBR_W<TBR_SPEC> {
        TBR_W::new(self, 0)
    }
}
#[doc = "Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbr::W`](W) writer structure"]
impl crate::Writable for TBR_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TBR_SPEC {
    const RESET_VALUE: u16 = 0;
}
