#[doc = "Register `USICNT` reader"]
pub type R = crate::R<USICNT_SPEC>;
#[doc = "Register `USICNT` writer"]
pub type W = crate::W<USICNT_SPEC>;
#[doc = "Field `USICNT0` reader - USI Bit Count 0"]
pub type USICNT0_R = crate::BitReader;
#[doc = "Field `USICNT0` writer - USI Bit Count 0"]
pub type USICNT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT1` reader - USI Bit Count 1"]
pub type USICNT1_R = crate::BitReader;
#[doc = "Field `USICNT1` writer - USI Bit Count 1"]
pub type USICNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT2` reader - USI Bit Count 2"]
pub type USICNT2_R = crate::BitReader;
#[doc = "Field `USICNT2` writer - USI Bit Count 2"]
pub type USICNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT3` reader - USI Bit Count 3"]
pub type USICNT3_R = crate::BitReader;
#[doc = "Field `USICNT3` writer - USI Bit Count 3"]
pub type USICNT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICNT4` reader - USI Bit Count 4"]
pub type USICNT4_R = crate::BitReader;
#[doc = "Field `USICNT4` writer - USI Bit Count 4"]
pub type USICNT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIIFGCC` reader - USI Interrupt Flag Clear Control"]
pub type USIIFGCC_R = crate::BitReader;
#[doc = "Field `USIIFGCC` writer - USI Interrupt Flag Clear Control"]
pub type USIIFGCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USI16B` reader - USI 16 Bit Shift Register Enable"]
pub type USI16B_R = crate::BitReader;
#[doc = "Field `USI16B` writer - USI 16 Bit Shift Register Enable"]
pub type USI16B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISCLREL` reader - USI SCL Released"]
pub type USISCLREL_R = crate::BitReader;
#[doc = "Field `USISCLREL` writer - USI SCL Released"]
pub type USISCLREL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&self) -> USICNT0_R {
        USICNT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&self) -> USICNT1_R {
        USICNT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&self) -> USICNT2_R {
        USICNT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&self) -> USICNT3_R {
        USICNT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&self) -> USICNT4_R {
        USICNT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&self) -> USIIFGCC_R {
        USIIFGCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&self) -> USI16B_R {
        USI16B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&self) -> USISCLREL_R {
        USISCLREL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Bit Count 0"]
    #[inline(always)]
    pub fn usicnt0(&mut self) -> USICNT0_W<USICNT_SPEC> {
        USICNT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - USI Bit Count 1"]
    #[inline(always)]
    pub fn usicnt1(&mut self) -> USICNT1_W<USICNT_SPEC> {
        USICNT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - USI Bit Count 2"]
    #[inline(always)]
    pub fn usicnt2(&mut self) -> USICNT2_W<USICNT_SPEC> {
        USICNT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - USI Bit Count 3"]
    #[inline(always)]
    pub fn usicnt3(&mut self) -> USICNT3_W<USICNT_SPEC> {
        USICNT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - USI Bit Count 4"]
    #[inline(always)]
    pub fn usicnt4(&mut self) -> USICNT4_W<USICNT_SPEC> {
        USICNT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - USI Interrupt Flag Clear Control"]
    #[inline(always)]
    pub fn usiifgcc(&mut self) -> USIIFGCC_W<USICNT_SPEC> {
        USIIFGCC_W::new(self, 5)
    }
    #[doc = "Bit 6 - USI 16 Bit Shift Register Enable"]
    #[inline(always)]
    pub fn usi16b(&mut self) -> USI16B_W<USICNT_SPEC> {
        USI16B_W::new(self, 6)
    }
    #[doc = "Bit 7 - USI SCL Released"]
    #[inline(always)]
    pub fn usisclrel(&mut self) -> USISCLREL_W<USICNT_SPEC> {
        USISCLREL_W::new(self, 7)
    }
}
#[doc = "USI Bit Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usicnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usicnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USICNT_SPEC;
impl crate::RegisterSpec for USICNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usicnt::R`](R) reader structure"]
impl crate::Readable for USICNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usicnt::W`](W) writer structure"]
impl crate::Writable for USICNT_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICNT to value 0"]
impl crate::Resettable for USICNT_SPEC {
    const RESET_VALUE: u8 = 0;
}
