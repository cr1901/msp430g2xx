#[doc = "Register `UCB0I2COA` reader"]
pub type R = crate::R<Ucb0i2coaSpec>;
#[doc = "Register `UCB0I2COA` writer"]
pub type W = crate::W<Ucb0i2coaSpec>;
#[doc = "Field `UCOA` reader - I2C Own Address 0"]
pub type UcoaR = crate::FieldReader<u16>;
#[doc = "Field `UCOA` writer - I2C Own Address 0"]
pub type UcoaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub type UcgcenR = crate::BitReader;
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub type UcgcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&self) -> UcoaR {
        UcoaR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UcgcenR {
        UcgcenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&mut self) -> UcoaW<Ucb0i2coaSpec> {
        UcoaW::new(self, 0)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UcgcenW<Ucb0i2coaSpec> {
        UcgcenW::new(self, 15)
    }
}
#[doc = "USCI B0 I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2coaSpec;
impl crate::RegisterSpec for Ucb0i2coaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa::R`](R) reader structure"]
impl crate::Readable for Ucb0i2coaSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa::W`](W) writer structure"]
impl crate::Writable for Ucb0i2coaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0I2COA to value 0"]
impl crate::Resettable for Ucb0i2coaSpec {
    const RESET_VALUE: u16 = 0;
}
