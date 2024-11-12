#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "Field `WDTIS` reader - WDTIS0"]
pub type WdtisR = crate::FieldReader;
#[doc = "Field `WDTIS` writer - WDTIS0"]
pub type WdtisW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub type WdtsselR = crate::BitReader;
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub type WdtsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub type WdtcntclR = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub type WdttmselR = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub type WdtnmiR = crate::BitReader;
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub type WdtnmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub type WdtnmiesR = crate::BitReader;
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub type WdtnmiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub type WdtholdR = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtpwr {
    #[doc = "105: Value always read from the Watchdog Password register"]
    Password = 105,
}
impl From<Wdtpwr> for u8 {
    #[inline(always)]
    fn from(variant: Wdtpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtpwr {
    type Ux = u8;
}
impl crate::IsEnum for Wdtpwr {}
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WdtpwR = crate::FieldReader<Wdtpwr>;
impl WdtpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdtpwr> {
        match self.bits {
            105 => Some(Wdtpwr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the Watchdog Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Wdtpwr::Password
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtpwwWO {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    Password = 90,
}
impl From<WdtpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: WdtpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for WdtpwwWO {}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WdtpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, WdtpwwWO>;
impl<'a, REG> WdtpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(WdtpwwWO::Password)
    }
}
impl R {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WdtisR {
        WdtisR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WdtsselR {
        WdtsselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WdtcntclR {
        WdtcntclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WdttmselR {
        WdttmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WdtnmiR {
        WdtnmiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WdtnmiesR {
        WdtnmiesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WdtholdR {
        WdtholdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WdtpwR {
        WdtpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WdtisW<WdtctlSpec> {
        WdtisW::new(self, 0)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WdtsselW<WdtctlSpec> {
        WdtsselW::new(self, 2)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WdtcntclW<WdtctlSpec> {
        WdtcntclW::new(self, 3)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WdttmselW<WdtctlSpec> {
        WdttmselW::new(self, 4)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WdtnmiW<WdtctlSpec> {
        WdtnmiW::new(self, 5)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WdtnmiesW<WdtctlSpec> {
        WdtnmiesW::new(self, 6)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WdtholdW<WdtctlSpec> {
        WdtholdW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WdtpwW<WdtctlSpec> {
        WdtpwW::new(self, 8)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtctlSpec;
impl crate::RegisterSpec for WdtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WdtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WdtctlSpec {
    const RESET_VALUE: u16 = 0;
}
