#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WDTCTL_SPEC>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WDTCTL_SPEC>;
#[doc = "Field `WDTIS` reader - WDTIS0"]
pub type WDTIS_R = crate::FieldReader;
#[doc = "Field `WDTIS` writer - WDTIS0"]
pub type WDTIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub type WDTSSEL_R = crate::BitReader;
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub type WDTSSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub type WDTCNTCL_R = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub type WDTCNTCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub type WDTTMSEL_R = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub type WDTTMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub type WDTNMI_R = crate::BitReader;
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub type WDTNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub type WDTNMIES_R = crate::BitReader;
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub type WDTNMIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub type WDTHOLD_R = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub type WDTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWR_A {
    #[doc = "105: Value always read from the Watchdog Password register"]
    PASSWORD = 105,
}
impl From<WDTPWR_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTPWR_A {
    type Ux = u8;
}
impl crate::IsEnum for WDTPWR_A {}
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WDTPW_R = crate::FieldReader<WDTPWR_A>;
impl WDTPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDTPWR_A> {
        match self.bits {
            105 => Some(WDTPWR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Value always read from the Watchdog Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == WDTPWR_A::PASSWORD
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWW_AW {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    PASSWORD = 90,
}
impl From<WDTPWW_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTPWW_AW {
    type Ux = u8;
}
impl crate::IsEnum for WDTPWW_AW {}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WDTPW_W<'a, REG> = crate::FieldWriter<'a, REG, 8, WDTPWW_AW>;
impl<'a, REG> WDTPW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(WDTPWW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WDTNMI_R {
        WDTNMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WDTNMIES_R {
        WDTNMIES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W<WDTCTL_SPEC> {
        WDTIS_W::new(self, 0)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<WDTCTL_SPEC> {
        WDTSSEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<WDTCTL_SPEC> {
        WDTCNTCL_W::new(self, 3)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<WDTCTL_SPEC> {
        WDTTMSEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WDTNMI_W<WDTCTL_SPEC> {
        WDTNMI_W::new(self, 5)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WDTNMIES_W<WDTCTL_SPEC> {
        WDTNMIES_W::new(self, 6)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W<WDTCTL_SPEC> {
        WDTHOLD_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W<WDTCTL_SPEC> {
        WDTPW_W::new(self, 8)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    const RESET_VALUE: u16 = 0;
}
