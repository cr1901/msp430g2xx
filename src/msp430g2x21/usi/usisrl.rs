#[doc = "Register `USISRL` reader"]
pub type R = crate::R<UsisrlSpec>;
#[doc = "Register `USISRL` writer"]
pub type W = crate::W<UsisrlSpec>;
#[doc = "Field `USISRL` reader - USI Low Byte Shift Register"]
pub type UsisrlR = crate::FieldReader;
#[doc = "Field `USISRL` writer - USI Low Byte Shift Register"]
pub type UsisrlW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USI Low Byte Shift Register"]
    #[inline(always)]
    pub fn usisrl(&self) -> UsisrlR {
        UsisrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USI Low Byte Shift Register"]
    #[inline(always)]
    pub fn usisrl(&mut self) -> UsisrlW<UsisrlSpec> {
        UsisrlW::new(self, 0)
    }
}
#[doc = "USI Low Byte Shift Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usisrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usisrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsisrlSpec;
impl crate::RegisterSpec for UsisrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usisrl::R`](R) reader structure"]
impl crate::Readable for UsisrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usisrl::W`](W) writer structure"]
impl crate::Writable for UsisrlSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USISRL to value 0"]
impl crate::Resettable for UsisrlSpec {
    const RESET_VALUE: u8 = 0;
}
