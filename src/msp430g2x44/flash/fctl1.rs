#[doc = "Register `FCTL1` reader"]
pub type R = crate::R<FCTL1_SPEC>;
#[doc = "Register `FCTL1` writer"]
pub type W = crate::W<FCTL1_SPEC>;
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MERAS_R = crate::BitReader;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MERAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEI` reader - Enable Erase Interrupts"]
pub type EEI_R = crate::BitReader;
#[doc = "Field `EEI` writer - Enable Erase Interrupts"]
pub type EEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEIEX` reader - Enable Emergency Interrupt Exit"]
pub type EEIEX_R = crate::BitReader;
#[doc = "Field `EEIEX` writer - Enable Emergency Interrupt Exit"]
pub type EEIEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WRT_R = crate::BitReader;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BLKWRT_R = crate::BitReader;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BLKWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYR_A {
    #[doc = "150: Value always read from the FCTL1 Password register"]
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
#[doc = "Field `FWKEY` reader - FCTL1 Password"]
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
    #[doc = "Value always read from the FCTL1 Password register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEYR_A::PASSWORD
    }
}
#[doc = "FCTL1 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYW_AW {
    #[doc = "165: Value which must be written to the FCTL1 Password register"]
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
#[doc = "Field `FWKEY` writer - FCTL1 Password"]
pub type FWKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, FWKEYW_AW>;
impl<'a, REG> FWKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FCTL1 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FWKEYW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Erase Interrupts"]
    #[inline(always)]
    pub fn eei(&self) -> EEI_R {
        EEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Emergency Interrupt Exit"]
    #[inline(always)]
    pub fn eeiex(&self) -> EEIEX_R {
        EEIEX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W<FCTL1_SPEC> {
        ERASE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&mut self) -> MERAS_W<FCTL1_SPEC> {
        MERAS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Erase Interrupts"]
    #[inline(always)]
    pub fn eei(&mut self) -> EEI_W<FCTL1_SPEC> {
        EEI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Emergency Interrupt Exit"]
    #[inline(always)]
    pub fn eeiex(&mut self) -> EEIEX_W<FCTL1_SPEC> {
        EEIEX_W::new(self, 4)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&mut self) -> WRT_W<FCTL1_SPEC> {
        WRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&mut self) -> BLKWRT_W<FCTL1_SPEC> {
        BLKWRT_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - FCTL1 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FWKEY_W<FCTL1_SPEC> {
        FWKEY_W::new(self, 8)
    }
}
#[doc = "FLASH Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL1_SPEC;
impl crate::RegisterSpec for FCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl1::R`](R) reader structure"]
impl crate::Readable for FCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl1::W`](W) writer structure"]
impl crate::Writable for FCTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for FCTL1_SPEC {
    const RESET_VALUE: u16 = 0;
}
