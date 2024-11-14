#[doc = "Register `TBIV` reader"]
pub type R = crate::R<TBIV_SPEC>;
#[doc = "Register `TBIV` writer"]
pub type W = crate::W<TBIV_SPEC>;
#[doc = "Timer B Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Capture/Compare 1"]
    TBCCR1 = 2,
    #[doc = "4: Capture/Compare 2"]
    TBCCR2 = 4,
    #[doc = "14: Timer overflow"]
    TBIFG = 14,
}
impl From<TBIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TBIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TBIV_A {
    type Ux = u8;
}
impl crate::IsEnum for TBIV_A {}
#[doc = "Field `TBIV` reader - Timer B Interrupt Vector value"]
pub type TBIV_R = crate::FieldReader<TBIV_A>;
impl TBIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TBIV_A> {
        match self.bits {
            0 => Some(TBIV_A::NONE),
            2 => Some(TBIV_A::TBCCR1),
            4 => Some(TBIV_A::TBCCR2),
            14 => Some(TBIV_A::TBIFG),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TBIV_A::NONE
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == TBIV_A::TBCCR1
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == TBIV_A::TBCCR2
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == TBIV_A::TBIFG
    }
}
#[doc = "Field `TBIV` writer - Timer B Interrupt Vector value"]
pub type TBIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TBIV_A>;
impl<'a, REG> TBIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TBIV_A::NONE)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn tbccr1(self) -> &'a mut crate::W<REG> {
        self.variant(TBIV_A::TBCCR1)
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn tbccr2(self) -> &'a mut crate::W<REG> {
        self.variant(TBIV_A::TBCCR2)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn tbifg(self) -> &'a mut crate::W<REG> {
        self.variant(TBIV_A::TBIFG)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer B Interrupt Vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TBIV_R {
        TBIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer B Interrupt Vector value"]
    #[inline(always)]
    pub fn tbiv(&mut self) -> TBIV_W<TBIV_SPEC> {
        TBIV_W::new(self, 0)
    }
}
#[doc = "Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBIV_SPEC;
impl crate::RegisterSpec for TBIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbiv::R`](R) reader structure"]
impl crate::Readable for TBIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbiv::W`](W) writer structure"]
impl crate::Writable for TBIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TBIV to value 0"]
impl crate::Resettable for TBIV_SPEC {
    const RESET_VALUE: u16 = 0;
}
