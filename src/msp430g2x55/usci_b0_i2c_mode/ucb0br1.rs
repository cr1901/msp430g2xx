#[doc = "Register `UCB0BR1` reader"]
pub type R = crate::R<Ucb0br1Spec>;
#[doc = "Register `UCB0BR1` writer"]
pub type W = crate::W<Ucb0br1Spec>;
#[doc = "Field `UCB0BR1` reader - USCI B0 Baud Rate 1 register"]
pub type Ucb0br1R = crate::FieldReader;
#[doc = "Field `UCB0BR1` writer - USCI B0 Baud Rate 1 register"]
pub type Ucb0br1W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn ucb0br1(&self) -> Ucb0br1R {
        Ucb0br1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn ucb0br1(&mut self) -> Ucb0br1W<Ucb0br1Spec> {
        Ucb0br1W::new(self, 0)
    }
}
#[doc = "USCI B0 Baud Rate 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0br1Spec;
impl crate::RegisterSpec for Ucb0br1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0br1::R`](R) reader structure"]
impl crate::Readable for Ucb0br1Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0br1::W`](W) writer structure"]
impl crate::Writable for Ucb0br1Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0BR1 to value 0"]
impl crate::Resettable for Ucb0br1Spec {
    const RESET_VALUE: u8 = 0;
}
