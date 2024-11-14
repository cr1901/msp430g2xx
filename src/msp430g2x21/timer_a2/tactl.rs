#[doc = "Register `TACTL` reader"]
pub type R = crate::R<TACTL_SPEC>;
#[doc = "Register `TACTL` writer"]
pub type W = crate::W<TACTL_SPEC>;
#[doc = "Field `TAIFG` reader - Timer A counter interrupt flag"]
pub type TAIFG_R = crate::BitReader;
#[doc = "Field `TAIFG` writer - Timer A counter interrupt flag"]
pub type TAIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIE` reader - Timer A counter interrupt enable"]
pub type TAIE_R = crate::BitReader;
#[doc = "Field `TAIE` writer - Timer A counter interrupt enable"]
pub type TAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACLR` reader - Timer A counter clear"]
pub type TACLR_R = crate::BitReader;
#[doc = "Field `TACLR` writer - Timer A counter clear"]
pub type TACLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer A mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Timer A mode control: 0 - Stop"]
    MC_0 = 0,
    #[doc = "1: Timer A mode control: 1 - Up to CCR0"]
    MC_1 = 1,
    #[doc = "2: Timer A mode control: 2 - Continous up"]
    MC_2 = 2,
    #[doc = "3: Timer A mode control: 3 - Up/Down"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MC_A {
    type Ux = u8;
}
impl crate::IsEnum for MC_A {}
#[doc = "Field `MC` reader - Timer A mode control 1"]
pub type MC_R = crate::FieldReader<MC_A>;
impl MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == MC_A::MC_0
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == MC_A::MC_1
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == MC_A::MC_2
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == MC_A::MC_3
    }
}
#[doc = "Field `MC` writer - Timer A mode control 1"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MC_A, crate::Safe>;
impl<'a, REG> MC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut crate::W<REG> {
        self.variant(MC_A::MC_3)
    }
}
#[doc = "Timer A clock input divider 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: Timer A input divider: 0 - /1"]
    ID_0 = 0,
    #[doc = "1: Timer A input divider: 1 - /2"]
    ID_1 = 1,
    #[doc = "2: Timer A input divider: 2 - /4"]
    ID_2 = 2,
    #[doc = "3: Timer A input divider: 3 - /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ID_A {
    type Ux = u8;
}
impl crate::IsEnum for ID_A {}
#[doc = "Field `ID` reader - Timer A clock input divider 1"]
pub type ID_R = crate::FieldReader<ID_A>;
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == ID_A::ID_0
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == ID_A::ID_1
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == ID_A::ID_2
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == ID_A::ID_3
    }
}
#[doc = "Field `ID` writer - Timer A clock input divider 1"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ID_A, crate::Safe>;
impl<'a, REG> ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut crate::W<REG> {
        self.variant(ID_A::ID_0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut crate::W<REG> {
        self.variant(ID_A::ID_1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut crate::W<REG> {
        self.variant(ID_A::ID_2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut crate::W<REG> {
        self.variant(ID_A::ID_3)
    }
}
#[doc = "Timer A clock source select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: Timer A clock source select: 0 - TACLK"]
    TASSEL_0 = 0,
    #[doc = "1: Timer A clock source select: 1 - ACLK"]
    TASSEL_1 = 1,
    #[doc = "2: Timer A clock source select: 2 - SMCLK"]
    TASSEL_2 = 2,
    #[doc = "3: Timer A clock source select: 3 - INCLK"]
    TASSEL_3 = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TASSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TASSEL_A {}
#[doc = "Field `TASSEL` reader - Timer A clock source select 1"]
pub type TASSEL_R = crate::FieldReader<TASSEL_A>;
impl TASSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TASSEL_0,
            1 => TASSEL_A::TASSEL_1,
            2 => TASSEL_A::TASSEL_2,
            3 => TASSEL_A::TASSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        *self == TASSEL_A::TASSEL_0
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        *self == TASSEL_A::TASSEL_1
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        *self == TASSEL_A::TASSEL_2
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        *self == TASSEL_A::TASSEL_3
    }
}
#[doc = "Field `TASSEL` writer - Timer A clock source select 1"]
pub type TASSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TASSEL_A, crate::Safe>;
impl<'a, REG> TASSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut crate::W<REG> {
        self.variant(TASSEL_A::TASSEL_0)
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut crate::W<REG> {
        self.variant(TASSEL_A::TASSEL_1)
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut crate::W<REG> {
        self.variant(TASSEL_A::TASSEL_2)
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut crate::W<REG> {
        self.variant(TASSEL_A::TASSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TAIFG_W<TACTL_SPEC> {
        TAIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W<TACTL_SPEC> {
        TAIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TACLR_W<TACTL_SPEC> {
        TACLR_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W<TACTL_SPEC> {
        MC_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<TACTL_SPEC> {
        ID_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TASSEL_W<TACTL_SPEC> {
        TASSEL_W::new(self, 8)
    }
}
#[doc = "Timer A Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TACTL_SPEC;
impl crate::RegisterSpec for TACTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tactl::R`](R) reader structure"]
impl crate::Readable for TACTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tactl::W`](W) writer structure"]
impl crate::Writable for TACTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TACTL to value 0"]
impl crate::Resettable for TACTL_SPEC {
    const RESET_VALUE: u16 = 0;
}
