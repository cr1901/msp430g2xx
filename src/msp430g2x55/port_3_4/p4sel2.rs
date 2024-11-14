#[doc = "Register `P4SEL2` reader"]
pub type R = crate::R<P4SEL2_SPEC>;
#[doc = "Register `P4SEL2` writer"]
pub type W = crate::W<P4SEL2_SPEC>;
#[doc = "Field `P0` reader - P0"]
pub type P0_R = crate::BitReader;
#[doc = "Field `P0` writer - P0"]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` reader - P1"]
pub type P1_R = crate::BitReader;
#[doc = "Field `P1` writer - P1"]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` reader - P2"]
pub type P2_R = crate::BitReader;
#[doc = "Field `P2` writer - P2"]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` reader - P3"]
pub type P3_R = crate::BitReader;
#[doc = "Field `P3` writer - P3"]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` reader - P4"]
pub type P4_R = crate::BitReader;
#[doc = "Field `P4` writer - P4"]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` reader - P5"]
pub type P5_R = crate::BitReader;
#[doc = "Field `P5` writer - P5"]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` reader - P6"]
pub type P6_R = crate::BitReader;
#[doc = "Field `P6` writer - P6"]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` reader - P7"]
pub type P7_R = crate::BitReader;
#[doc = "Field `P7` writer - P7"]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W<P4SEL2_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W<P4SEL2_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W<P4SEL2_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W<P4SEL2_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W<P4SEL2_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W<P4SEL2_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W<P4SEL2_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W<P4SEL2_SPEC> {
        P7_W::new(self, 7)
    }
}
#[doc = "Port 4 Selection 2\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4SEL2_SPEC;
impl crate::RegisterSpec for P4SEL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4sel2::R`](R) reader structure"]
impl crate::Readable for P4SEL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p4sel2::W`](W) writer structure"]
impl crate::Writable for P4SEL2_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets P4SEL2 to value 0"]
impl crate::Resettable for P4SEL2_SPEC {
    const RESET_VALUE: u8 = 0;
}
