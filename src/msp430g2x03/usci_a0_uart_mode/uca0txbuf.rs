#[doc = "Register `UCA0TXBUF` reader"]
pub type R = crate::R<UCA0TXBUF_SPEC>;
#[doc = "Register `UCA0TXBUF` writer"]
pub type W = crate::W<UCA0TXBUF_SPEC>;
#[doc = "Field `UCA0TXBUF` reader - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_R = crate::FieldReader;
#[doc = "Field `UCA0TXBUF` writer - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> UCA0TXBUF_R {
        UCA0TXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&mut self) -> UCA0TXBUF_W<UCA0TXBUF_SPEC> {
        UCA0TXBUF_W::new(self, 0)
    }
}
#[doc = "USCI A0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0TXBUF_SPEC;
impl crate::RegisterSpec for UCA0TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0txbuf::R`](R) reader structure"]
impl crate::Readable for UCA0TXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0txbuf::W`](W) writer structure"]
impl crate::Writable for UCA0TXBUF_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0TXBUF to value 0"]
impl crate::Resettable for UCA0TXBUF_SPEC {
    const RESET_VALUE: u8 = 0;
}
