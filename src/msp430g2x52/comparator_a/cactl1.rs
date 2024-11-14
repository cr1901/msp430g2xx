#[doc = "Register `CACTL1` reader"]
pub type R = crate::R<CACTL1_SPEC>;
#[doc = "Register `CACTL1` writer"]
pub type W = crate::W<CACTL1_SPEC>;
#[doc = "Field `CAIFG` reader - Comp. A Interrupt Flag"]
pub type CAIFG_R = crate::BitReader;
#[doc = "Field `CAIFG` writer - Comp. A Interrupt Flag"]
pub type CAIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIE` reader - Comp. A Interrupt Enable"]
pub type CAIE_R = crate::BitReader;
#[doc = "Field `CAIE` writer - Comp. A Interrupt Enable"]
pub type CAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAIES` reader - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CAIES_R = crate::BitReader;
#[doc = "Field `CAIES` writer - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CAIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAON` reader - Comp. A enable"]
pub type CAON_R = crate::BitReader;
#[doc = "Field `CAON` writer - Comp. A enable"]
pub type CAON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comp. A Internal Reference Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAREF_A {
    #[doc = "0: Comp. A Int. Ref. Select 0 : Off"]
    CAREF_0 = 0,
    #[doc = "1: Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    CAREF_1 = 1,
    #[doc = "2: Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    CAREF_2 = 2,
    #[doc = "3: Comp. A Int. Ref. Select 3 : Vt"]
    CAREF_3 = 3,
}
impl From<CAREF_A> for u8 {
    #[inline(always)]
    fn from(variant: CAREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAREF_A {
    type Ux = u8;
}
impl crate::IsEnum for CAREF_A {}
#[doc = "Field `CAREF` reader - Comp. A Internal Reference Select 0"]
pub type CAREF_R = crate::FieldReader<CAREF_A>;
impl CAREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAREF_A {
        match self.bits {
            0 => CAREF_A::CAREF_0,
            1 => CAREF_A::CAREF_1,
            2 => CAREF_A::CAREF_2,
            3 => CAREF_A::CAREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn is_caref_0(&self) -> bool {
        *self == CAREF_A::CAREF_0
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn is_caref_1(&self) -> bool {
        *self == CAREF_A::CAREF_1
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn is_caref_2(&self) -> bool {
        *self == CAREF_A::CAREF_2
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn is_caref_3(&self) -> bool {
        *self == CAREF_A::CAREF_3
    }
}
#[doc = "Field `CAREF` writer - Comp. A Internal Reference Select 0"]
pub type CAREF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAREF_A, crate::Safe>;
impl<'a, REG> CAREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn caref_0(self) -> &'a mut crate::W<REG> {
        self.variant(CAREF_A::CAREF_0)
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn caref_1(self) -> &'a mut crate::W<REG> {
        self.variant(CAREF_A::CAREF_1)
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn caref_2(self) -> &'a mut crate::W<REG> {
        self.variant(CAREF_A::CAREF_2)
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn caref_3(self) -> &'a mut crate::W<REG> {
        self.variant(CAREF_A::CAREF_3)
    }
}
#[doc = "Field `CARSEL` reader - Comp. A Internal Reference Enable"]
pub type CARSEL_R = crate::BitReader;
#[doc = "Field `CARSEL` writer - Comp. A Internal Reference Enable"]
pub type CARSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEX` reader - Comp. A Exchange Inputs"]
pub type CAEX_R = crate::BitReader;
#[doc = "Field `CAEX` writer - Comp. A Exchange Inputs"]
pub type CAEX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&self) -> CAIFG_R {
        CAIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&self) -> CAIES_R {
        CAIES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&self) -> CAON_R {
        CAON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&self) -> CAREF_R {
        CAREF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&self) -> CARSEL_R {
        CARSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&self) -> CAEX_R {
        CAEX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&mut self) -> CAIFG_W<CACTL1_SPEC> {
        CAIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W<CACTL1_SPEC> {
        CAIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&mut self) -> CAIES_W<CACTL1_SPEC> {
        CAIES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&mut self) -> CAON_W<CACTL1_SPEC> {
        CAON_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&mut self) -> CAREF_W<CACTL1_SPEC> {
        CAREF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&mut self) -> CARSEL_W<CACTL1_SPEC> {
        CARSEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&mut self) -> CAEX_W<CACTL1_SPEC> {
        CAEX_W::new(self, 7)
    }
}
#[doc = "Comparator A Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACTL1_SPEC;
impl crate::RegisterSpec for CACTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cactl1::R`](R) reader structure"]
impl crate::Readable for CACTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cactl1::W`](W) writer structure"]
impl crate::Writable for CACTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CACTL1 to value 0"]
impl crate::Resettable for CACTL1_SPEC {
    const RESET_VALUE: u8 = 0;
}
