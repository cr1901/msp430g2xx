#[doc = "Register `UCA0IRRCTL` reader"]
pub type R = crate::R<UCA0IRRCTL_SPEC>;
#[doc = "Register `UCA0IRRCTL` writer"]
pub type W = crate::W<UCA0IRRCTL_SPEC>;
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub type UCIRRXFE_R = crate::BitReader;
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub type UCIRRXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub type UCIRRXPL_R = crate::BitReader;
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub type UCIRRXPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL` reader - IRDA Receive Filter Length 0"]
pub type UCIRRXFL_R = crate::FieldReader;
#[doc = "Field `UCIRRXFL` writer - IRDA Receive Filter Length 0"]
pub type UCIRRXFL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UCIRRXFL_R {
        UCIRRXFL_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W<UCA0IRRCTL_SPEC> {
        UCIRRXFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W<UCA0IRRCTL_SPEC> {
        UCIRRXPL_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UCIRRXFL_W<UCA0IRRCTL_SPEC> {
        UCIRRXFL_W::new(self, 2)
    }
}
#[doc = "USCI A0 IrDA Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0IRRCTL_SPEC;
impl crate::RegisterSpec for UCA0IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0irrctl::R`](R) reader structure"]
impl crate::Readable for UCA0IRRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0irrctl::W`](W) writer structure"]
impl crate::Writable for UCA0IRRCTL_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0IRRCTL to value 0"]
impl crate::Resettable for UCA0IRRCTL_SPEC {
    const RESET_VALUE: u8 = 0;
}
