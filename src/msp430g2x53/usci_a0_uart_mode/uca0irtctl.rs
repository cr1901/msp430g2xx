#[doc = "Register `UCA0IRTCTL` reader"]
pub type R = crate::R<Uca0irtctlSpec>;
#[doc = "Register `UCA0IRTCTL` writer"]
pub type W = crate::W<Uca0irtctlSpec>;
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub type UcirenR = crate::BitReader;
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub type UcirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub type UcirtxclkR = crate::BitReader;
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub type UcirtxclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL` reader - IRDA Transmit Pulse Length 0"]
pub type UcirtxplR = crate::FieldReader;
#[doc = "Field `UCIRTXPL` writer - IRDA Transmit Pulse Length 0"]
pub type UcirtxplW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UcirenR {
        UcirenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UcirtxclkR {
        UcirtxclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UcirtxplR {
        UcirtxplR::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UcirenW<Uca0irtctlSpec> {
        UcirenW::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UcirtxclkW<Uca0irtctlSpec> {
        UcirtxclkW::new(self, 1)
    }
    #[doc = "Bits 2:7 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UcirtxplW<Uca0irtctlSpec> {
        UcirtxplW::new(self, 2)
    }
}
#[doc = "USCI A0 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0irtctlSpec;
impl crate::RegisterSpec for Uca0irtctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0irtctl::R`](R) reader structure"]
impl crate::Readable for Uca0irtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0irtctl::W`](W) writer structure"]
impl crate::Writable for Uca0irtctlSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCA0IRTCTL to value 0"]
impl crate::Resettable for Uca0irtctlSpec {
    const RESET_VALUE: u8 = 0;
}
