#[doc = "Register `P4REN` reader"]
pub type R = crate::R<P4renSpec>;
#[doc = "Register `P4REN` writer"]
pub type W = crate::W<P4renSpec>;
#[doc = "Field `P0` reader - P0"]
pub type P0R = crate::BitReader;
#[doc = "Field `P0` writer - P0"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` reader - P1"]
pub type P1R = crate::BitReader;
#[doc = "Field `P1` writer - P1"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` reader - P2"]
pub type P2R = crate::BitReader;
#[doc = "Field `P2` writer - P2"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` reader - P3"]
pub type P3R = crate::BitReader;
#[doc = "Field `P3` writer - P3"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` reader - P4"]
pub type P4R = crate::BitReader;
#[doc = "Field `P4` writer - P4"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` reader - P5"]
pub type P5R = crate::BitReader;
#[doc = "Field `P5` writer - P5"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` reader - P6"]
pub type P6R = crate::BitReader;
#[doc = "Field `P6` writer - P6"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` reader - P7"]
pub type P7R = crate::BitReader;
#[doc = "Field `P7` writer - P7"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&self) -> P2R {
        P2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&self) -> P3R {
        P3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&self) -> P4R {
        P4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&self) -> P5R {
        P5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&self) -> P6R {
        P6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&self) -> P7R {
        P7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0W<P4renSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1W<P4renSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2W<P4renSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3W<P4renSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4W<P4renSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5W<P4renSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6W<P4renSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7W<P4renSpec> {
        P7W::new(self, 7)
    }
}
#[doc = "Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4renSpec;
impl crate::RegisterSpec for P4renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ren::R`](R) reader structure"]
impl crate::Readable for P4renSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ren::W`](W) writer structure"]
impl crate::Writable for P4renSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets P4REN to value 0"]
impl crate::Resettable for P4renSpec {
    const RESET_VALUE: u8 = 0;
}