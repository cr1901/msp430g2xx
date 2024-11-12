#[doc = "Register `UCA0RXBUF` reader"]
pub type R = crate::R<Uca0rxbufSpec>;
#[doc = "Register `UCA0RXBUF` writer"]
pub type W = crate::W<Uca0rxbufSpec>;
#[doc = "Field `UCA0RXBUF` reader - USCI A0 Receive Buffer register"]
pub type Uca0rxbufR = crate::FieldReader;
#[doc = "Field `UCA0RXBUF` writer - USCI A0 Receive Buffer register"]
pub type Uca0rxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> Uca0rxbufR {
        Uca0rxbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&mut self) -> Uca0rxbufW<Uca0rxbufSpec> {
        Uca0rxbufW::new(self, 0)
    }
}
#[doc = "USCI A0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0rxbufSpec;
impl crate::RegisterSpec for Uca0rxbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0rxbuf::R`](R) reader structure"]
impl crate::Readable for Uca0rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0rxbuf::W`](W) writer structure"]
impl crate::Writable for Uca0rxbufSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0RXBUF to value 0"]
impl crate::Resettable for Uca0rxbufSpec {
    const RESET_VALUE: u8 = 0;
}
