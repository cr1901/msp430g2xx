#[doc = "Register `TAIV` reader"]
pub type R = crate::R<TaivSpec>;
#[doc = "Register `TAIV` writer"]
pub type W = crate::W<TaivSpec>;
#[doc = "Timer A Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Taiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Capture/Compare 1"]
    Taccr1 = 2,
    #[doc = "10: Timer overflow"]
    Taifg = 10,
}
impl From<Taiv> for u8 {
    #[inline(always)]
    fn from(variant: Taiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taiv {
    type Ux = u8;
}
impl crate::IsEnum for Taiv {}
#[doc = "Field `TAIV` reader - Timer A Interrupt Vector value"]
pub type TaivR = crate::FieldReader<Taiv>;
impl TaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Taiv> {
        match self.bits {
            0 => Some(Taiv::None),
            2 => Some(Taiv::Taccr1),
            10 => Some(Taiv::Taifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Taiv::None
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == Taiv::Taccr1
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == Taiv::Taifg
    }
}
#[doc = "Field `TAIV` writer - Timer A Interrupt Vector value"]
pub type TaivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Taiv>;
impl<'a, REG> TaivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::None)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr1)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taifg)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TaivR {
        TaivR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&mut self) -> TaivW<TaivSpec> {
        TaivW::new(self, 0)
    }
}
#[doc = "Timer A Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`taiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`taiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaivSpec;
impl crate::RegisterSpec for TaivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`taiv::R`](R) reader structure"]
impl crate::Readable for TaivSpec {}
#[doc = "`write(|w| ..)` method takes [`taiv::W`](W) writer structure"]
impl crate::Writable for TaivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAIV to value 0"]
impl crate::Resettable for TaivSpec {
    const RESET_VALUE: u16 = 0;
}
