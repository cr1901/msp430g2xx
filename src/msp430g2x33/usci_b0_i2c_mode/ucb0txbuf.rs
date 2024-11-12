#[doc = "Register `UCB0TXBUF` reader"]
pub type R = crate::R<Ucb0txbufSpec>;
#[doc = "Register `UCB0TXBUF` writer"]
pub type W = crate::W<Ucb0txbufSpec>;
#[doc = "Field `UCB0TXBUF` reader - USCI B0 Transmit Buffer register"]
pub type Ucb0txbufR = crate::FieldReader;
#[doc = "Field `UCB0TXBUF` writer - USCI B0 Transmit Buffer register"]
pub type Ucb0txbufW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> Ucb0txbufR {
        Ucb0txbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&mut self) -> Ucb0txbufW<Ucb0txbufSpec> {
        Ucb0txbufW::new(self, 0)
    }
}
#[doc = "USCI B0 Transmit Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0txbufSpec;
impl crate::RegisterSpec for Ucb0txbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0txbuf::R`](R) reader structure"]
impl crate::Readable for Ucb0txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0txbuf::W`](W) writer structure"]
impl crate::Writable for Ucb0txbufSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0TXBUF to value 0"]
impl crate::Resettable for Ucb0txbufSpec {
    const RESET_VALUE: u8 = 0;
}
