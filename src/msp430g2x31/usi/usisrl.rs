#[doc = "Register `USISRL` reader"]
pub type R = crate::R<USISRL_SPEC>;
#[doc = "Register `USISRL` writer"]
pub type W = crate::W<USISRL_SPEC>;
#[doc = "Field `USISRL` reader - USI Low Byte Shift Register"]
pub type USISRL_R = crate::FieldReader;
#[doc = "Field `USISRL` writer - USI Low Byte Shift Register"]
pub type USISRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USI Low Byte Shift Register"]
    #[inline(always)]
    pub fn usisrl(&self) -> USISRL_R {
        USISRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USI Low Byte Shift Register"]
    #[inline(always)]
    pub fn usisrl(&mut self) -> USISRL_W<USISRL_SPEC> {
        USISRL_W::new(self, 0)
    }
}
#[doc = "USI Low Byte Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usisrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usisrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USISRL_SPEC;
impl crate::RegisterSpec for USISRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usisrl::R`](R) reader structure"]
impl crate::Readable for USISRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usisrl::W`](W) writer structure"]
impl crate::Writable for USISRL_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USISRL to value 0"]
impl crate::Resettable for USISRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
