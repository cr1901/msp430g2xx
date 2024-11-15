#[doc = "Register `UCB0CTL0` reader"]
pub type R = crate::R<UCB0CTL0_SPEC>;
#[doc = "Register `UCB0CTL0` writer"]
pub type W = crate::W<UCB0CTL0_SPEC>;
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_R = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sync. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for UCMODE_A {}
#[doc = "Field `UCMODE` reader - Sync. Mode: USCI Mode 1"]
pub type UCMODE_R = crate::FieldReader<UCMODE_A>;
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Field `UCMODE` writer - Sync. Mode: USCI Mode 1"]
pub type UCMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCMODE_A, crate::Safe>;
impl<'a, REG> UCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCMODE_A::UCMODE_3)
    }
}
#[doc = "Field `UCMST` reader - Sync. Mode: Master Select"]
pub type UCMST_R = crate::BitReader;
#[doc = "Field `UCMST` writer - Sync. Mode: Master Select"]
pub type UCMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCMM` reader - Multi-Master Environment"]
pub type UCMM_R = crate::BitReader;
#[doc = "Field `UCMM` writer - Multi-Master Environment"]
pub type UCMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSLA10` reader - 10-bit Slave Address Mode"]
pub type UCSLA10_R = crate::BitReader;
#[doc = "Field `UCSLA10` writer - 10-bit Slave Address Mode"]
pub type UCSLA10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA10` reader - 10-bit Address Mode"]
pub type UCA10_R = crate::BitReader;
#[doc = "Field `UCA10` writer - 10-bit Address Mode"]
pub type UCA10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&self) -> UCMM_R {
        UCMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&self) -> UCSLA10_R {
        UCSLA10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&self) -> UCA10_R {
        UCA10_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W<UCB0CTL0_SPEC> {
        UCSYNC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W<UCB0CTL0_SPEC> {
        UCMODE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W<UCB0CTL0_SPEC> {
        UCMST_W::new(self, 3)
    }
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UCMM_W<UCB0CTL0_SPEC> {
        UCMM_W::new(self, 5)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> UCSLA10_W<UCB0CTL0_SPEC> {
        UCSLA10_W::new(self, 6)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&mut self) -> UCA10_W<UCB0CTL0_SPEC> {
        UCA10_W::new(self, 7)
    }
}
#[doc = "USCI B0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0CTL0_SPEC;
impl crate::RegisterSpec for UCB0CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ctl0::R`](R) reader structure"]
impl crate::Readable for UCB0CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctl0::W`](W) writer structure"]
impl crate::Writable for UCB0CTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0CTL0 to value 0"]
impl crate::Resettable for UCB0CTL0_SPEC {
    const RESET_VALUE: u8 = 0;
}
