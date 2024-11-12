#[doc = "Register `TBIV` reader"]
pub type R = crate::R<TbivSpec>;
#[doc = "Register `TBIV` writer"]
pub type W = crate::W<TbivSpec>;
#[doc = "Timer B Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Capture/Compare 1"]
    Tbccr1 = 2,
    #[doc = "4: Capture/Compare 2"]
    Tbccr2 = 4,
    #[doc = "14: Timer overflow"]
    Tbifg = 14,
}
impl From<Tbiv> for u8 {
    #[inline(always)]
    fn from(variant: Tbiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbiv {
    type Ux = u8;
}
impl crate::IsEnum for Tbiv {}
#[doc = "Field `TBIV` reader - Timer B Interrupt Vector value"]
pub type TbivR = crate::FieldReader<Tbiv>;
impl TbivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tbiv> {
        match self.bits {
            0 => Some(Tbiv::None),
            2 => Some(Tbiv::Tbccr1),
            4 => Some(Tbiv::Tbccr2),
            14 => Some(Tbiv::Tbifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Tbiv::None
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == Tbiv::Tbccr1
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == Tbiv::Tbccr2
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == Tbiv::Tbifg
    }
}
#[doc = "Field `TBIV` writer - Timer B Interrupt Vector value"]
pub type TbivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tbiv>;
impl<'a, REG> TbivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::None)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn tbccr1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr1)
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn tbccr2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr2)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn tbifg(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbifg)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer B Interrupt Vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TbivR {
        TbivR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer B Interrupt Vector value"]
    #[inline(always)]
    pub fn tbiv(&mut self) -> TbivW<TbivSpec> {
        TbivW::new(self, 0)
    }
}
#[doc = "Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbivSpec;
impl crate::RegisterSpec for TbivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbiv::R`](R) reader structure"]
impl crate::Readable for TbivSpec {}
#[doc = "`write(|w| ..)` method takes [`tbiv::W`](W) writer structure"]
impl crate::Writable for TbivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBIV to value 0"]
impl crate::Resettable for TbivSpec {
    const RESET_VALUE: u16 = 0;
}
