#[doc = "Register `UCB0BR0` reader"]
pub type R = crate::R<UCB0BR0_SPEC>;
#[doc = "Register `UCB0BR0` writer"]
pub type W = crate::W<UCB0BR0_SPEC>;
#[doc = "Field `UCB0BR0` reader - USCI B0 Baud Rate 0 register"]
pub type UCB0BR0_R = crate::FieldReader;
#[doc = "Field `UCB0BR0` writer - USCI B0 Baud Rate 0 register"]
pub type UCB0BR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn ucb0br0(&self) -> UCB0BR0_R {
        UCB0BR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn ucb0br0(&mut self) -> UCB0BR0_W<UCB0BR0_SPEC> {
        UCB0BR0_W::new(self, 0)
    }
}
#[doc = "USCI B0 Baud Rate 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0br0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0br0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0BR0_SPEC;
impl crate::RegisterSpec for UCB0BR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0br0::R`](R) reader structure"]
impl crate::Readable for UCB0BR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0br0::W`](W) writer structure"]
impl crate::Writable for UCB0BR0_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0BR0 to value 0"]
impl crate::Resettable for UCB0BR0_SPEC {
    const RESET_VALUE: u8 = 0;
}
