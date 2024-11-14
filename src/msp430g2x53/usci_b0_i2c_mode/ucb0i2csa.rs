#[doc = "Register `UCB0I2CSA` reader"]
pub type R = crate::R<UCB0I2CSA_SPEC>;
#[doc = "Register `UCB0I2CSA` writer"]
pub type W = crate::W<UCB0I2CSA_SPEC>;
#[doc = "Field `UCSA` reader - I2C Slave Address 0"]
pub type UCSA_R = crate::FieldReader<u16>;
#[doc = "Field `UCSA` writer - I2C Slave Address 0"]
pub type UCSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:9 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa(&self) -> UCSA_R {
        UCSA_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa(&mut self) -> UCSA_W<UCB0I2CSA_SPEC> {
        UCSA_W::new(self, 0)
    }
}
#[doc = "USCI B0 I2C Slave Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0I2CSA_SPEC;
impl crate::RegisterSpec for UCB0I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2csa::R`](R) reader structure"]
impl crate::Readable for UCB0I2CSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2csa::W`](W) writer structure"]
impl crate::Writable for UCB0I2CSA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0I2CSA to value 0"]
impl crate::Resettable for UCB0I2CSA_SPEC {
    const RESET_VALUE: u16 = 0;
}
