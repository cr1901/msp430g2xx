#[doc = "Register `UCB0STAT` reader"]
pub type R = crate::R<UCB0STAT_SPEC>;
#[doc = "Register `UCB0STAT` writer"]
pub type W = crate::W<UCB0STAT_SPEC>;
#[doc = "Field `UCALIFG` reader - Arbitration Lost interrupt Flag"]
pub type UCALIFG_R = crate::BitReader;
#[doc = "Field `UCALIFG` writer - Arbitration Lost interrupt Flag"]
pub type UCALIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIFG` reader - START Condition interrupt Flag"]
pub type UCSTTIFG_R = crate::BitReader;
#[doc = "Field `UCSTTIFG` writer - START Condition interrupt Flag"]
pub type UCSTTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIFG` reader - STOP Condition interrupt Flag"]
pub type UCSTPIFG_R = crate::BitReader;
#[doc = "Field `UCSTPIFG` writer - STOP Condition interrupt Flag"]
pub type UCSTPIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIFG` reader - NAK Condition interrupt Flag"]
pub type UCNACKIFG_R = crate::BitReader;
#[doc = "Field `UCNACKIFG` writer - NAK Condition interrupt Flag"]
pub type UCNACKIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UCBBUSY_R = crate::BitReader;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UCBBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UCGC_R = crate::BitReader;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UCGC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UCSCLLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UCLISTEN_R = crate::BitReader;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UCLISTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W<UCB0STAT_SPEC> {
        UCALIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W<UCB0STAT_SPEC> {
        UCSTTIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W<UCB0STAT_SPEC> {
        UCSTPIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W<UCB0STAT_SPEC> {
        UCNACKIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W<UCB0STAT_SPEC> {
        UCBBUSY_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UCGC_W<UCB0STAT_SPEC> {
        UCGC_W::new(self, 5)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W<UCB0STAT_SPEC> {
        UCSCLLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W<UCB0STAT_SPEC> {
        UCLISTEN_W::new(self, 7)
    }
}
#[doc = "USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0STAT_SPEC;
impl crate::RegisterSpec for UCB0STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0stat::R`](R) reader structure"]
impl crate::Readable for UCB0STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0stat::W`](W) writer structure"]
impl crate::Writable for UCB0STAT_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0STAT to value 0"]
impl crate::Resettable for UCB0STAT_SPEC {
    const RESET_VALUE: u8 = 0;
}
