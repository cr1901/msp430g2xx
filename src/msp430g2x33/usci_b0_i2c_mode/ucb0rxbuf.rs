#[doc = "Register `UCB0RXBUF` reader"]
pub type R = crate::R<UCB0RXBUF_SPEC>;
#[doc = "Register `UCB0RXBUF` writer"]
pub type W = crate::W<UCB0RXBUF_SPEC>;
#[doc = "Field `UCB0RXBUF` reader - USCI B0 Receive Buffer register"]
pub type UCB0RXBUF_R = crate::FieldReader;
#[doc = "Field `UCB0RXBUF` writer - USCI B0 Receive Buffer register"]
pub type UCB0RXBUF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI B0 Receive Buffer register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&self) -> UCB0RXBUF_R {
        UCB0RXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Receive Buffer register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&mut self) -> UCB0RXBUF_W<UCB0RXBUF_SPEC> {
        UCB0RXBUF_W::new(self, 0)
    }
}
#[doc = "USCI B0 Receive Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0RXBUF_SPEC;
impl crate::RegisterSpec for UCB0RXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0rxbuf::R`](R) reader structure"]
impl crate::Readable for UCB0RXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0rxbuf::W`](W) writer structure"]
impl crate::Writable for UCB0RXBUF_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0RXBUF to value 0"]
impl crate::Resettable for UCB0RXBUF_SPEC {
    const RESET_VALUE: u8 = 0;
}
