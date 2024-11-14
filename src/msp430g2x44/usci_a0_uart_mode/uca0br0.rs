#[doc = "Register `UCA0BR0` reader"]
pub type R = crate::R<UCA0BR0_SPEC>;
#[doc = "Register `UCA0BR0` writer"]
pub type W = crate::W<UCA0BR0_SPEC>;
#[doc = "Field `UCA0BR0` reader - USCI A0 Baud Rate 0 register"]
pub type UCA0BR0_R = crate::FieldReader;
#[doc = "Field `UCA0BR0` writer - USCI A0 Baud Rate 0 register"]
pub type UCA0BR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&self) -> UCA0BR0_R {
        UCA0BR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&mut self) -> UCA0BR0_W<UCA0BR0_SPEC> {
        UCA0BR0_W::new(self, 0)
    }
}
#[doc = "USCI A0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0BR0_SPEC;
impl crate::RegisterSpec for UCA0BR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0br0::R`](R) reader structure"]
impl crate::Readable for UCA0BR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0br0::W`](W) writer structure"]
impl crate::Writable for UCA0BR0_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0BR0 to value 0"]
impl crate::Resettable for UCA0BR0_SPEC {
    const RESET_VALUE: u8 = 0;
}
