#[doc = "Register `USICTL0` reader"]
pub type R = crate::R<Usictl0Spec>;
#[doc = "Register `USICTL0` writer"]
pub type W = crate::W<Usictl0Spec>;
#[doc = "Field `USISWRST` reader - USI Software Reset"]
pub type UsiswrstR = crate::BitReader;
#[doc = "Field `USISWRST` writer - USI Software Reset"]
pub type UsiswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIOE` reader - USI Output Enable"]
pub type UsioeR = crate::BitReader;
#[doc = "Field `USIOE` writer - USI Output Enable"]
pub type UsioeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIGE` reader - USI General Output Enable Latch"]
pub type UsigeR = crate::BitReader;
#[doc = "Field `USIGE` writer - USI General Output Enable Latch"]
pub type UsigeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIMST` reader - USI Master Select 0:Slave / 1:Master"]
pub type UsimstR = crate::BitReader;
#[doc = "Field `USIMST` writer - USI Master Select 0:Slave / 1:Master"]
pub type UsimstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USILSB` reader - USI LSB first 1:LSB / 0:MSB"]
pub type UsilsbR = crate::BitReader;
#[doc = "Field `USILSB` writer - USI LSB first 1:LSB / 0:MSB"]
pub type UsilsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE5` reader - USI Port Enable Px.5"]
pub type Usipe5R = crate::BitReader;
#[doc = "Field `USIPE5` writer - USI Port Enable Px.5"]
pub type Usipe5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE6` reader - USI Port Enable Px.6"]
pub type Usipe6R = crate::BitReader;
#[doc = "Field `USIPE6` writer - USI Port Enable Px.6"]
pub type Usipe6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIPE7` reader - USI Port Enable Px.7"]
pub type Usipe7R = crate::BitReader;
#[doc = "Field `USIPE7` writer - USI Port Enable Px.7"]
pub type Usipe7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&self) -> UsiswrstR {
        UsiswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&self) -> UsioeR {
        UsioeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&self) -> UsigeR {
        UsigeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&self) -> UsimstR {
        UsimstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&self) -> UsilsbR {
        UsilsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&self) -> Usipe5R {
        Usipe5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&self) -> Usipe6R {
        Usipe6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&self) -> Usipe7R {
        Usipe7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Reset"]
    #[inline(always)]
    pub fn usiswrst(&mut self) -> UsiswrstW<Usictl0Spec> {
        UsiswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - USI Output Enable"]
    #[inline(always)]
    pub fn usioe(&mut self) -> UsioeW<Usictl0Spec> {
        UsioeW::new(self, 1)
    }
    #[doc = "Bit 2 - USI General Output Enable Latch"]
    #[inline(always)]
    pub fn usige(&mut self) -> UsigeW<Usictl0Spec> {
        UsigeW::new(self, 2)
    }
    #[doc = "Bit 3 - USI Master Select 0:Slave / 1:Master"]
    #[inline(always)]
    pub fn usimst(&mut self) -> UsimstW<Usictl0Spec> {
        UsimstW::new(self, 3)
    }
    #[doc = "Bit 4 - USI LSB first 1:LSB / 0:MSB"]
    #[inline(always)]
    pub fn usilsb(&mut self) -> UsilsbW<Usictl0Spec> {
        UsilsbW::new(self, 4)
    }
    #[doc = "Bit 5 - USI Port Enable Px.5"]
    #[inline(always)]
    pub fn usipe5(&mut self) -> Usipe5W<Usictl0Spec> {
        Usipe5W::new(self, 5)
    }
    #[doc = "Bit 6 - USI Port Enable Px.6"]
    #[inline(always)]
    pub fn usipe6(&mut self) -> Usipe6W<Usictl0Spec> {
        Usipe6W::new(self, 6)
    }
    #[doc = "Bit 7 - USI Port Enable Px.7"]
    #[inline(always)]
    pub fn usipe7(&mut self) -> Usipe7W<Usictl0Spec> {
        Usipe7W::new(self, 7)
    }
}
#[doc = "USI Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usictl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usictl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usictl0Spec;
impl crate::RegisterSpec for Usictl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usictl0::R`](R) reader structure"]
impl crate::Readable for Usictl0Spec {}
#[doc = "`write(|w| ..)` method takes [`usictl0::W`](W) writer structure"]
impl crate::Writable for Usictl0Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICTL0 to value 0"]
impl crate::Resettable for Usictl0Spec {
    const RESET_VALUE: u8 = 0;
}
