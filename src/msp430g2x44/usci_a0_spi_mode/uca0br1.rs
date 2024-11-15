#[doc = "Register `UCA0BR1` reader"]
pub type R = crate::R<UCA0BR1_SPEC>;
#[doc = "Register `UCA0BR1` writer"]
pub type W = crate::W<UCA0BR1_SPEC>;
#[doc = "Field `UCA0BR1` reader - USCI A0 Baud Rate 1 register"]
pub type UCA0BR1_R = crate::FieldReader;
#[doc = "Field `UCA0BR1` writer - USCI A0 Baud Rate 1 register"]
pub type UCA0BR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn uca0br1(&self) -> UCA0BR1_R {
        UCA0BR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn uca0br1(&mut self) -> UCA0BR1_W<UCA0BR1_SPEC> {
        UCA0BR1_W::new(self, 0)
    }
}
#[doc = "USCI A0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0BR1_SPEC;
impl crate::RegisterSpec for UCA0BR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0br1::R`](R) reader structure"]
impl crate::Readable for UCA0BR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0br1::W`](W) writer structure"]
impl crate::Writable for UCA0BR1_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0BR1 to value 0"]
impl crate::Resettable for UCA0BR1_SPEC {
    const RESET_VALUE: u8 = 0;
}
