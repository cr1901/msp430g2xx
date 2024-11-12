#[doc = "Register `UCB0STAT` reader"]
pub type R = crate::R<Ucb0statSpec>;
#[doc = "Register `UCB0STAT` writer"]
pub type W = crate::W<Ucb0statSpec>;
#[doc = "Field `UCALIFG` reader - Arbitration Lost interrupt Flag"]
pub type UcalifgR = crate::BitReader;
#[doc = "Field `UCALIFG` writer - Arbitration Lost interrupt Flag"]
pub type UcalifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIFG` reader - START Condition interrupt Flag"]
pub type UcsttifgR = crate::BitReader;
#[doc = "Field `UCSTTIFG` writer - START Condition interrupt Flag"]
pub type UcsttifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIFG` reader - STOP Condition interrupt Flag"]
pub type UcstpifgR = crate::BitReader;
#[doc = "Field `UCSTPIFG` writer - STOP Condition interrupt Flag"]
pub type UcstpifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIFG` reader - NAK Condition interrupt Flag"]
pub type UcnackifgR = crate::BitReader;
#[doc = "Field `UCNACKIFG` writer - NAK Condition interrupt Flag"]
pub type UcnackifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UcbbusyR = crate::BitReader;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UcbbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UcgcR = crate::BitReader;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UcgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UcscllowR = crate::BitReader;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UcscllowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UclistenR = crate::BitReader;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UclistenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UcalifgR {
        UcalifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UcsttifgR {
        UcsttifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UcstpifgR {
        UcstpifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UcnackifgR {
        UcnackifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UcbbusyR {
        UcbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UcgcR {
        UcgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UcscllowR {
        UcscllowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UclistenR {
        UclistenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UcalifgW<Ucb0statSpec> {
        UcalifgW::new(self, 0)
    }
    #[doc = "Bit 1 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UcsttifgW<Ucb0statSpec> {
        UcsttifgW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UcstpifgW<Ucb0statSpec> {
        UcstpifgW::new(self, 2)
    }
    #[doc = "Bit 3 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UcnackifgW<Ucb0statSpec> {
        UcnackifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UcbbusyW<Ucb0statSpec> {
        UcbbusyW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UcgcW<Ucb0statSpec> {
        UcgcW::new(self, 5)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UcscllowW<Ucb0statSpec> {
        UcscllowW::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UclistenW<Ucb0statSpec> {
        UclistenW::new(self, 7)
    }
}
#[doc = "USCI B0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0statSpec;
impl crate::RegisterSpec for Ucb0statSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0stat::R`](R) reader structure"]
impl crate::Readable for Ucb0statSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0stat::W`](W) writer structure"]
impl crate::Writable for Ucb0statSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0STAT to value 0"]
impl crate::Resettable for Ucb0statSpec {
    const RESET_VALUE: u8 = 0;
}
