#[doc = "Register `FCTL2` reader"]
pub type R = crate::R<Fctl2Spec>;
#[doc = "Register `FCTL2` writer"]
pub type W = crate::W<Fctl2Spec>;
#[doc = "Field `FN` reader - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FnR = crate::FieldReader;
#[doc = "Field `FN` writer - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
pub type FnW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
#[doc = "Flash clock select 0 */ /* to distinguish from USART SSELx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fssel {
    #[doc = "0: Flash clock select: 0 - ACLK"]
    Fssel0 = 0,
    #[doc = "1: Flash clock select: 1 - MCLK"]
    Fssel1 = 1,
    #[doc = "2: Flash clock select: 2 - SMCLK"]
    Fssel2 = 2,
    #[doc = "3: Flash clock select: 3 - SMCLK"]
    Fssel3 = 3,
}
impl From<Fssel> for u8 {
    #[inline(always)]
    fn from(variant: Fssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fssel {
    type Ux = u8;
}
impl crate::IsEnum for Fssel {}
#[doc = "Field `FSSEL` reader - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FsselR = crate::FieldReader<Fssel>;
impl FsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fssel {
        match self.bits {
            0 => Fssel::Fssel0,
            1 => Fssel::Fssel1,
            2 => Fssel::Fssel2,
            3 => Fssel::Fssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn is_fssel_0(&self) -> bool {
        *self == Fssel::Fssel0
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn is_fssel_1(&self) -> bool {
        *self == Fssel::Fssel1
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_2(&self) -> bool {
        *self == Fssel::Fssel2
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn is_fssel_3(&self) -> bool {
        *self == Fssel::Fssel3
    }
}
#[doc = "Field `FSSEL` writer - Flash clock select 0 */ /* to distinguish from USART SSELx"]
pub type FsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fssel, crate::Safe>;
impl<'a, REG> FsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash clock select: 0 - ACLK"]
    #[inline(always)]
    pub fn fssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel0)
    }
    #[doc = "Flash clock select: 1 - MCLK"]
    #[inline(always)]
    pub fn fssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel1)
    }
    #[doc = "Flash clock select: 2 - SMCLK"]
    #[inline(always)]
    pub fn fssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel2)
    }
    #[doc = "Flash clock select: 3 - SMCLK"]
    #[inline(always)]
    pub fn fssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fssel::Fssel3)
    }
}
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fwkeyr {
    #[doc = "150: Value always read from the FCTL2 Password register"]
    Password = 150,
}
impl From<Fwkeyr> for u8 {
    #[inline(always)]
    fn from(variant: Fwkeyr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fwkeyr {
    type Ux = u8;
}
impl crate::IsEnum for Fwkeyr {}
#[doc = "Field `FWKEY` reader - FCTL2 Password"]
pub type FwkeyR = crate::FieldReader<Fwkeyr>;
impl FwkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fwkeyr> {
        match self.bits {
            150 => Some(Fwkeyr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the FCTL2 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Fwkeyr::Password
    }
}
#[doc = "FCTL2 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FwkeywWO {
    #[doc = "165: Value which must be written to the FCTL2 Password register"]
    Password = 165,
}
impl From<FwkeywWO> for u8 {
    #[inline(always)]
    fn from(variant: FwkeywWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FwkeywWO {
    type Ux = u8;
}
impl crate::IsEnum for FwkeywWO {}
#[doc = "Field `FWKEY` writer - FCTL2 Password"]
pub type FwkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, FwkeywWO>;
impl<'a, REG> FwkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL2 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FwkeywWO::Password)
    }
}
impl R {
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&self) -> FsselR {
        FsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FwkeyR {
        FwkeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
    #[inline(always)]
    pub fn fn_(&mut self) -> FnW<Fctl2Spec> {
        FnW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FsselW<Fctl2Spec> {
        FsselW::new(self, 6)
    }
    #[doc = "Bits 8:15 - FCTL2 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FwkeyW<Fctl2Spec> {
        FwkeyW::new(self, 8)
    }
}
#[doc = "FLASH Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl2Spec;
impl crate::RegisterSpec for Fctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl2::R`](R) reader structure"]
impl crate::Readable for Fctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl2::W`](W) writer structure"]
impl crate::Writable for Fctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FCTL2 to value 0"]
impl crate::Resettable for Fctl2Spec {
    const RESET_VALUE: u16 = 0;
}
