#[doc = "Register `DCOCTL` reader"]
pub type R = crate::R<DCOCTL_SPEC>;
#[doc = "Register `DCOCTL` writer"]
pub type W = crate::W<DCOCTL_SPEC>;
#[doc = "Field `DCOCTL` reader - DCO Clock Frequency Control register"]
pub type DCOCTL_R = crate::FieldReader;
#[doc = "Field `DCOCTL` writer - DCO Clock Frequency Control register"]
pub type DCOCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `MOD` reader - Modulation Bit 0"]
pub type MOD_R = crate::FieldReader;
#[doc = "Field `MOD` writer - Modulation Bit 0"]
pub type MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `DCO` reader - DCO Select Bit 0"]
pub type DCO_R = crate::FieldReader;
#[doc = "Field `DCO` writer - DCO Select Bit 0"]
pub type DCO_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&self) -> DCOCTL_R {
        DCOCTL_R::new(self.bits)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&self) -> DCO_R {
        DCO_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&mut self) -> DCOCTL_W<DCOCTL_SPEC> {
        DCOCTL_W::new(self, 0)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W<DCOCTL_SPEC> {
        MOD_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&mut self) -> DCO_W<DCOCTL_SPEC> {
        DCO_W::new(self, 5)
    }
}
#[doc = "DCO Clock Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCOCTL_SPEC;
impl crate::RegisterSpec for DCOCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dcoctl::R`](R) reader structure"]
impl crate::Readable for DCOCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcoctl::W`](W) writer structure"]
impl crate::Writable for DCOCTL_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DCOCTL to value 0"]
impl crate::Resettable for DCOCTL_SPEC {
    const RESET_VALUE: u8 = 0;
}
