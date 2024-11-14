#[doc = "Register `IFG2` reader"]
pub type R = crate::R<IFG2_SPEC>;
#[doc = "Register `IFG2` writer"]
pub type W = crate::W<IFG2_SPEC>;
#[doc = "Field `UCA0RXIFG` reader - UCA0RXIFG"]
pub type UCA0RXIFG_R = crate::BitReader;
#[doc = "Field `UCA0RXIFG` writer - UCA0RXIFG"]
pub type UCA0RXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA0TXIFG` reader - UCA0TXIFG"]
pub type UCA0TXIFG_R = crate::BitReader;
#[doc = "Field `UCA0TXIFG` writer - UCA0TXIFG"]
pub type UCA0TXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0RXIFG` reader - UCB0RXIFG"]
pub type UCB0RXIFG_R = crate::BitReader;
#[doc = "Field `UCB0RXIFG` writer - UCB0RXIFG"]
pub type UCB0RXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0TXIFG` reader - UCB0TXIFG"]
pub type UCB0TXIFG_R = crate::BitReader;
#[doc = "Field `UCB0TXIFG` writer - UCB0TXIFG"]
pub type UCB0TXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&self) -> UCA0RXIFG_R {
        UCA0RXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&self) -> UCA0TXIFG_R {
        UCA0TXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&self) -> UCB0RXIFG_R {
        UCB0RXIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&self) -> UCB0TXIFG_R {
        UCB0TXIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&mut self) -> UCA0RXIFG_W<IFG2_SPEC> {
        UCA0RXIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&mut self) -> UCA0TXIFG_W<IFG2_SPEC> {
        UCA0TXIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&mut self) -> UCB0RXIFG_W<IFG2_SPEC> {
        UCB0RXIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&mut self) -> UCB0TXIFG_W<IFG2_SPEC> {
        UCB0TXIFG_W::new(self, 3)
    }
}
#[doc = "Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFG2_SPEC;
impl crate::RegisterSpec for IFG2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg2::R`](R) reader structure"]
impl crate::Readable for IFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifg2::W`](W) writer structure"]
impl crate::Writable for IFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IFG2 to value 0"]
impl crate::Resettable for IFG2_SPEC {
    const RESET_VALUE: u8 = 0;
}
