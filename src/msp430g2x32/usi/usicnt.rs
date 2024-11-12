#[doc = "Register `USICNT` reader"]
pub type R = crate::R<UsicntSpec>;
#[doc = "Register `USICNT` writer"]
pub type W = crate::W<UsicntSpec>;
#[doc = "Field `USICNT0` reader - USI Bit Count 0"]
pub type Usicnt0R = crate::BitReader;
#[doc = "Field `USICNT0` writer - USI Bit Count 0"]
pub type Usicnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT1` reader - USI Bit Count 1"]
pub type Usicnt1R = crate::BitReader;
#[doc = "Field `USICNT1` writer - USI Bit Count 1"]
pub type Usicnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT2` reader - USI Bit Count 2"]
pub type Usicnt2R = crate::BitReader;
#[doc = "Field `USICNT2` writer - USI Bit Count 2"]
pub type Usicnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT3` reader - USI Bit Count 3"]
pub type Usicnt3R = crate::BitReader;
#[doc = "Field `USICNT3` writer - USI Bit Count 3"]
pub type Usicnt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT4` reader - USI Bit Count 4"]
pub type Usicnt4R = crate::BitReader;
#[doc = "Field `USICNT4` writer - USI Bit Count 4"]
pub type Usicnt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIIFGCC` reader - USI Interrupt Flag Clear Control"]
pub type UsiifgccR = crate::BitReader;
#[doc = "Field `USIIFGCC` writer - USI Interrupt Flag Clear Control"]
pub type UsiifgccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USI16B` reader - USI 16 Bit Shift Register Enable"]
pub type Usi16bR = crate::BitReader;
#[doc = "Field `USI16B` writer - USI 16 Bit Shift Register Enable"]
pub type Usi16bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISCLREL` reader - USI SCL Released"]
pub type UsisclrelR = crate::BitReader;
#[doc = "Field `USISCLREL` writer - USI SCL Released"]
pub type UsisclrelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&self) -> Usicnt0R {
        Usicnt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&self) -> Usicnt1R {
        Usicnt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&self) -> Usicnt2R {
        Usicnt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&self) -> Usicnt3R {
        Usicnt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&self) -> Usicnt4R {
        Usicnt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&self) -> UsiifgccR {
        UsiifgccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&self) -> Usi16bR {
        Usi16bR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&self) -> UsisclrelR {
        UsisclrelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&mut self) -> Usicnt0W<UsicntSpec> {
        Usicnt0W::new(self, 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&mut self) -> Usicnt1W<UsicntSpec> {
        Usicnt1W::new(self, 1)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&mut self) -> Usicnt2W<UsicntSpec> {
        Usicnt2W::new(self, 2)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&mut self) -> Usicnt3W<UsicntSpec> {
        Usicnt3W::new(self, 3)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&mut self) -> Usicnt4W<UsicntSpec> {
        Usicnt4W::new(self, 4)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&mut self) -> UsiifgccW<UsicntSpec> {
        UsiifgccW::new(self, 5)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&mut self) -> Usi16bW<UsicntSpec> {
        Usi16bW::new(self, 6)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&mut self) -> UsisclrelW<UsicntSpec> {
        UsisclrelW::new(self, 7)
    }
}
#[doc = "USI Bit Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usicnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usicnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsicntSpec;
impl crate::RegisterSpec for UsicntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usicnt::R`](R) reader structure"]
impl crate::Readable for UsicntSpec {}
#[doc = "`write(|w| ..)` method takes [`usicnt::W`](W) writer structure"]
impl crate::Writable for UsicntSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICNT to value 0"]
impl crate::Resettable for UsicntSpec {
    const RESET_VALUE: u8 = 0;
}
