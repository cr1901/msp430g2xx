#[doc = "Register `FCTL2` reader"]
pub type R = crate::R<FCTL2_SPEC>;
#[doc = "Register `FCTL2` writer"]
pub type W = crate::W<FCTL2_SPEC>;
#[doc = "Field `FN` reader - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FN_R = crate::FieldReader;
#[doc = "Field `FN` writer - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FN_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSSEL_A {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    FSSEL_0 = 0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    FSSEL_1 = 1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    FSSEL_2 = 2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    FSSEL_3 = 3,
}
impl From<FSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for FSSEL_A {}
#[doc = "Field `FSSEL` reader - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FSSEL_R = crate::FieldReader<FSSEL_A>;
impl FSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSSEL_A {
        match self.bits {
            0 => FSSEL_A::FSSEL_0,
            1 => FSSEL_A::FSSEL_1,
            2 => FSSEL_A::FSSEL_2,
            3 => FSSEL_A::FSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn is_fssel_0(&self) -> bool {
        *self == FSSEL_A::FSSEL_0
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn is_fssel_1(&self) -> bool {
        *self == FSSEL_A::FSSEL_1
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_2(&self) -> bool {
        *self == FSSEL_A::FSSEL_2
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_3(&self) -> bool {
        *self == FSSEL_A::FSSEL_3
    }
}
#[doc = "Field `FSSEL` writer - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FSSEL_A, crate::Safe>;
impl<'a, REG> FSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn fssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(FSSEL_A::FSSEL_0)
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn fssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(FSSEL_A::FSSEL_1)
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn fssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(FSSEL_A::FSSEL_2)
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn fssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(FSSEL_A::FSSEL_3)
    }
}
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYR_A {
    #[doc = "150: Value always read from the FCTL2 Password register"]
    PASSWORD = 150,
}
impl From<FWKEYR_A> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FWKEYR_A {
    type Ux = u8;
}
impl crate::IsEnum for FWKEYR_A {}
#[doc = "Field `FWKEY` reader - FCTL2 Password"]
pub type FWKEY_R = crate::FieldReader<FWKEYR_A>;
impl FWKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FWKEYR_A> {
        match self.bits {
            150 => Some(FWKEYR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Value always read from the FCTL2 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEYR_A::PASSWORD
    }
}
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYW_AW {
    #[doc = "165: Value which must be written to the FCTL2 Password register"]
    PASSWORD = 165,
}
impl From<FWKEYW_AW> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYW_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FWKEYW_AW {
    type Ux = u8;
}
impl crate::IsEnum for FWKEYW_AW {}
#[doc = "Field `FWKEY` writer - FCTL2 Password"]
pub type FWKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, FWKEYW_AW>;
impl<'a, REG> FWKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL2 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FWKEYW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&mut self) -> FN_W<FCTL2_SPEC> {
        FN_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FSSEL_W<FCTL2_SPEC> {
        FSSEL_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FWKEY_W<FCTL2_SPEC> {
        FWKEY_W::new(self, 8)
    }
}
#[doc = "FLASH Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL2_SPEC;
impl crate::RegisterSpec for FCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl2::R`](R) reader structure"]
impl crate::Readable for FCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl2::W`](W) writer structure"]
impl crate::Writable for FCTL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FCTL2 to value 0"]
impl crate::Resettable for FCTL2_SPEC {
    const RESET_VALUE: u16 = 0;
}
