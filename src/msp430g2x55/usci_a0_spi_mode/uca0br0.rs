#[doc = "Register `UCA0BR0` reader"]
pub type R = crate::R<Uca0br0Spec>;
#[doc = "Register `UCA0BR0` writer"]
pub type W = crate::W<Uca0br0Spec>;
#[doc = "Field `UCA0BR0` reader - USCI A0 Baud Rate 0 register"]
pub type Uca0br0R = crate::FieldReader;
#[doc = "Field `UCA0BR0` writer - USCI A0 Baud Rate 0 register"]
pub type Uca0br0W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&self) -> Uca0br0R {
        Uca0br0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&mut self) -> Uca0br0W<Uca0br0Spec> {
        Uca0br0W::new(self, 0)
    }
}
#[doc = "USCI A0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0br0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0br0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0br0Spec;
impl crate::RegisterSpec for Uca0br0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0br0::R`](R) reader structure"]
impl crate::Readable for Uca0br0Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0br0::W`](W) writer structure"]
impl crate::Writable for Uca0br0Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0BR0 to value 0"]
impl crate::Resettable for Uca0br0Spec {
    const RESET_VALUE: u8 = 0;
}
