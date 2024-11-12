#[doc = "Register `UCB0I2CIE` reader"]
pub type R = crate::R<Ucb0i2cieSpec>;
#[doc = "Register `UCB0I2CIE` writer"]
pub type W = crate::W<Ucb0i2cieSpec>;
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub type UcalieR = crate::BitReader;
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub type UcalieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub type UcsttieR = crate::BitReader;
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub type UcsttieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub type UcstpieR = crate::BitReader;
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub type UcstpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub type UcnackieR = crate::BitReader;
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub type UcnackieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UcalieR {
        UcalieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UcsttieR {
        UcsttieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UcstpieR {
        UcstpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UcnackieR {
        UcnackieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UcalieW<Ucb0i2cieSpec> {
        UcalieW::new(self, 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UcsttieW<Ucb0i2cieSpec> {
        UcsttieW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UcstpieW<Ucb0i2cieSpec> {
        UcstpieW::new(self, 2)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UcnackieW<Ucb0i2cieSpec> {
        UcnackieW::new(self, 3)
    }
}
#[doc = "USCI B0 I2C Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2cie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2cie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2cieSpec;
impl crate::RegisterSpec for Ucb0i2cieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0i2cie::R`](R) reader structure"]
impl crate::Readable for Ucb0i2cieSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2cie::W`](W) writer structure"]
impl crate::Writable for Ucb0i2cieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0I2CIE to value 0"]
impl crate::Resettable for Ucb0i2cieSpec {
    const RESET_VALUE: u8 = 0;
}
