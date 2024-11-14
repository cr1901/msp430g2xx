#[doc = "Register `CAPD` reader"]
pub type R = crate::R<CAPD_SPEC>;
#[doc = "Register `CAPD` writer"]
pub type W = crate::W<CAPD_SPEC>;
#[doc = "Field `CAPD0` reader - Comp. A Disable Input Buffer of Port Register .0"]
pub type CAPD0_R = crate::BitReader;
#[doc = "Field `CAPD0` writer - Comp. A Disable Input Buffer of Port Register .0"]
pub type CAPD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD` reader - Comparator A Port Disable register"]
pub type CAPD_R = crate::FieldReader;
#[doc = "Field `CAPD` writer - Comparator A Port Disable register"]
pub type CAPD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `CAPD1` reader - Comp. A Disable Input Buffer of Port Register .1"]
pub type CAPD1_R = crate::BitReader;
#[doc = "Field `CAPD1` writer - Comp. A Disable Input Buffer of Port Register .1"]
pub type CAPD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD2` reader - Comp. A Disable Input Buffer of Port Register .2"]
pub type CAPD2_R = crate::BitReader;
#[doc = "Field `CAPD2` writer - Comp. A Disable Input Buffer of Port Register .2"]
pub type CAPD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD3` reader - Comp. A Disable Input Buffer of Port Register .3"]
pub type CAPD3_R = crate::BitReader;
#[doc = "Field `CAPD3` writer - Comp. A Disable Input Buffer of Port Register .3"]
pub type CAPD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD4` reader - Comp. A Disable Input Buffer of Port Register .4"]
pub type CAPD4_R = crate::BitReader;
#[doc = "Field `CAPD4` writer - Comp. A Disable Input Buffer of Port Register .4"]
pub type CAPD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD5` reader - Comp. A Disable Input Buffer of Port Register .5"]
pub type CAPD5_R = crate::BitReader;
#[doc = "Field `CAPD5` writer - Comp. A Disable Input Buffer of Port Register .5"]
pub type CAPD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD6` reader - Comp. A Disable Input Buffer of Port Register .6"]
pub type CAPD6_R = crate::BitReader;
#[doc = "Field `CAPD6` writer - Comp. A Disable Input Buffer of Port Register .6"]
pub type CAPD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD7` reader - Comp. A Disable Input Buffer of Port Register .7"]
pub type CAPD7_R = crate::BitReader;
#[doc = "Field `CAPD7` writer - Comp. A Disable Input Buffer of Port Register .7"]
pub type CAPD7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&self) -> CAPD0_R {
        CAPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&self) -> CAPD_R {
        CAPD_R::new(self.bits)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&self) -> CAPD1_R {
        CAPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&self) -> CAPD2_R {
        CAPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&self) -> CAPD3_R {
        CAPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&self) -> CAPD4_R {
        CAPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&self) -> CAPD5_R {
        CAPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&self) -> CAPD6_R {
        CAPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&self) -> CAPD7_R {
        CAPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&mut self) -> CAPD0_W<CAPD_SPEC> {
        CAPD0_W::new(self, 0)
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&mut self) -> CAPD_W<CAPD_SPEC> {
        CAPD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&mut self) -> CAPD1_W<CAPD_SPEC> {
        CAPD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&mut self) -> CAPD2_W<CAPD_SPEC> {
        CAPD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&mut self) -> CAPD3_W<CAPD_SPEC> {
        CAPD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&mut self) -> CAPD4_W<CAPD_SPEC> {
        CAPD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&mut self) -> CAPD5_W<CAPD_SPEC> {
        CAPD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&mut self) -> CAPD6_W<CAPD_SPEC> {
        CAPD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&mut self) -> CAPD7_W<CAPD_SPEC> {
        CAPD7_W::new(self, 7)
    }
}
#[doc = "Comparator A Port Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`capd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPD_SPEC;
impl crate::RegisterSpec for CAPD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`capd::R`](R) reader structure"]
impl crate::Readable for CAPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capd::W`](W) writer structure"]
impl crate::Writable for CAPD_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CAPD to value 0"]
impl crate::Resettable for CAPD_SPEC {
    const RESET_VALUE: u8 = 0;
}
