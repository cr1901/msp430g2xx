#[doc = "Register `UCB0TXBUF` reader"]
pub type R = crate::R<UCB0TXBUF_SPEC>;
#[doc = "Register `UCB0TXBUF` writer"]
pub type W = crate::W<UCB0TXBUF_SPEC>;
#[doc = "Field `UCB0TXBUF` reader - USCI B0 Transmit Buffer register"]
pub type UCB0TXBUF_R = crate::FieldReader;
#[doc = "Field `UCB0TXBUF` writer - USCI B0 Transmit Buffer register"]
pub type UCB0TXBUF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> UCB0TXBUF_R {
        UCB0TXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&mut self) -> UCB0TXBUF_W<UCB0TXBUF_SPEC> {
        UCB0TXBUF_W::new(self, 0)
    }
}
#[doc = "USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0TXBUF_SPEC;
impl crate::RegisterSpec for UCB0TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0txbuf::R`](R) reader structure"]
impl crate::Readable for UCB0TXBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0txbuf::W`](W) writer structure"]
impl crate::Writable for UCB0TXBUF_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0TXBUF to value 0"]
impl crate::Resettable for UCB0TXBUF_SPEC {
    const RESET_VALUE: u8 = 0;
}
