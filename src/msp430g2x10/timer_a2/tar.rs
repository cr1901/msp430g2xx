#[doc = "Register `TAR` reader"]
pub type R = crate::R<TAR_SPEC>;
#[doc = "Register `TAR` writer"]
pub type W = crate::W<TAR_SPEC>;
#[doc = "Field `TAR` reader - Timer A Counter Register"]
pub type TAR_R = crate::FieldReader<u16>;
#[doc = "Field `TAR` writer - Timer A Counter Register"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<TAR_SPEC> {
        TAR_W::new(self, 0)
    }
}
#[doc = "Timer A Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAR to value 0"]
impl crate::Resettable for TAR_SPEC {
    const RESET_VALUE: u16 = 0;
}
