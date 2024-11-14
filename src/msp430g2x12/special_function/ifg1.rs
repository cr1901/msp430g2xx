#[doc = "Register `IFG1` reader"]
pub type R = crate::R<IFG1_SPEC>;
#[doc = "Register `IFG1` writer"]
pub type W = crate::W<IFG1_SPEC>;
#[doc = "Field `WDTIFG` reader - Watchdog Interrupt Flag"]
pub type WDTIFG_R = crate::BitReader;
#[doc = "Field `WDTIFG` writer - Watchdog Interrupt Flag"]
pub type WDTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIFG` reader - Osc. Fault Interrupt Flag"]
pub type OFIFG_R = crate::BitReader;
#[doc = "Field `OFIFG` writer - Osc. Fault Interrupt Flag"]
pub type OFIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORIFG` reader - Power On Interrupt Flag"]
pub type PORIFG_R = crate::BitReader;
#[doc = "Field `PORIFG` writer - Power On Interrupt Flag"]
pub type PORIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIFG` reader - Reset Interrupt Flag"]
pub type RSTIFG_R = crate::BitReader;
#[doc = "Field `RSTIFG` writer - Reset Interrupt Flag"]
pub type RSTIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NMIIFG_R = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NMIIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PORIFG_R {
        PORIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RSTIFG_R {
        RSTIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W<IFG1_SPEC> {
        WDTIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W<IFG1_SPEC> {
        OFIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&mut self) -> PORIFG_W<IFG1_SPEC> {
        PORIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&mut self) -> RSTIFG_W<IFG1_SPEC> {
        RSTIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W<IFG1_SPEC> {
        NMIIFG_W::new(self, 4)
    }
}
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFG1_SPEC;
impl crate::RegisterSpec for IFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg1::R`](R) reader structure"]
impl crate::Readable for IFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifg1::W`](W) writer structure"]
impl crate::Writable for IFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for IFG1_SPEC {
    const RESET_VALUE: u8 = 0;
}
