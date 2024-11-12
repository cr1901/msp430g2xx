#[doc = "Register `USICTL1` reader"]
pub type R = crate::R<Usictl1Spec>;
#[doc = "Register `USICTL1` writer"]
pub type W = crate::W<Usictl1Spec>;
#[doc = "Field `USIIFG` reader - USI Counter Interrupt Flag"]
pub type UsiifgR = crate::BitReader;
#[doc = "Field `USIIFG` writer - USI Counter Interrupt Flag"]
pub type UsiifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISTTIFG` reader - USI START Condition interrupt Flag"]
pub type UsisttifgR = crate::BitReader;
#[doc = "Field `USISTTIFG` writer - USI START Condition interrupt Flag"]
pub type UsisttifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISTP` reader - USI STOP Condition received"]
pub type UsistpR = crate::BitReader;
#[doc = "Field `USISTP` writer - USI STOP Condition received"]
pub type UsistpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIAL` reader - USI Arbitration Lost"]
pub type UsialR = crate::BitReader;
#[doc = "Field `USIAL` writer - USI Arbitration Lost"]
pub type UsialW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIIE` reader - USI Counter Interrupt enable"]
pub type UsiieR = crate::BitReader;
#[doc = "Field `USIIE` writer - USI Counter Interrupt enable"]
pub type UsiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISTTIE` reader - USI START Condition interrupt enable"]
pub type UsisttieR = crate::BitReader;
#[doc = "Field `USISTTIE` writer - USI START Condition interrupt enable"]
pub type UsisttieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USII2C` reader - USI I2C Mode"]
pub type Usii2cR = crate::BitReader;
#[doc = "Field `USII2C` writer - USI I2C Mode"]
pub type Usii2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICKPH` reader - USI Sync. Mode: Clock Phase"]
pub type UsickphR = crate::BitReader;
#[doc = "Field `USICKPH` writer - USI Sync. Mode: Clock Phase"]
pub type UsickphW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    pub fn usiifg(&self) -> UsiifgR {
        UsiifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    pub fn usisttifg(&self) -> UsisttifgR {
        UsisttifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    pub fn usistp(&self) -> UsistpR {
        UsistpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    pub fn usial(&self) -> UsialR {
        UsialR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    pub fn usiie(&self) -> UsiieR {
        UsiieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    pub fn usisttie(&self) -> UsisttieR {
        UsisttieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    pub fn usii2c(&self) -> Usii2cR {
        Usii2cR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn usickph(&self) -> UsickphR {
        UsickphR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Counter Interrupt Flag"]
    #[inline(always)]
    pub fn usiifg(&mut self) -> UsiifgW<Usictl1Spec> {
        UsiifgW::new(self, 0)
    }
    #[doc = "Bit 1 - USI START Condition interrupt Flag"]
    #[inline(always)]
    pub fn usisttifg(&mut self) -> UsisttifgW<Usictl1Spec> {
        UsisttifgW::new(self, 1)
    }
    #[doc = "Bit 2 - USI STOP Condition received"]
    #[inline(always)]
    pub fn usistp(&mut self) -> UsistpW<Usictl1Spec> {
        UsistpW::new(self, 2)
    }
    #[doc = "Bit 3 - USI Arbitration Lost"]
    #[inline(always)]
    pub fn usial(&mut self) -> UsialW<Usictl1Spec> {
        UsialW::new(self, 3)
    }
    #[doc = "Bit 4 - USI Counter Interrupt enable"]
    #[inline(always)]
    pub fn usiie(&mut self) -> UsiieW<Usictl1Spec> {
        UsiieW::new(self, 4)
    }
    #[doc = "Bit 5 - USI START Condition interrupt enable"]
    #[inline(always)]
    pub fn usisttie(&mut self) -> UsisttieW<Usictl1Spec> {
        UsisttieW::new(self, 5)
    }
    #[doc = "Bit 6 - USI I2C Mode"]
    #[inline(always)]
    pub fn usii2c(&mut self) -> Usii2cW<Usictl1Spec> {
        Usii2cW::new(self, 6)
    }
    #[doc = "Bit 7 - USI Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn usickph(&mut self) -> UsickphW<Usictl1Spec> {
        UsickphW::new(self, 7)
    }
}
#[doc = "USI Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`usictl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usictl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usictl1Spec;
impl crate::RegisterSpec for Usictl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usictl1::R`](R) reader structure"]
impl crate::Readable for Usictl1Spec {}
#[doc = "`write(|w| ..)` method takes [`usictl1::W`](W) writer structure"]
impl crate::Writable for Usictl1Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICTL1 to value 0"]
impl crate::Resettable for Usictl1Spec {
    const RESET_VALUE: u8 = 0;
}
