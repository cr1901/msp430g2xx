#[doc = "Register `UCB0CTL1` reader"]
pub type R = crate::R<UCB0CTL1_SPEC>;
#[doc = "Register `UCB0CTL1` writer"]
pub type W = crate::W<UCB0CTL1_SPEC>;
#[doc = "Field `UCSWRST` reader - USCI Software Reset"]
pub type UCSWRST_R = crate::BitReader;
#[doc = "Field `UCSWRST` writer - USCI Software Reset"]
pub type UCSWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USCI 1 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: USCI 0 Clock Source: 0"]
    UCSSEL_0 = 0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    UCSSEL_1 = 1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    UCSSEL_2 = 2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    UCSSEL_3 = 3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCSSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for UCSSEL_A {}
#[doc = "Field `UCSSEL` reader - USCI 1 Clock Source Select 1"]
pub type UCSSEL_R = crate::FieldReader<UCSSEL_A>;
impl UCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCSSEL_0,
            1 => UCSSEL_A::UCSSEL_1,
            2 => UCSSEL_A::UCSSEL_2,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_0
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_1
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_2
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Field `UCSSEL` writer - USCI 1 Clock Source Select 1"]
pub type UCSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCSSEL_A, crate::Safe>;
impl<'a, REG> UCSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCSSEL_A::UCSSEL_0)
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCSSEL_A::UCSSEL_1)
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCSSEL_A::UCSSEL_2)
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W<UCB0CTL1_SPEC> {
        UCSWRST_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W<UCB0CTL1_SPEC> {
        UCSSEL_W::new(self, 6)
    }
}
#[doc = "USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0CTL1_SPEC;
impl crate::RegisterSpec for UCB0CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ctl1::R`](R) reader structure"]
impl crate::Readable for UCB0CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctl1::W`](W) writer structure"]
impl crate::Writable for UCB0CTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCB0CTL1 to value 0"]
impl crate::Resettable for UCB0CTL1_SPEC {
    const RESET_VALUE: u8 = 0;
}
