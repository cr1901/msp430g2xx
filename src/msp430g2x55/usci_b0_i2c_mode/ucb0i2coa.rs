#[doc = "Register `UCB0I2COA` reader"]
pub type R = crate::R<UCB0I2COA_SPEC>;
#[doc = "Register `UCB0I2COA` writer"]
pub type W = crate::W<UCB0I2COA_SPEC>;
#[doc = "Field `UCOA` reader - I2C Own Address 0"]
pub type UCOA_R = crate::FieldReader<u16>;
#[doc = "Field `UCOA` writer - I2C Own Address 0"]
pub type UCOA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub type UCGCEN_R = crate::BitReader;
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub type UCGCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&self) -> UCOA_R {
        UCOA_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&mut self) -> UCOA_W<UCB0I2COA_SPEC> {
        UCOA_W::new(self, 0)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W<UCB0I2COA_SPEC> {
        UCGCEN_W::new(self, 15)
    }
}
#[doc = "USCI B0 I2C Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0I2COA_SPEC;
impl crate::RegisterSpec for UCB0I2COA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa::R`](R) reader structure"]
impl crate::Readable for UCB0I2COA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa::W`](W) writer structure"]
impl crate::Writable for UCB0I2COA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0I2COA to value 0"]
impl crate::Resettable for UCB0I2COA_SPEC {
    const RESET_VALUE: u16 = 0;
}
