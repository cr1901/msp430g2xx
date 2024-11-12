#[doc = "Register `DCOCTL` reader"]
pub type R = crate::R<DcoctlSpec>;
#[doc = "Register `DCOCTL` writer"]
pub type W = crate::W<DcoctlSpec>;
#[doc = "Field `DCOCTL` reader - DCO Clock Frequency Control register"]
pub type DcoctlR = crate::FieldReader;
#[doc = "Field `DCOCTL` writer - DCO Clock Frequency Control register"]
pub type DcoctlW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `MOD` reader - Modulation Bit 0"]
pub type ModR = crate::FieldReader;
#[doc = "Field `MOD` writer - Modulation Bit 0"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `DCO` reader - DCO Select Bit 0"]
pub type DcoR = crate::FieldReader;
#[doc = "Field `DCO` writer - DCO Select Bit 0"]
pub type DcoW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&self) -> DcoctlR {
        DcoctlR::new(self.bits)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&self) -> DcoR {
        DcoR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&mut self) -> DcoctlW<DcoctlSpec> {
        DcoctlW::new(self, 0)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<DcoctlSpec> {
        ModW::new(self, 0)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&mut self) -> DcoW<DcoctlSpec> {
        DcoW::new(self, 5)
    }
}
#[doc = "DCO Clock Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoctlSpec;
impl crate::RegisterSpec for DcoctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dcoctl::R`](R) reader structure"]
impl crate::Readable for DcoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcoctl::W`](W) writer structure"]
impl crate::Writable for DcoctlSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DCOCTL to value 0"]
impl crate::Resettable for DcoctlSpec {
    const RESET_VALUE: u8 = 0;
}
