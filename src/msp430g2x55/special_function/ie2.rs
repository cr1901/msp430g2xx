#[doc = "Register `IE2` reader"]
pub type R = crate::R<IE2_SPEC>;
#[doc = "Register `IE2` writer"]
pub type W = crate::W<IE2_SPEC>;
#[doc = "Field `UCA0RXIE` reader - UCA0RXIE"]
pub type UCA0RXIE_R = crate::BitReader;
#[doc = "Field `UCA0RXIE` writer - UCA0RXIE"]
pub type UCA0RXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA0TXIE` reader - UCA0TXIE"]
pub type UCA0TXIE_R = crate::BitReader;
#[doc = "Field `UCA0TXIE` writer - UCA0TXIE"]
pub type UCA0TXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0RXIE` reader - UCB0RXIE"]
pub type UCB0RXIE_R = crate::BitReader;
#[doc = "Field `UCB0RXIE` writer - UCB0RXIE"]
pub type UCB0RXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0TXIE` reader - UCB0TXIE"]
pub type UCB0TXIE_R = crate::BitReader;
#[doc = "Field `UCB0TXIE` writer - UCB0TXIE"]
pub type UCB0TXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&self) -> UCA0RXIE_R {
        UCA0RXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&self) -> UCA0TXIE_R {
        UCA0TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&self) -> UCB0RXIE_R {
        UCB0RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&self) -> UCB0TXIE_R {
        UCB0TXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&mut self) -> UCA0RXIE_W<IE2_SPEC> {
        UCA0RXIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&mut self) -> UCA0TXIE_W<IE2_SPEC> {
        UCA0TXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&mut self) -> UCB0RXIE_W<IE2_SPEC> {
        UCB0RXIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&mut self) -> UCB0TXIE_W<IE2_SPEC> {
        UCB0TXIE_W::new(self, 3)
    }
}
#[doc = "Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ie2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE2_SPEC;
impl crate::RegisterSpec for IE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie2::R`](R) reader structure"]
impl crate::Readable for IE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie2::W`](W) writer structure"]
impl crate::Writable for IE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IE2 to value 0"]
impl crate::Resettable for IE2_SPEC {
    const RESET_VALUE: u8 = 0;
}
