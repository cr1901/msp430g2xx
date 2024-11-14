#[doc = "Register `UCB0I2CIE` reader"]
pub type R = crate::R<UCB0I2CIE_SPEC>;
#[doc = "Register `UCB0I2CIE` writer"]
pub type W = crate::W<UCB0I2CIE_SPEC>;
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader;
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader;
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub type UCSTTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader;
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader;
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W<UCB0I2CIE_SPEC> {
        UCALIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<UCB0I2CIE_SPEC> {
        UCSTTIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<UCB0I2CIE_SPEC> {
        UCSTPIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<UCB0I2CIE_SPEC> {
        UCNACKIE_W::new(self, 3)
    }
}
#[doc = "USCI B0 I2C Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2cie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2cie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0I2CIE_SPEC;
impl crate::RegisterSpec for UCB0I2CIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0i2cie::R`](R) reader structure"]
impl crate::Readable for UCB0I2CIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2cie::W`](W) writer structure"]
impl crate::Writable for UCB0I2CIE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0I2CIE to value 0"]
impl crate::Resettable for UCB0I2CIE_SPEC {
    const RESET_VALUE: u8 = 0;
}
