#[doc = "Register `USICTL0` reader"]
pub type R = crate::R<USICTL0_SPEC>;
#[doc = "Register `USICTL0` writer"]
pub type W = crate::W<USICTL0_SPEC>;
#[doc = "Field `USISWRST` reader - USI Software Reset"]
pub type USISWRST_R = crate::BitReader;
#[doc = "Field `USISWRST` writer - USI Software Reset"]
pub type USISWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIOE` reader - USI Output Enable"]
pub type USIOE_R = crate::BitReader;
#[doc = "Field `USIOE` writer - USI Output Enable"]
pub type USIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIGE` reader - USI General Output Enable Latch"]
pub type USIGE_R = crate::BitReader;
#[doc = "Field `USIGE` writer - USI General Output Enable Latch"]
pub type USIGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIMST` reader - USI Master Select 0:Slave / 1:Master"]
pub type USIMST_R = crate::BitReader;
#[doc = "Field `USIMST` writer - USI Master Select 0:Slave / 1:Master"]
pub type USIMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USILSB` reader - USI LSB first 1:LSB / 0:MSB"]
pub type USILSB_R = crate::BitReader;
#[doc = "Field `USILSB` writer - USI LSB first 1:LSB / 0:MSB"]
pub type USILSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE5` reader - USI Port Enable Px.5"]
pub type USIPE5_R = crate::BitReader;
#[doc = "Field `USIPE5` writer - USI Port Enable Px.5"]
pub type USIPE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE6` reader - USI Port Enable Px.6"]
pub type USIPE6_R = crate::BitReader;
#[doc = "Field `USIPE6` writer - USI Port Enable Px.6"]
pub type USIPE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE7` reader - USI Port Enable Px.7"]
pub type USIPE7_R = crate::BitReader;
#[doc = "Field `USIPE7` writer - USI Port Enable Px.7"]
pub type USIPE7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&self) -> USISWRST_R {
        USISWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&self) -> USIOE_R {
        USIOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&self) -> USIGE_R {
        USIGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&self) -> USIMST_R {
        USIMST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&self) -> USILSB_R {
        USILSB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&self) -> USIPE5_R {
        USIPE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&self) -> USIPE6_R {
        USIPE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&self) -> USIPE7_R {
        USIPE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&mut self) -> USISWRST_W<USICTL0_SPEC> {
        USISWRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&mut self) -> USIOE_W<USICTL0_SPEC> {
        USIOE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&mut self) -> USIGE_W<USICTL0_SPEC> {
        USIGE_W::new(self, 2)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&mut self) -> USIMST_W<USICTL0_SPEC> {
        USIMST_W::new(self, 3)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&mut self) -> USILSB_W<USICTL0_SPEC> {
        USILSB_W::new(self, 4)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&mut self) -> USIPE5_W<USICTL0_SPEC> {
        USIPE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&mut self) -> USIPE6_W<USICTL0_SPEC> {
        USIPE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&mut self) -> USIPE7_W<USICTL0_SPEC> {
        USIPE7_W::new(self, 7)
    }
}
#[doc = "USI Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usictl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usictl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USICTL0_SPEC;
impl crate::RegisterSpec for USICTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usictl0::R`](R) reader structure"]
impl crate::Readable for USICTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usictl0::W`](W) writer structure"]
impl crate::Writable for USICTL0_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICTL0 to value 0"]
impl crate::Resettable for USICTL0_SPEC {
    const RESET_VALUE: u8 = 0;
}
