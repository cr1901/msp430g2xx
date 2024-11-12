#[doc = "Register `FCTL1` reader"]
pub type R = crate::R<Fctl1Spec>;
#[doc = "Register `FCTL1` writer"]
pub type W = crate::W<Fctl1Spec>;
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MerasR = crate::BitReader;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MerasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WrtR = crate::BitReader;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BlkwrtR = crate::BitReader;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BlkwrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fwkeyr {
    #[doc = "150: Value always read from the FCTL1 Password register"]
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
#[doc = "Field `FWKEY` reader - FCTL1 Password"]
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
    #[doc = "Value always read from the FCTL1 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Fwkeyr::Password
    }
}
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FwkeywWO {
    #[doc = "165: Value which must be written to the FCTL1 Password register"]
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
#[doc = "Field `FWKEY` writer - FCTL1 Password"]
pub type FwkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, FwkeywWO>;
impl<'a, REG> FwkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL1 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FwkeywWO::Password)
    }
}
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MerasR {
        MerasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WrtR {
        WrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BlkwrtR {
        BlkwrtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FwkeyR {
        FwkeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<Fctl1Spec> {
        EraseW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&mut self) -> MerasW<Fctl1Spec> {
        MerasW::new(self, 2)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&mut self) -> WrtW<Fctl1Spec> {
        WrtW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&mut self) -> BlkwrtW<Fctl1Spec> {
        BlkwrtW::new(self, 7)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FwkeyW<Fctl1Spec> {
        FwkeyW::new(self, 8)
    }
}
#[doc = "FLASH Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl1Spec;
impl crate::RegisterSpec for Fctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl1::R`](R) reader structure"]
impl crate::Readable for Fctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl1::W`](W) writer structure"]
impl crate::Writable for Fctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for Fctl1Spec {
    const RESET_VALUE: u16 = 0;
}
