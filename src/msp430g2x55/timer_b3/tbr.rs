#[doc = "Register `TBR` reader"]
pub type R = crate::R<TbrSpec>;
#[doc = "Register `TBR` writer"]
pub type W = crate::W<TbrSpec>;
#[doc = "Field `TBR` reader - Timer B Counter Register"]
pub type TbrR = crate::FieldReader<u16>;
#[doc = "Field `TBR` writer - Timer B Counter Register"]
pub type TbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Timer B Counter Register"]
    #[inline(always)]
    pub fn tbr(&self) -> TbrR {
        TbrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer B Counter Register"]
    #[inline(always)]
    pub fn tbr(&mut self) -> TbrW<TbrSpec> {
        TbrW::new(self, 0)
    }
}
#[doc = "Timer B Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbrSpec;
impl crate::RegisterSpec for TbrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TbrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbr::W`](W) writer structure"]
impl crate::Writable for TbrSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TbrSpec {
    const RESET_VALUE: u16 = 0;
}
